pub mod auth;
pub mod header;

use byteorder::{BigEndian, ByteOrder};
use xdr_codec::{Pack, Unpack, Read, Write};
#[cfg(target_os = "wasi")]
use crate::wasi_ext::TcpStream;
#[cfg(not(target_os = "wasi"))]
use std::net::TcpStream;
use crate::{Result, Error, ErrorKind};

use auth::Auth;
pub(crate) use header::Header;

pub(crate) const RPC_VERSION: u32 = 2;
pub(crate) const PORTMAP_PROG: u32 = 100000;
pub(crate) const PORTMAP_VERSION: u32 = 2;
pub(crate) const PORTMAP_PORT: u16 = 111;
pub(crate) const MOUNT3_PROG: u32 = 100005;
pub(crate) const MOUNT3_VERSION: u32 = 3;
pub(crate) const NFS3_PROG: u32 = 100003;
pub(crate) const NFS3_VERSION: u32 = 3;

const IPPROTO_TCP: u32 = 6;
// const IPPROTO_UDP: u32 = 17;

enum PortmapProc2 {
    Null = 0,
    // Set = 1,
    // Unset = 2,
    GetPort = 3,
    // Dump = 4,
    // CallIt = 5,
}

pub(crate) fn portmap(host: &String, prog: u32, vers: u32, auth: &Auth) -> Result<u16> {
    if let Some(conn) = TcpStream::connect((host.as_str(), PORTMAP_PORT)).ok() {
        let client = Client::new(Some(conn), None);
        let args = Header::new(RPC_VERSION, PORTMAP_PROG, PORTMAP_VERSION, PortmapProc2::Null as u32, &auth, &Auth::new_null());
        let mut buf = Vec::<u8>::new();
        let res = args.pack(&mut buf);
        if res.is_err() {
            let _ = client.shutdown();
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }
        if client.call(buf).is_ok() {
            let args = GETPORT2args{
                header: Header::new(RPC_VERSION, PORTMAP_PROG, PORTMAP_VERSION, PortmapProc2::GetPort as u32, &auth, &Auth::new_null()),
                prog,
                vers,
                proto: IPPROTO_TCP,
                port: 0,
            };
            let mut buf = Vec::<u8>::new();
            let res = args.pack(&mut buf);
            if res.is_err() {
                let _ = client.shutdown();
                return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
            }
            if let Some(res) = client.call(buf).ok() {
                let port = BigEndian::read_u32(res.as_slice()) as u16;
                let _ = client.shutdown();
                return Ok(port);
            }
        }
        let _ = client.shutdown();
    }
    Err(Error::new(ErrorKind::Other, "error obtaining ports from portmapper"))
}

#[derive(Debug, PartialEq)]
struct GETPORT2args {
    header: Header,
    prog: u32,
    vers: u32,
    proto: u32,
    port: u32,
}

impl<Out: Write> Pack<Out> for GETPORT2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.header.pack(out)? + self.prog.pack(out)? + self.vers.pack(out)? + self.proto.pack(out)? + self.port.pack(out)?)
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    nfs_conn: Option<TcpStream>,
    mount_conn: Option<TcpStream>,
}

impl Client {
    pub(crate) fn new(nfs_conn: Option<TcpStream>, mount_conn: Option<TcpStream>) -> Self {
        Self{nfs_conn, mount_conn}
    }

    fn get_conn(&self, reqmsg: &Message) -> &TcpStream {
        match reqmsg.program() {
            MOUNT3_PROG => self.mount_conn.as_ref().unwrap(),
            NFS3_PROG | PORTMAP_PROG => self.nfs_conn.as_ref().unwrap(),
            _ => panic!("unknown RPC program - RPC header values: rpc_version={} program={} version={} procedure={}", reqmsg.rpc_version(), reqmsg.program(), reqmsg.version(), reqmsg.procedure()),
        }
    }

    pub(crate) fn call(&self, msg_body: Vec<u8>) -> Result<Vec<u8>> {
        let reqmsg = Message::new(msg_body);
        let mut buf = Vec::<u8>::new();
        let b = reqmsg.pack(&mut buf);
        if b.is_err() {
            return Err(Error::new(ErrorKind::Other, b.unwrap_err()));
        }

        let mut hdr_buf = [0u8; 4];
        BigEndian::write_u32(&mut hdr_buf, b.unwrap() as u32 | 0x80000000);
        let mut hdr_buf_vec = hdr_buf.to_vec();
        hdr_buf_vec.append(&mut buf);

        #[allow(unused_mut)]
        let mut conn = self.get_conn(&reqmsg);
        let _ = conn.write_all(hdr_buf_vec.as_slice())?;

        let mut hdr = [0u8; 4];
        let _ = conn.read_exact(&mut hdr)?;
        let sz = BigEndian::read_u32(&hdr) & 0x7fffffff;

        let mut res = vec![0u8; sz as usize];
        let _ = conn.read_exact(&mut res)?;

        let mut r = res.as_slice();
        let z: xdr_codec::Result<Message> = xdr_codec::unpack(&mut r);
        if z.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse response message"));
        }

        let resmsg = z.unwrap();
        if resmsg.xid != reqmsg.xid {
            return Err(Error::new(ErrorKind::Other, "response id does not match expected one"));
        }
        if resmsg.msgtype != MessageType::Response {
            return Err(Error::new(ErrorKind::Other, "response type does not match expected one"));
        }

