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
use crate::bindings::wasi::sockets::network::{
    ErrorCode, IpAddress, IpAddressFamily, IpSocketAddress, Ipv4SocketAddress, Ipv6SocketAddress,
    Network,
};
use crate::bindings::wasi::sockets::tcp::TcpSocket;
use crate::{add_resource, get_resource, remove_resource};
use wit_bindgen::rt::{RustResource, WasmResource};
use xdr_codec::Write;

fn check_error<T>(res: Result<T, ErrorCode>, err_msg: &str) -> io::Result<T> {
    res.map_err(|code| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("{} - {}", err_msg, code.message()).as_str(),
        )
    })
}

fn check_error_retrying_on_would_block<X, T>(
    x: &X,
    f: fn(x: &X) -> Result<T, ErrorCode>,
    err_msg: &str,
) -> io::Result<T>
where
    T: std::fmt::Debug,
{
    loop {
        let res = f(x);
        if !res.is_err() {
            return Ok(res.unwrap());
        }
        let code = res.unwrap_err();
        if code != ErrorCode::WouldBlock {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{} - {}", err_msg, code.message()).as_str(),
            ));
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub struct TcpStream {
    sock: Option<TcpSocket>,
    input: Option<InputStream>,
    output: Option<OutputStream>,
}

impl From<Ipv4SocketAddress> for IpAddr {
    fn from(ip4: Ipv4SocketAddress) -> Self {
        IpAddr::V4(Ipv4Addr::new(
            ip4.address.0,
            ip4.address.1,
            ip4.address.2,
            ip4.address.3,
        ))
    }
}

impl From<Ipv6SocketAddress> for IpAddr {
    fn from(ip6: Ipv6SocketAddress) -> Self {
        IpAddr::V6(Ipv6Addr::new(
            ip6.address.0,
            ip6.address.1,
            ip6.address.2,
            ip6.address.3,
            ip6.address.4,
            ip6.address.5,
            ip6.address.6,
            ip6.address.7,
        ))
    }
}

impl From<IpAddress> for IpAddr {
    fn from(ip: IpAddress) -> Self {
        match ip {
            IpAddress::Ipv4(address) => Ipv4SocketAddress { address, port: 0 }.into(),
            IpAddress::Ipv6(address) => Ipv6SocketAddress {
                address,
                port: 0,
                flow_info: 0,
                scope_id: 0,
            }
            .into(),
        }
    }
}

impl From<Ipv4SocketAddress> for SocketAddr {
    fn from(ip4: Ipv4SocketAddress) -> Self {
        Self::new(ip4.into(), ip4.port)
    }
}

impl From<Ipv6SocketAddress> for SocketAddr {
    fn from(ip6: Ipv6SocketAddress) -> Self {
        Self::new(ip6.into(), ip6.port)
    }
}

impl From<IpSocketAddress> for SocketAddr {
    fn from(addr: IpSocketAddress) -> Self {
        match addr {
            IpSocketAddress::Ipv4(ip4) => ip4.into(),
            IpSocketAddress::Ipv6(ip6) => ip6.into(),
        }
    }
}

impl Into<IpSocketAddress> for &SocketAddrV4 {
    fn into(self) -> IpSocketAddress {
        let octets = self.ip().octets();
        IpSocketAddress::Ipv4(Ipv4SocketAddress {
            address: (octets[0], octets[1], octets[2], octets[3]),
            port: self.port(),
        })
    }
}

impl Into<IpSocketAddress> for &SocketAddrV6 {
    fn into(self) -> IpSocketAddress {
        let segments = self.ip().segments();
        IpSocketAddress::Ipv6(Ipv6SocketAddress {
            address: (
                segments[0],
                segments[1],
                segments[2],
                segments[3],
                segments[4],
                segments[5],
                segments[6],
                segments[7],
            ),
            port: self.port(),
            flow_info: self.flowinfo(),
            scope_id: self.scope_id(),
        })
    }
}

impl Into<IpSocketAddress> for &SocketAddr {
    fn into(self) -> IpSocketAddress {
        match self {
            SocketAddr::V4(addr4) => addr4.into(),
            SocketAddr::V6(addr6) => addr6.into(),
        }
    }
}

impl Into<sockets::tcp::ShutdownType> for Shutdown {
    fn into(self) -> sockets::tcp::ShutdownType {
        match self {
            Shutdown::Read => sockets::tcp::ShutdownType::Receive,
            Shutdown::Write => sockets::tcp::ShutdownType::Send,
            Shutdown::Both => sockets::tcp::ShutdownType::Both,
        }
    }
}

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = <Vec<SocketAddr> as IntoIterator>::IntoIter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let (host, port) = *self;
        let addrs = if let Ok(addr) = host.parse::<Ipv4Addr>() {
            vec![SocketAddr::V4(SocketAddrV4::new(addr, port))]
        } else if let Ok(addr) = host.parse::<Ipv6Addr>() {
            vec![SocketAddr::V6(SocketAddrV6::new(addr, port, 0, 0))]
        } else {
            LookupHost::try_from(*self)?.collect()
        };
        if !addrs.is_empty() {
            return Ok(addrs.into_iter());
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "could not resolve host",
        ))
    }
}

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
        let wasi_addr = addr.into();
        let address_family = match wasi_addr {
            IpSocketAddress::Ipv4(_) => IpAddressFamily::Ipv4,
            IpSocketAddress::Ipv6(_) => IpAddressFamily::Ipv6,
        };
        let sock = check_error(
            sockets::tcp_create_socket::create_tcp_socket(address_family),
            "create tcp socket error",
        )?;
        let nw = sockets::instance_network::instance_network();
        let _ = check_error(sock.start_connect(&nw, wasi_addr), "start connect error")?;
        let (input, output) = check_error_retrying_on_would_block(
            &sock,
            |s: &TcpSocket| s.finish_connect(),
            "finish connect error",
        )?;
        let stream = Self {
            sock: Some(sock),
            input: Some(input),
            output: Some(output),
        };
        let _ = stream.peer_addr()?;
        Ok(stream)
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
        let res = self.input.as_ref().unwrap().blocking_read(len);
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
                // XXX: despite documentation for wasi::io::streams::blocking_read stating that it should
                //      block until at least one byte can be read, it seems that host implementations can
                //      differ - with some returning empty array when they really should keep blocking
                //      so, let's "handle" this scenario by blocking (sleeping) on zero bytes read
                Ok(0) => std::thread::sleep(std::time::Duration::from_millis(1)),
                Ok(n) => buf = &mut buf[n..],
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }
        Ok(())
    }

    pub fn read_vectored(&self, iov: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        iov.iter_mut().map(|buf| self.read(buf)).sum()
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        // XXX: according to the documentation for wasi::io::streams::OutputStream::blocking_write_and_flush,
        //      it only writes up to 4096 bytes - so we have to make sure not to pass in all of `buf`, if it
        //      is larger than that
        let sz = buf.len().min(4096);
        self.output
            .as_ref()
            .unwrap()
            .blocking_write_and_flush(&buf[..sz]);
        Ok(sz)
    }

    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ))
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn write_vectored(&self, iov: &[IoSlice<'_>]) -> io::Result<usize> {
        iov.iter().map(|buf| self.write(buf)).sum()
    }

    pub fn is_write_vectored(&self) -> bool {
        true
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        check_error(
            self.sock.as_ref().unwrap().remote_address(),
            "error getting remote address",
        )
        .map(Into::into)
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        check_error(
            self.sock.as_ref().unwrap().local_address(),
            "error getting local address",
        )
        .map(Into::into)
    }

    pub fn shutdown(&self, shutdown_type: Shutdown) -> io::Result<()> {
        check_error(
            self.sock.as_ref().unwrap().shutdown(shutdown_type.into()),
            "shutdown error",
        )
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
            TcpStream {
                sock: Some(TcpSocket::from_handle(self.sock.as_ref().unwrap().handle())),
                input: Some(InputStream::from_handle(
                    self.input.as_ref().unwrap().handle(),
                )),
                output: Some(OutputStream::from_handle(
                    self.output.as_ref().unwrap().handle(),
                )),
            }
        })
    }
}

