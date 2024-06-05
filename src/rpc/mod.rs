pub mod auth;
pub mod header;

use crate::{
    connect_tcp_stream, reconnect_tcp_stream, remove_tcp_stream, using_tcp_stream,
    using_tcp_stream_with_buffer, Error, ErrorKind, Result, SocketAddr,
};
use byteorder::{BigEndian, ByteOrder};
use xdr_codec::{Pack, Read, Unpack, Write};

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

pub(crate) fn portmap(addrs: &Vec<SocketAddr>, prog: u32, vers: u32, auth: &Auth) -> Result<u16> {
    for addr in addrs {
        let res = portmap_on_addr(&addr, prog, vers, auth);
        if res.is_ok() {
            return Ok(res.unwrap());
        }
    }
    Err(Error::new(
        ErrorKind::Other,
        "error obtaining ports from portmapper",
    ))
}

fn portmap_on_addr(addr: &SocketAddr, prog: u32, vers: u32, auth: &Auth) -> Result<u16> {
    let res = connect_tcp_stream(addr);
    if res.is_err() {
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }
    let stream_id = res.unwrap();
    let client = Client::new(stream_id, None);
    let args = Header::new(
        RPC_VERSION,
        PORTMAP_PROG,
        PORTMAP_VERSION,
        PortmapProc2::Null as u32,
        &auth,
        &Auth::new_null(),
    );
    let mut buf = Vec::<u8>::new();
    let res = args.pack(&mut buf);
    if res.is_err() {
        let _ = client.shutdown();
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }
    let res = client.call(buf);
    if res.is_err() {
        let _ = client.shutdown();
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }
    let args = GETPORT2args {
        header: Header::new(
            RPC_VERSION,
            PORTMAP_PROG,
            PORTMAP_VERSION,
            PortmapProc2::GetPort as u32,
            &auth,
            &Auth::new_null(),
        ),
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
    let res = client.call(buf);
    if res.is_err() {
        let _ = client.shutdown();
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }
    let res = res.unwrap();
    let port = BigEndian::read_u32(res.as_slice()) as u16;
    let _ = client.shutdown();
    Ok(port)
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
        Ok(self.header.pack(out)?
            + self.prog.pack(out)?
            + self.vers.pack(out)?
            + self.proto.pack(out)?
            + self.port.pack(out)?)
    }
}