        let mut zbuf = resmsg.body.as_slice();
        let zres: xdr_codec::Result<u32> = xdr_codec::unpack(&mut zbuf);
        if zres.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse response message status"));
        }

        let msgstatus = zres.unwrap();
        match msgstatus { // FIXME: add message status enum and cover each known value
            0 => {
                let padding: xdr_codec::Result<u32> = xdr_codec::unpack(&mut zbuf);
                if padding.is_err() {
                    return Err(Error::new(ErrorKind::Other, "could not parse response message padding"));
                }
                let opaquelen: xdr_codec::Result<u32> = xdr_codec::unpack(&mut zbuf);
                if opaquelen.is_err() {
                    return Err(Error::new(ErrorKind::Other, "could not parse response message opaque length"));
                }
                let opaquelen = opaquelen.unwrap();
                if opaquelen > 0 {
                    // "seek" opaquelen bytes from current position
                    let seek = xdr_codec::unpack_opaque_flex(&mut zbuf, Some(opaquelen as usize));
                    if seek.is_err() {
                        return Err(Error::new(ErrorKind::Other, "could not parse response"));
                    }
                }
                let acceptstatus: xdr_codec::Result<u32> = xdr_codec::unpack(&mut zbuf);
                if acceptstatus.is_err() {
                    return Err(Error::new(ErrorKind::Other, "could not parse response message accept status"));
                }
                match acceptstatus.unwrap() { // FIXME: add accept status enum and cover each known value
                    0 => Ok(zbuf.to_vec()),
                    _ => Err(Error::new(ErrorKind::Other, "request rejected")),
                }
            },
            _ => Err(Error::new(ErrorKind::Other, "could not parse response due to bad status")),
        }
    }

    fn shutdown(&self) -> Result<()> {
        if let Some(nfs_conn) = &self.nfs_conn {
            nfs_conn.shutdown(std::net::Shutdown::Both)?;
        }
        if let Some(mount_conn) = &self.mount_conn {
            mount_conn.shutdown(std::net::Shutdown::Both)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MessageType {
    Request = 0,
    Response = 1,
}

impl<In: Read> Unpack<In> for MessageType {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MessageType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MessageType::Request as i32 => MessageType::Request,
                    x if x == MessageType::Response as i32 => MessageType::Response,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

static mut XID: u32 = 0;

fn get_xid() -> u32 {
    unsafe {
        if XID == 0 {
            XID = get_current_time();
        }
        XID += 1;
        XID
    }
}

struct Message {
    xid: u32,
    msgtype: MessageType,
    body: Vec<u8>,
}

impl Message {
    fn new(msg_body: Vec<u8>) -> Self {
        Self{
            xid: get_xid(),
            msgtype: MessageType::Request,
            body: msg_body,
        }
    }

    fn rpc_version(&self) -> u32 {
        BigEndian::read_u32(&self.body[0..4])
    }

    fn program(&self) -> u32 {
        BigEndian::read_u32(&self.body[4..8])
    }

    fn version(&self) -> u32 {
        BigEndian::read_u32(&self.body[8..12])
    }

    fn procedure(&self) -> u32 {
        BigEndian::read_u32(&self.body[12..16])
    }
}

impl<Out: Write> Pack<Out> for Message {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.xid.pack(out)? + (self.msgtype.clone() as u32).pack(out)? + xdr_codec::pack_opaque_array(self.body.as_slice(), self.body.len(), out)?)
    }
}

impl<In: Read> Unpack<In> for Message {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Message, usize)> {
        let mut sz = 0;
        Ok((
            Message{
                xid: {
                    let (v, fsz) = Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                msgtype: {
                    let (v, fsz) = Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                body: {
                    let mut v = Vec::new();
                    let usz = input.read_to_end(&mut v)?;
                    sz += usz;
                    v
                },
            },
            sz,
        ))
    }
}

fn get_current_time() -> u32 {
    let now = std::time::SystemTime::now();
    let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    (since_epoch.as_secs() as u32).wrapping_mul(1000) + since_epoch.subsec_nanos() / 1_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_new() {
        let msg = Message::new(vec![]);
        assert_ne!(msg.xid, 0);
        let xid = msg.xid;
        let msg = Message::new(vec![]);
        assert_eq!(msg.xid, xid+1);
    }

    #[test]
    fn message_rpc_version() {
        let msg = Message::new(vec![0,0,0,2,0,0,0,3,0,0,0,4,0,0,0,5]);
        assert_eq!(msg.rpc_version(), 2);
    }

    #[test]
    fn message_program() {
        let msg = Message::new(vec![0,0,0,2,0,0,0,3,0,0,0,4,0,0,0,5]);
        assert_eq!(msg.program(), 3);
    }

    #[test]
    fn message_version() {
        let msg = Message::new(vec![0,0,0,2,0,0,0,3,0,0,0,4,0,0,0,5]);
        assert_eq!(msg.version(), 4);
    }

    #[test]
    fn message_procedure() {
        let msg = Message::new(vec![0,0,0,2,0,0,0,3,0,0,0,4,0,0,0,5]);
        assert_eq!(msg.procedure(), 5);
    }
}
