use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io;
use std::io::{IoSlice, IoSliceMut};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddrV4, SocketAddrV6};
use std::time::Duration;
use std::unimplemented;

pub use std::net::SocketAddr;

use crate::bindings::wasi::io::streams::{InputStream, OutputStream};
use crate::bindings::wasi::sockets;
use crate::bindings::wasi::sockets::ip_name_lookup::ResolveAddressStream;
use crate::bindings::wasi::sockets::network::{ErrorCode, IpAddressFamily, IpSocketAddress, Ipv4SocketAddress, Ipv6SocketAddress, Network};
use crate::bindings::wasi::sockets::tcp::TcpSocket;
use xdr_codec::Write;

const ERRNO_SUCCESS: i32 = 0;

fn check_error<T>(res: &Result<T, ErrorCode>, err_msg: &str) -> io::Result<()> {
    if res.is_err() {
        let code = res.as_ref().err().unwrap();
        return Err(io::Error::new(io::ErrorKind::Other, format!("{} - {}", err_msg, code.message()).as_str()));
    }
    Ok(())
}

pub struct TcpStream {
    socket_addr: SocketAddr,
    sock: TcpSocket,
    input: InputStream,
    output: OutputStream,
}

fn sock_addr_remote(sock: &TcpSocket) -> io::Result<IpSocketAddress> {
    let res = sock.remote_address();
    check_error(&res, "error getting remote address")?;
    Ok(res.unwrap())
}

fn sock_addr_local(sock: &TcpSocket) -> io::Result<IpSocketAddress> {
    let res = sock.local_address();
    check_error(&res, "error getting local address")?;
    Ok(res.unwrap())
}

fn to_socket_addr(addr: &IpSocketAddress) -> io::Result<SocketAddr> {
    match addr {
        IpSocketAddress::Ipv4(ref ip4) => Ok(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                ip4.address.0,
                ip4.address.1,
                ip4.address.2,
                ip4.address.3,
            )),
            ip4.port,
        )),
        IpSocketAddress::Ipv6(ref ip6) => Ok(SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                ip6.address.0,
                ip6.address.1,
                ip6.address.2,
                ip6.address.3,
                ip6.address.4,
                ip6.address.5,
                ip6.address.6,
                ip6.address.7,
            )),
            ip6.port,
        )),
    }
}