unsafe impl RustResource for TcpStream {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for TcpStream {
    unsafe fn drop(handle: u32) {
        remove_resource(handle);
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream")
            .field("sock", &self.sock.as_ref().unwrap().handle())
            .field("input", &self.input.as_ref().unwrap().handle())
            .field("output", &self.output.as_ref().unwrap().handle())
            .finish()
    }
}

struct LookupHost {
    stream: ResolveAddressStream,
    port: u16,
}

impl Iterator for LookupHost {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<SocketAddr> {
        check_error_retrying_on_would_block(
            &self.stream,
            |stream: &ResolveAddressStream| stream.resolve_next_address(),
            "host lookup error",
        )
        .map(|opt_ip| opt_ip.map(|ip| SocketAddr::new(ip.into(), self.port)))
        .map_err(|_e| Option::<SocketAddr>::None)
        .unwrap()
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(s: &str) -> io::Result<LookupHost> {
        let mut parts_iter = s.rsplitn(2, ':');
        let port_str = parts_iter.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid socket address",
        ))?;
        let host = parts_iter.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid socket address",
        ))?;
        let port: u16 = port_str
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid port value"))?;
        (host, port).try_into()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from((host, port): (&'a str, u16)) -> io::Result<LookupHost> {
        let nw = sockets::instance_network::instance_network();
        let stream = check_error(
            sockets::ip_name_lookup::resolve_addresses(&nw, host),
            "addr_resolve error",
        )?;
        Ok(LookupHost { stream, port })
    }
}
