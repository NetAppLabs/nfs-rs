use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io::{self, IoSlice, IoSliceMut};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::Duration;
use std::unimplemented;

use bindings::component::nfs_rs_component::wasi_experimental_sockets;
use xdr_codec::Write;

mod errno;
use errno::*;

fn to_error(code: u16, msg: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, format!("{} - {}", msg, strerror(code)).as_str());
}

#[derive(Debug)]
pub struct WasiFd {
    fd: wasi_experimental_sockets::Fd,
}

impl WasiFd {
    pub unsafe fn from_raw(fd: wasi_experimental_sockets::Fd) -> WasiFd {
        WasiFd { fd }
    }

    pub fn into_raw(self) -> wasi_experimental_sockets::Fd {
        let ret = self.fd;
        mem::forget(self);
        ret
    }

    pub fn as_raw(&self) -> wasi_experimental_sockets::Fd {
        self.fd
    }
}

pub struct TcpStream {
    fd: WasiFd,
}

pub unsafe fn sock_addr_remote(fd: u32) -> io::Result<wasi_experimental_sockets::Addr> {
    let res = wasi_experimental_sockets::sock_addr_remote(fd);
    if res.is_err() {
        return Err(to_error(res.unwrap_err(), "error getting remote address"));
    }
    Ok(res.unwrap())
}
pub unsafe fn sock_addr_local(fd: u32) -> io::Result<wasi_experimental_sockets::Addr> {
    let res = wasi_experimental_sockets::sock_addr_local(fd);
    if res.is_err() {
        return Err(to_error(res.unwrap_err(), "error getting local address"));
    }
    Ok(res.unwrap())
}

pub unsafe fn to_socket_addr(addr: &wasi_experimental_sockets::Addr) -> io::Result<SocketAddr> {
    match addr.u {
        wasi_experimental_sockets::AddrU::AddrIp4Port(ip4) => Ok(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                ip4.addr.n0,
                ip4.addr.n1,
                ip4.addr.h0,
                ip4.addr.h1,
            )),
            ip4.port,
        )),
        wasi_experimental_sockets::AddrU::AddrIp6Port(ip6) => Ok(SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                ip6.addr.n0,
                ip6.addr.n1,
                ip6.addr.n2,
                ip6.addr.n3,
                ip6.addr.h0,
                ip6.addr.h1,
                ip6.addr.h2,
                ip6.addr.h3,
            )),
            ip6.port,
        )),
    }
}

pub unsafe fn to_wasi_addr(addr: &SocketAddr) -> io::Result<wasi_experimental_sockets::Addr> {
    match addr {
        SocketAddr::V4(ref addr) => {
            let octets = addr.ip().octets();
            Ok(wasi_experimental_sockets::Addr {
                tag: wasi_experimental_sockets::AddrType::Ip4,
                u: wasi_experimental_sockets::AddrU::AddrIp4Port(wasi_experimental_sockets::AddrIp4Port {
                    addr: wasi_experimental_sockets::AddrIp4 {
                        n0: octets[0],
                        n1: octets[1],
                        h0: octets[2],
                        h1: octets[3],
                    },
                    port: addr.port(),
                }),
            })
        }
        SocketAddr::V6(ref addr) => {
            let segments = addr.ip().segments();
            Ok(wasi_experimental_sockets::Addr {
                tag: wasi_experimental_sockets::AddrType::Ip6,
                u: wasi_experimental_sockets::AddrU::AddrIp6Port(wasi_experimental_sockets::AddrIp6Port {
                    addr: wasi_experimental_sockets::AddrIp6 {
                        n0: segments[0],
                        n1: segments[1],
                        n2: segments[2],
                        n3: segments[3],
                        h0: segments[4],
                        h1: segments[5],
                        h2: segments[6],
                        h3: segments[7],
                    },
                    port: addr.port(),
                }),
            })
        }
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
        return Ok(addrs)
    }
    Err(io::Error::new(io::ErrorKind::Other, "could not resolve host"))
}