fn to_wasi_addr(addr: &SocketAddr) -> io::Result<IpSocketAddress> {
    match addr {
        SocketAddr::V4(ref addr) => {
            let octets = addr.ip().octets();
            Ok(IpSocketAddress::Ipv4(Ipv4SocketAddress{
                address: (octets[0], octets[1], octets[2], octets[3]),
                port: addr.port(),
            }))
        },
        SocketAddr::V6(ref addr) => {
            let segments = addr.ip().segments();
            Ok(IpSocketAddress::Ipv6(Ipv6SocketAddress{
                address: (segments[0], segments[1], segments[2], segments[3], segments[4], segments[5], segments[6], segments[7]),
                port: addr.port(),
                flow_info: 0,
                scope_id: 0,
            }))
        },
    }
}

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = <Vec<SocketAddr> as IntoIterator>::IntoIter;
    fn to_socket_addrs(&self) -> io::Result<<Vec<SocketAddr> as IntoIterator>::IntoIter> {
        let (host, port) = *self;
        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            return Ok(vec![SocketAddr::V4(addr)].into_iter());
        }
        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            return Ok(vec![SocketAddr::V6(addr)].into_iter());
        }
        let mut addrs = vec![];
        for a in LookupHost::try_from(*self)? {
            addrs.push(a);
        }
        if !addrs.is_empty() {
            return Ok(addrs.into_iter());
        }
        Err(io::Error::new(io::ErrorKind::Other, "could not resolve host"))
    }
}

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
        let res = to_wasi_addr(&addr);
        if res.is_err() {
            return Err(res.unwrap_err());
        }

        let wasi_addr = res.unwrap();
        let address_family = match wasi_addr {
            IpSocketAddress::Ipv4(_) => IpAddressFamily::Ipv4,
            IpSocketAddress::Ipv6(_) => IpAddressFamily::Ipv6,
        };
        let res = sockets::tcp_create_socket::create_tcp_socket(address_family);
        check_error(&res, "create tcp socket error")?;

        let sock = res.unwrap();
        let nw = sockets::instance_network::instance_network();
        let res = sock.start_connect(&nw, wasi_addr);
        check_error(&res, "start connect error")?;

        let res = sock.finish_connect();
        check_error(&res, "finish connect error")?;

        let (input, output) = res.unwrap();
        let socket_addr = addr.clone();
        let stream = Self{socket_addr, sock, input, output};
        if let Some(err) = stream.peer_addr().err() {
            return Err(err);
        }

        Ok(stream)
    }


    pub fn reconnect(&mut self) -> io::Result<()> {
        let addr = self.socket_addr;
        let res = to_wasi_addr(&addr);
        if res.is_err() {
            return Err(res.unwrap_err());
        }

        let wasi_addr = res.unwrap();
        let address_family = match wasi_addr {
            IpSocketAddress::Ipv4(_) => IpAddressFamily::Ipv4,
            IpSocketAddress::Ipv6(_) => IpAddressFamily::Ipv6,
        };
        let res = sockets::tcp_create_socket::create_tcp_socket(address_family);
        check_error(&res, "create tcp socket error")?;

        let sock = res.unwrap();
        let nw = sockets::instance_network::instance_network();
        let res = sock.start_connect(&nw, wasi_addr);
        check_error(&res, "start connect error")?;

        let res = sock.finish_connect();
        check_error(&res, "finish connect error")?;

        let (input, output) = res.unwrap();
        let socket_addr = addr.clone();
        //let stream = Self{socket_addr, sock, input, output};
        //if let Some(err) = stream.peer_addr().err() {
        //    return Err(err);
        //}
        self.input = input;
        self.output = output;
        self.sock = sock;
        //Ok(stream)
        Ok(())
    }


    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unimplemented!("connect_timeout")
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!("read_timeout")
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!("write_timeout")
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unimplemented!("peek")
    }

    pub fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        return self.read_with_reconnect(buf, 5);
    }

    pub fn read_with_reconnect(&mut self, mut buf: &mut [u8], reconnect_try: u32) -> io::Result<usize> {
        // XXX: there is no wasi::sockets::tcp::recv or equivalent -- should use wasi::io::streams::read or,
        //      what is probably more apt in our case, wasi::io::streams::blocking_read
        let len = buf.len() as u64;
        let res = self.input.blocking_read(len);
        match res {
            Ok(received_bytes) => {
                buf.write(received_bytes.as_slice());
                return Ok(received_bytes.len());
            },
            Err(str_err) => {
                match str_err {
                    crate::bindings::wasi::io::streams::StreamError::LastOperationFailed(lofe) => {
                        return Err(io::Error::new(io::ErrorKind::Other, lofe.to_debug_string()));
                    },
                    crate::bindings::wasi::io::streams::StreamError::Closed => {
                        if reconnect_try > 0 {
                            self.reconnect()?;
                            return self.read_with_reconnect(buf, reconnect_try -1 )
                        } else {
                            return Err(io::Error::new(io::ErrorKind::Other, "sock_recv error closed. reconnect exhausted"));
                        }
                    }
                }
            },
        }
    }

    pub fn read_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
        } else {
            Ok(())
        }
    }

    pub fn read_vectored(&mut self, iov: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        let mut res = 0;
        for buf in iov {
            res += self.read(buf).unwrap();
        }
        Ok(res)
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        return self.write_with_reconnect(buf, 5);
    }

    pub fn write_with_reconnect(&mut self, buf: &[u8], reconnect_try: u32) -> io::Result<usize> {
        // XXX: according to the documentation for wasi::io::streams::OutputStream::blocking_write_and_flush,
        //      it only writes up to 4096 bytes - so we have to make sure not to pass in all of `buf`, if it
        //      is larger than that

        let mut buf_len = 4096;

        let buf_len_res = self.output.check_write();
        match buf_len_res {
            Ok(buf_len_u64) => {
                buf_len = buf_len_u64 as usize;
            },
            Err(_) => {},
        }
        let sz = buf.len().min(buf_len);

        let res = self.output.blocking_write_and_flush(&buf[..sz]);
        match res {
            Ok(_) => {
                Ok(sz)
            }
            Err(err) => {
                    match err {
                        crate::bindings::wasi::io::streams::StreamError::LastOperationFailed(lofe) => {
                            return Err(io::Error::new(io::ErrorKind::Other, lofe.to_debug_string()));
                        },
                        crate::bindings::wasi::io::streams::StreamError::Closed => {
                            if reconnect_try > 0 {
                                self.reconnect()?;
                                return self.write_with_reconnect(buf, reconnect_try -1 )
                            } else {
                                return Err(io::Error::new(io::ErrorKind::Other, "sock_recv error closed. reconnect exhausted"));
                            }
                        }
    
                    }
            }
        }
    }

    pub fn write_all(&mut self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole buffer")),
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn write_vectored(&mut self, iov: &[IoSlice<'_>]) -> io::Result<usize> {
        let mut res = 0;
        for buf in iov {
            res += self.write(buf).unwrap();
        }
        Ok(res)
    }

    pub fn is_write_vectored(&self) -> bool {
        true
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        to_socket_addr(&sock_addr_remote(&self.sock)?)
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        return Ok(self.socket_addr)
    }

    pub fn shutdown(&self, shutdown_type: Shutdown) -> io::Result<()> {
        let shutdown_type = match shutdown_type {
            Shutdown::Read => sockets::tcp::ShutdownType::Receive,
            Shutdown::Write => sockets::tcp::ShutdownType::Send,
            Shutdown::Both => sockets::tcp::ShutdownType::Both,
        };
        let res = self.sock.shutdown(shutdown_type);
        check_error(&res, "shutdown error")
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unimplemented!("duplicate")
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_nodelay")
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unimplemented!("nodelay")
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unimplemented!("set_ttl")
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unimplemented!("ttl")
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!("take_error")
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_nonblocking")
    }

    pub fn try_clone(&self) -> io::Result<TcpStream> {
        Ok(unsafe {
            TcpStream{
                socket_addr: self.socket_addr.clone(),
                sock: TcpSocket::from_handle(self.sock.handle()),
                input: InputStream::from_handle(self.input.handle()),
                output: OutputStream::from_handle(self.output.handle()),
            }
        })
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream")
            .field("sock", &self.sock.handle())
            .finish()
    }
}

struct LookupHost {
    stream: ResolveAddressStream,
    port: u16,
}

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        let mut res = Err(ErrorCode::Unknown);
        loop {
            res = self.stream.resolve_next_address();
            if res.is_err() {
                let code = res.unwrap_err();
                if code == ErrorCode::WouldBlock { // TODO: also sleep and retry on ErrorCode::TemporaryResolverFailure?
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                }
                // self.stream.drop_resolve_address_stream(); FIXME?
                return None;
            }
            break;
        }
        if let Some(addr_) = res.unwrap() {
            let addr = match addr_ {
                sockets::network::IpAddress::Ipv4(addr4) => IpSocketAddress::Ipv4(Ipv4SocketAddress{port: self.port, address: addr4}),
                sockets::network::IpAddress::Ipv6(addr6) => IpSocketAddress::Ipv6(Ipv6SocketAddress{port: self.port, address: addr6, flow_info: 0, scope_id: 0}),
            };
            to_socket_addr(&addr).ok()
        } else {
            // self.stream.drop_resolve_address_stream(); FIXME?
            None
        }
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(s: &str) -> io::Result<LookupHost> {
        macro_rules! try_opt {
            ($e:expr, $msg:expr) => {
                match $e {
                    Some(r) => r,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
                }
            };
        }

        // split the string by ':' and convert the second part to u16
        let mut parts_iter = s.rsplitn(2, ':');
        let port_str = try_opt!(parts_iter.next(), "invalid socket address");
        let host = try_opt!(parts_iter.next(), "invalid socket address");
        let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");

        (host, port).try_into()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from((host, port): (&'a str, u16)) -> io::Result<LookupHost> {
        let nw = sockets::instance_network::instance_network();
        let res = sockets::ip_name_lookup::resolve_addresses(&nw, host);
        check_error(&res, "addr_resolve error")?;
        let stream = res.unwrap();
        Ok(LookupHost{stream, port})
    }
}
