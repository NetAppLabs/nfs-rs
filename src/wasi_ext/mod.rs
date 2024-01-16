use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io;
use std::io::{IoSlice, IoSliceMut};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::Duration;
use std::unimplemented;

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
    sock: TcpSocket,
    input: InputStream,
    output: OutputStream,
}

pub fn sock_addr_remote(sock: &TcpSocket) -> io::Result<IpSocketAddress> {
    let res = sock.remote_address();
    check_error(&res, "error getting remote address")?;
    Ok(res.unwrap())
}

pub fn sock_addr_local(sock: &TcpSocket) -> io::Result<IpSocketAddress> {
    let res = sock.local_address();
    check_error(&res, "error getting local address")?;
    Ok(res.unwrap())
}

pub fn to_socket_addr(addr: &IpSocketAddress) -> io::Result<SocketAddr> {
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

pub fn to_wasi_addr(addr: &SocketAddr) -> io::Result<IpSocketAddress> {
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

fn to_socket_addrs(addr: (&str, u16)) -> io::Result<Vec<SocketAddr>> {
    let (host, port) = addr;
    if let Ok(addr) = host.parse::<Ipv4Addr>() {
        let addr = SocketAddrV4::new(addr, port);
        return Ok(vec![SocketAddr::V4(addr)]);
    }
    if let Ok(addr) = host.parse::<Ipv6Addr>() {
        let addr = SocketAddrV6::new(addr, port, 0, 0);
        return Ok(vec![SocketAddr::V6(addr)]);
    }
    let mut addrs = vec![];
    for a in LookupHost::try_from(addr)? {
        addrs.push(a);
    }
    if !addrs.is_empty() {
        return Ok(addrs);
    }
    Err(io::Error::new(io::ErrorKind::Other, "could not resolve host"))
}

impl TcpStream {
    pub fn connect(addr: (&str, u16)) -> io::Result<TcpStream> {
        for addr in to_socket_addrs(addr)? {
            let res = to_wasi_addr(&addr);
            if res.is_err() {
                println!("TcpStream::connect to_wasi_addr err: {}", res.err().unwrap());
                continue;
            }
            let wasi_addr = res.unwrap();

            let address_family = match wasi_addr {
                IpSocketAddress::Ipv4(_) => IpAddressFamily::Ipv4,
                IpSocketAddress::Ipv6(_) => IpAddressFamily::Ipv6,
            };
            let res = sockets::tcp_create_socket::create_tcp_socket(address_family);
            if res.is_err() {
                let code = res.unwrap_err();
                println!("TcpStream::connect sock_open err: ({}) {}", code, code.message());
                continue;
            }
            let sock = res.unwrap();

            let nw = sockets::instance_network::instance_network();
            let res = sock.start_connect(&nw, wasi_addr);
            if res.is_err() {
                let code = res.unwrap_err();
                println!("TcpStream::connect sock_connect err: ({}) {}", code, code.message());
                // sock.drop_tcp_socket(); FIXME: replace with call to shutdown?
                continue;
            }

            let res = sock.finish_connect();
            if res.is_err() {
                let code = res.unwrap_err();
                println!("TcpStream::connect sock_connect err: ({}) {}", code, code.message());
                // sock.drop_tcp_socket(); FIXME: replace with call to shutdown?
                continue;
            }

            let (input, output) = res.unwrap();
            let stream = Self{sock, input, output};
            if let Some(err) = stream.peer_addr().err() {
                if let Some(os_err) = err.raw_os_error() {
                    if os_err != ERRNO_SUCCESS { // XXX: don't print out "success" error
                        println!("TcpStream::connect sock_peer_addr err: {}", err);
                    }
                }
                // sock.drop_tcp_socket(); FIXME: replace with call to shutdown?
                continue;
            }
            return Ok(stream);
        }

        Err(io::Error::new(io::ErrorKind::Other, "no valid socket address"))
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

    pub fn read(&self, mut buf: &mut [u8]) -> io::Result<usize> {
        // XXX: there is no wasi::sockets::tcp::recv or equivalent -- should use wasi::io::streams::read or,
        //      what is probably more apt in our case, wasi::io::streams::blocking_read
        let len = buf.len() as u64;
        let res = self.input.blocking_read(len);
        if res.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "sock_recv error"));
        }
        let received_bytes = res.unwrap();
        buf.write(received_bytes.as_slice());
        Ok(received_bytes.len())
    }

    pub fn read_exact(&self, mut buf: &mut [u8]) -> io::Result<()> {
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

    pub fn read_vectored(&self, iov: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        let mut res = 0;
        for buf in iov {
            res += self.read(buf).unwrap();
        }
        Ok(res)
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        // XXX: according to the documentation for wasi::io::streams::OutputStream::blocking_write_and_flush,
        //      it only writes up to 4096 bytes - so we have to make sure not to pass in all of `buf`, if it
        //      is larger than that
        let sz = buf.len().min(4096);
        self.output.blocking_write_and_flush(&buf[..sz]);
        Ok(sz)
    }

    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
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

    pub fn write_vectored(&self, iov: &[IoSlice<'_>]) -> io::Result<usize> {
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
        unimplemented!("socket_addr")
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

pub struct LookupHost {
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