impl TcpStream {
    pub fn connect(addr: (&str, u16)) -> io::Result<TcpStream> {
        //if let SocketAddr::V4(ipv4) = addr? {

        //println!("net.tcpstream.connect");
        for addr in to_socket_addrs(addr)? {
            let x = unsafe { to_wasi_addr(&addr) };
            if x.is_err() {
                println!("TcpStream::connect to_wasi_addr err: {}", x.err().unwrap());
                continue;
            }
            let wasi_addr = x.unwrap(); // unsafe { to_wasi_addr(&addr).unwrap() };

            let x = unsafe {
                wasi_experimental_sockets::sock_open(wasi_experimental_sockets::AddressFamily::Inet4, wasi_experimental_sockets::SocketType::Strm)
            };
            if x.is_err() {
                let code = x.unwrap_err();
                println!("TcpStream::connect sock_open err: ({}) {}", code, strerror(code));
                continue;
            }
            let wasi_fd = x.unwrap();

            let x = unsafe {
                wasi_experimental_sockets::sock_connect(wasi_fd, wasi_addr)
            };
            if x.is_err() {
                let code = x.unwrap_err();
                // if code != 0 {
                    println!("TcpStream::connect sock_connect err: ({}) {}", code, strerror(code));
                    let x = unsafe { wasi_experimental_sockets::sock_close(wasi_fd) };
                    if x.is_err() {
                        let code = x.unwrap_err();
                        println!("TcpStream::connect sock_close err: ({}) {}", code, strerror(code));
                    }
                    continue;
                // }
            }

            let stream = Self{fd: unsafe { WasiFd::from_raw(wasi_fd) }};
            if let Some(err) = stream.peer_addr().err() {
                println!("TcpStream::connect sock_peer_addr err: {}", err);
                let x = unsafe { wasi_experimental_sockets::sock_close(wasi_fd) };
                if x.is_err() {
                    let code = x.unwrap_err();
                    println!("TcpStream::connect sock_close err: ({}) {}", code, strerror(code));
                }
                continue;
            }
            return Ok(stream);
        }

        Err(to_error(ERRNO_NOENT, "no valid socket address"))
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
        let res = unsafe {
            wasi_experimental_sockets::sock_recv(self.fd.as_raw(), buf.len().try_into().unwrap(), 0)
        };
        if res.is_err() {
            let code = res.unwrap_err();
            if code == ERRNO_AGAIN {
                std::thread::sleep(std::time::Duration::from_millis(1));
                return self.read(buf);
            }
            println!("TcpStream::read sock_recv err: ({}) {}", code, strerror(code));
            return Err(to_error(code, "sock_recv error"));
        }
        let received = res.unwrap();
        buf.write(received.as_slice());
        Ok(received.len())
    }

    pub fn read_exact(&self, mut buf: &mut [u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
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
        let res = unsafe { wasi_experimental_sockets::sock_send(self.fd.as_raw(), buf, buf.len().try_into().unwrap(), 0) };
        if res.is_err() {
            let code = res.unwrap_err();
            println!("TcpStream::write sock_send err: ({}) {}", code, strerror(code));
            return Err(to_error(code, "sock_send error"));
        }
        Ok(res.unwrap().try_into().unwrap())
    }

    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole buffer"));
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
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
        unsafe { to_socket_addr(&sock_addr_remote(self.fd.fd)?) }
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!("socket_addr")
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        unimplemented!("shutdown")
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

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }

    pub fn try_clone(&self) -> io::Result<TcpStream> {
        Ok(TcpStream{fd: WasiFd{fd: self.fd.fd}})
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream")
            .field("fd", &self.fd.as_raw())
            .finish()
    }
}

pub struct LookupHost {
    stream: u32,
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
        let mut res = Err(ERRNO_AGAIN);
        loop {
            res = unsafe {
                wasi_experimental_sockets::addr_resolve_stream_next(self.stream)
            };
            if res.is_err() {
                let err = res.err().unwrap();
                if err == ERRNO_AGAIN {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                } else {
                    let _ = unsafe {
                        wasi_experimental_sockets::addr_resolve_stream_dispose(self.stream)
                    };
                    return None;
                }
            }
            break;
        }
        if let Some(addr) = res.unwrap() {
            unsafe { to_socket_addr(&addr).ok() }
        } else {
            let _ = unsafe {
                wasi_experimental_sockets::addr_resolve_stream_dispose(self.stream)
            };
            None
        }

        // const IPV4_LEN: usize = 8;
        // const IPV6_LEN: usize = 20;
        // loop {
        //     unsafe {
        //         if self.index < self.res.len() {
        //             let cur = self.res[self.index];
        //             if let Some(addr) = to_socket_addr(&cur).ok() {
        //                 let size = if addr.is_ipv4() { IPV4_LEN } else { IPV6_LEN };
        //                 self.index += size;
        //                 return Some(addr);
        //             }
        //             let size = if cur.tag == wasi_experimental_sockets::AddrType::Ip4 { IPV4_LEN } else { IPV6_LEN };
        //             self.index += size;
        //         } else {
        //             return None;
        //         }
        //     }
        // }
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
        let p = match port {
            0 => None,
            _ => Some(port),
        };
        //const BUF_LEN: usize = 20;
        //wasi_experimental_sockets::sock_addr_remote(fd,  buf.as_mut_ptr(), BUF_LEN).unwrap();
        // TODO: handle error
        let res = unsafe {
            wasi_experimental_sockets::addr_resolve(&host, p)
        };
        if res.is_err() {
            let code = res.unwrap_err();
            println!("LookupHost::try_from addr_resolve err: ({}) {}", code, strerror(code));
            return Err(to_error(code, "addr_resolve error"));
        }

        let stream = res.unwrap();
        // println!("LookupHost::try_from addr_resolve stream: {}", stream);
        Ok(LookupHost {
            stream,
            port,
        })
    }
}