fn parse_xdr_response<T>(res: xdr_codec::Result<T>, response_part_being_parsed: &str) -> Result<T> {
    if res.is_err() {
        Err(Error::new(
            ErrorKind::Other,
            format!("could not parse response {}", response_part_being_parsed).as_str(),
        ))
    } else {
        Ok(res.unwrap())
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    nfs_stream_id: u32,
    mount_stream_id: Option<u32>,
}

impl Client {
    pub(crate) fn new(nfs_stream_id: u32, mount_stream_id: Option<u32>) -> Self {
        Self {
            nfs_stream_id,
            mount_stream_id,
        }
    }

    fn get_stream_id(&self, reqmsg: &Message) -> u32 {
        match reqmsg.program() {
            MOUNT3_PROG => self.mount_stream_id.unwrap_or(self.nfs_stream_id),
            NFS3_PROG | PORTMAP_PROG => self.nfs_stream_id,
            _ => panic!("unknown RPC program - RPC header values: rpc_version={} program={} version={} procedure={}", reqmsg.rpc_version(), reqmsg.program(), reqmsg.version(), reqmsg.procedure()),
        }
    }

    pub(crate) fn call(&self, msg_body: Vec<u8>) -> Result<Vec<u8>> {
        let max_retries = 10;
        let mut num_retries = 0;
        while num_retries < max_retries {
            let res = self._call(msg_body.to_vec());
            if res.is_err() {
                let err = res.unwrap_err();
                match err.kind() {
                    ErrorKind::BrokenPipe
                    | ErrorKind::ConnectionAborted
                    | ErrorKind::ConnectionReset => {}
                    _ => return Err(err),
                }
                std::thread::sleep(std::time::Duration::from_millis(num_retries * 100));
                let res = reconnect_tcp_stream(self.nfs_stream_id);
                if res.is_err() {
                    let err = res.unwrap_err();
                    println!(
                        "error attempting reconnect to NFS server: {}",
                        err.to_string()
                    )
                }
                if self.mount_stream_id.is_some() {
                    let res = reconnect_tcp_stream(self.mount_stream_id.unwrap());
                    if res.is_err() {
                        let err = res.unwrap_err();
                        println!(
                            "error attempting reconnect to NFS server (mount protocol): {}",
                            err.to_string()
                        )
                    }
                }
                num_retries += 1;
                continue;
            }
            return res;
        }
        Err(Error::new(
            ErrorKind::NotConnected,
            "unable to reconnect to NFS server",
        ))
    }

    fn _call(&self, msg_body: Vec<u8>) -> Result<Vec<u8>> {
        const SIZE_HDR_BIT: u32 = 0x80000000;
        const SIZE_HDR_BITS: u32 = SIZE_HDR_BIT - 1;

        // construct request message, along with a dummy request message size, and pack it into a byte buffer
        let reqmsg = Message::new(msg_body);
        let mut buf = vec![0u8; 4]; // initializing with 4 zero bytes as a dummy request message size
        let b = reqmsg.pack(&mut buf);
        if b.is_err() {
            return Err(Error::new(ErrorKind::Other, b.unwrap_err()));
        }

        // overwrite previously written dummy request message size with real request message size
        BigEndian::write_u32(&mut buf[0..4], b.unwrap() as u32 | SIZE_HDR_BIT);

        let stream_id = self.get_stream_id(&reqmsg);
        let res = using_tcp_stream_with_buffer(
            &stream_id,
            &buf,
            #[allow(unused_mut)]
            |mut stream, buf| -> Result<Vec<u8>> {
                // send the byte buffer to RPC service
                let _ = stream.write_all(buf.as_slice())?;

                // read response message size from RPC service
                let mut hdr = [0u8; 4];
                let _ = stream.read_exact(&mut hdr)?;
                let sz = BigEndian::read_u32(&hdr) & SIZE_HDR_BITS;

                // read response message from RPC service
                let mut res = vec![0u8; sz as usize];
                let _ = stream.read_exact(&mut res)?;
                Ok(res)
            },
        )?;

        // unpack response message
        let mut r = res.as_slice();
        let resmsg = parse_xdr_response(xdr_codec::unpack::<_, Message>(&mut r), "message")?;

        // verify response message matches expected
        if resmsg.xid != reqmsg.xid {
            return Err(Error::new(
                ErrorKind::Other,
                "response id does not match expected one",
            ));
        }
        if resmsg.msgtype != MessageType::Response {
            return Err(Error::new(
                ErrorKind::Other,
                "response type does not match expected one",
            ));
        }

        // unpack message status
        let mut zbuf = resmsg.body.as_slice();
        let messagestatus = parse_xdr_response(
            xdr_codec::unpack::<_, MessageStatus>(&mut zbuf),
            "message status",
        )?;

        // check message status
        match messagestatus {
            MessageStatus::Accepted => {}
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "could not parse response due to bad status",
                ))
            }
        }

        // unpack padding
        let _padding =
            parse_xdr_response(xdr_codec::unpack::<_, u32>(&mut zbuf), "message padding")?;
        // TODO: should there be a "seek" for padding, equivalent to what is done for opaque length?

        // unpack opaque length
        let opaquelen = parse_xdr_response(
            xdr_codec::unpack::<_, u32>(&mut zbuf),
            "message opaque length",
        )?;
        if opaquelen > 0 {
            // "seek" opaquelen bytes from current position
            let seek = xdr_codec::unpack_opaque_flex(&mut zbuf, Some(opaquelen as usize));
            if seek.is_err() {
                return Err(Error::new(ErrorKind::Other, "could not parse response"));
            }
        }

        // unpack accept status
        let acceptstatus = parse_xdr_response(
            xdr_codec::unpack::<_, AcceptStatus>(&mut zbuf),
            "message accept status",
        )?;

        // check accept status
        match acceptstatus {
            AcceptStatus::Success => Ok(zbuf.to_vec()),
            _ => Err(Error::new(ErrorKind::Other, "request rejected")),
        }
    }

    fn shutdown(&self) -> Result<()> {
        using_tcp_stream(&self.nfs_stream_id, |stream| -> Result<()> {
            stream.shutdown(std::net::Shutdown::Both)
        })?;
        if self.mount_stream_id.is_some() {
            using_tcp_stream(
                self.mount_stream_id.as_ref().unwrap(),
                |stream| -> Result<()> { stream.shutdown(std::net::Shutdown::Both) },
            )?;
        }
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        remove_tcp_stream(&self.nfs_stream_id);
        if self.mount_stream_id.is_some() {
            remove_tcp_stream(self.mount_stream_id.as_ref().unwrap())
        }
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

enum MessageStatus {
    Accepted = 0,
    Denied = 1,
}

impl<In: Read> Unpack<In> for MessageStatus {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MessageStatus, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MessageStatus::Accepted as i32 => MessageStatus::Accepted,
                    x if x == MessageStatus::Denied as i32 => MessageStatus::Denied,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

enum AcceptStatus {
    Success = 0,
    ProgUnavail = 1,
    ProgMismatch = 2,
    ProcUnavail = 3,
    GarbageArgs = 4,
}

impl<In: Read> Unpack<In> for AcceptStatus {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AcceptStatus, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AcceptStatus::Success as i32 => AcceptStatus::Success,
                    x if x == AcceptStatus::ProgUnavail as i32 => AcceptStatus::ProgUnavail,
                    x if x == AcceptStatus::ProgMismatch as i32 => AcceptStatus::ProgMismatch,
                    x if x == AcceptStatus::ProcUnavail as i32 => AcceptStatus::ProcUnavail,
                    x if x == AcceptStatus::GarbageArgs as i32 => AcceptStatus::GarbageArgs,
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
        Self {
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
        Ok(self.xid.pack(out)?
            + (self.msgtype.clone() as u32).pack(out)?
            + xdr_codec::pack_opaque_array(self.body.as_slice(), self.body.len(), out)?)
    }
}

impl<In: Read> Unpack<In> for Message {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Message, usize)> {
        let mut sz = 0;
        Ok((
            Message {
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
        assert_eq!(msg.xid, xid + 1);
    }

    #[test]
    fn message_rpc_version() {
        let msg = Message::new(vec![0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5]);
        assert_eq!(msg.rpc_version(), 2);
    }

    #[test]
    fn message_program() {
        let msg = Message::new(vec![0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5]);
        assert_eq!(msg.program(), 3);
    }

    #[test]
    fn message_version() {
        let msg = Message::new(vec![0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5]);
        assert_eq!(msg.version(), 4);
    }

    #[test]
    fn message_procedure() {
        let msg = Message::new(vec![0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5]);
        assert_eq!(msg.procedure(), 5);
    }
}
