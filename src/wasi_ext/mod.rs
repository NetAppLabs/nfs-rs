use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::io::{self, IoSlice, IoSliceMut};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
//use std::sys::fd::WasiFd;
//use std::sys_common::FromInner;

use std::time::Duration;
use std::unimplemented;


pub mod error;
#[allow(unused)]
mod lib_generated;

use lib_generated as wasi_experimental_sockets;
use wasi_experimental_sockets::*;

//use super::err2io;
//use crate::mem;
//use crate::cell::UnsafeCell;
//use super::{iovecmut,ciovecmut};
//use super::{iovecmut};
//use super::{ciovecmut};
//use super::{copy_buf_mut_slice};

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

const BUF_LEN: usize = 20;
pub unsafe fn sock_addr_remote(fd: u32) -> wasi_experimental_sockets::Addr {
    let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
    wasi_experimental_sockets::sock_addr_remote(fd, buf.as_mut_ptr(), BUF_LEN).unwrap();
    let addr_ptr = buf.as_ptr() as *const wasi_experimental_sockets::Addr;
    addr_ptr.read_unaligned()
}
pub unsafe fn sock_addr_local(fd: u32) -> wasi_experimental_sockets::Addr {
    let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
    wasi_experimental_sockets::sock_addr_local(fd, buf.as_mut_ptr(), BUF_LEN).unwrap();
    let addr_ptr = buf.as_ptr() as *const wasi_experimental_sockets::Addr;
    addr_ptr.read_unaligned()
}

pub unsafe fn to_socket_addr(addr: &wasi_experimental_sockets::Addr) -> io::Result<SocketAddr> {
    if addr.tag == wasi_experimental_sockets::ADDR_TYPE_IP4 {
        let sock_addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                addr.u.ip4.addr.n0,
                addr.u.ip4.addr.n1,
                addr.u.ip4.addr.h0,
                addr.u.ip4.addr.h1,
            )),
            addr.u.ip4.port,
        );
        return Ok(sock_addr);
    } else if addr.tag == wasi_experimental_sockets::ADDR_TYPE_IP6 {
        let sock_addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                addr.u.ip6.addr.n0,
                addr.u.ip6.addr.n1,
                addr.u.ip6.addr.n2,
                addr.u.ip6.addr.n3,
                addr.u.ip6.addr.h0,
                addr.u.ip6.addr.h1,
                addr.u.ip6.addr.h2,
                addr.u.ip6.addr.h3,
            )),
            addr.u.ip6.port,
        );
        return Ok(sock_addr);
    }
    unimplemented!("to_socket_addr")
}

pub unsafe fn to_wasi_addr(addr: &SocketAddr) -> io::Result<wasi_experimental_sockets::Addr> {
    match addr {
        SocketAddr::V4(ref addr) => {
            let octets = addr.ip().octets();
            //new:
            let wasi_addr = wasi_experimental_sockets::Addr {
                tag: wasi_experimental_sockets::ADDR_TYPE_IP4,
                u: wasi_experimental_sockets::AddrU {
                    ip4: wasi_experimental_sockets::AddrIp4Port {
                        addr: wasi_experimental_sockets::AddrIp4 {
                            n0: octets[0],
                            n1: octets[1],
                            h0: octets[2],
                            h1: octets[3],
                        },
                        port: addr.port(),
                    },
                },
            };
            return Ok(wasi_addr);
        }
        SocketAddr::V6(ref addr) => {
            let segments = addr.ip().segments();
            //new:
            let wasi_addr = wasi_experimental_sockets::Addr {
                tag: wasi_experimental_sockets::ADDR_TYPE_IP6,
                u: wasi_experimental_sockets::AddrU {
                    ip6: wasi_experimental_sockets::AddrIp6Port {
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
                    },
                },
            };
            return Ok(wasi_addr);
        }
    }
}

impl TcpStream {
    pub fn connect<A: std::net::ToSocketAddrs>(addrs: A) -> io::Result<TcpStream> {
        //if let SocketAddr::V4(ipv4) = addr? {

        //println!("net.tcpstream.connect");
        for addr in addrs.to_socket_addrs()? {
            let x = unsafe { to_wasi_addr(&addr) };
            if x.is_err() {
                println!("TcpStream::connect to_wasi_addr err: {}", x.err().unwrap());
                continue;
            }
            let mut wasi_addr = x.unwrap(); // unsafe { to_wasi_addr(&addr).unwrap() };

            let x = unsafe {
                wasi_experimental_sockets::sock_open(wasi_experimental_sockets::ADDRESS_FAMILY_INET4, wasi_experimental_sockets::SOCK_TYPE_SOCKET_STREAM)
            };
            if x.is_err() {
                println!("TcpStream::connect sock_open err: {}", x.err().unwrap());
                continue;
            }
            let wasi_fd = x.unwrap();

            let x = unsafe {
                wasi_experimental_sockets::sock_connect(wasi_fd, &mut wasi_addr as *mut wasi_experimental_sockets::Addr)
            };
            if x.is_err() {
                println!("TcpStream::connect sock_connect err: {}", x.err().unwrap());
                let x = unsafe { wasi_experimental_sockets::sock_close(wasi_fd) };
                if x.is_err() {
                    println!("TcpStream::connect sock_close err: {}", x.err().unwrap());
                }
                continue;
            }

            return Ok(Self{fd: unsafe { WasiFd::from_raw(wasi_fd) }});
        }

        Err(io::Error::new(io::ErrorKind::Other, "no valid socket address"))
        //old:
        //let addr: u32 = ipv4.ip().clone().into();
        //let port: u16 = ipv4.port();
        //let fd = unsafe { wasi_experimental_sockets::sock_connect(addr, port).unwrap() };
        //Ok(Self { fd: unsafe { WasiFd::from_raw(fd) } })
        //} else {
        //    unimplemented!("connect")
        //}
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unimplemented!("connect_timeout")
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        //unimplemented!("set_read_timeout")
        Ok(())
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        //unimplemented!("set_write_timeout")
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

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        //self.read_vectored(&mut [IoSliceMut::new(buf)])
        //println!("net.read");

        let res = unsafe {
            wasi_experimental_sockets::sock_recv(self.fd.as_raw(), buf.as_mut_ptr() as *mut u8, buf.len(), 0)
        };
        if res.is_err() {
            let err = res.err().unwrap();
            if err.raw_error() == ERRNO_AGAIN {
                std::thread::sleep(std::time::Duration::from_millis(1));
                return self.read(buf);
            }
            println!("TcpStream::read sock_recv err: {}", err);
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string().as_str()));
        }
        Ok(res.unwrap())
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
        //Ok(unsafe { wasi_experimental_sockets::sock_recv(self.fd.as_raw(), &mut iovec(iov), iov.len(), 0).unwrap()})
        //let iovecvar = iovec(iov);
        //let iovecptr = iovecvar.as_mut_ptr() as *mut u8;
        //unsafe {
        //    wasi_experimental_sockets::sock_recv(self.fd.as_raw(), iovecptr, iovecvar.len(), 0).map_err(err2io)
        //}
        //let iovec = iovecmut(iov);
        //Ok(unsafe{ wasi_experimental_sockets::sock_recv(self.fd.as_raw(), iovec.as_mut_ptr() as *mut u8, iovec.len(), 0).unwrap()})
        //println!("net.read_vectored");

        let mut res = 0;
        for buf in iov {
            res += self.read(buf).unwrap();
        }
        Ok(res)
        //old:
        //Ok(unsafe { wasi_experimental_sockets::sock_recv(self.fd.as_raw(), iovec(iov), 0).unwrap().0 })
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        //println!("net.write");

        //let bufmut: &mut [u8] = &mut buf;
        //let buf_len = buf.len();
        //let bufmut: &mut [u8; buf_len] = &mut [0; buf_len];
        //copy_buf_mut_slice(bufmut,buf);
        let len = buf.len();
        let buf_ptr: *const u8 = buf.as_ptr();
        let bufmut_ptr: *mut u8 = unsafe { mem::transmute(buf_ptr) };
        let res = unsafe { wasi_experimental_sockets::sock_send(self.fd.as_raw(), bufmut_ptr, len, 0) };
        if res.is_err() {
            println!("TcpStream::write sock_send err: {}", res.err().unwrap());
            return Err(io::Error::new(io::ErrorKind::Other, res.err().unwrap().to_string().as_str()));
        }
        Ok(res.unwrap())
        //old:
        //self.write_vectored(&[IoSlice::new(buf)])
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
        //println!("net.write_vectored");

        //let ciovec = ciovecmut(iov);
        //Ok(unsafe{wasi_experimental_sockets::sock_send(self.fd.as_raw(), ciovec.as_mut_ptr() as *mut u8, ciovec.len(), 0).unwrap()})
        //old:
        // Ok(unsafe { wasi_experimental_sockets::sock_send(self.fd.as_raw(), &mut ciovec(iov), iov.len(), 0).unwrap() })
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
        unsafe { to_socket_addr(&sock_addr_remote(self.fd.fd)) }
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

/*impl FromInner<u32> for TcpStream {
    fn from_inner(fd: u32) -> TcpStream {
        unsafe {
            TcpStream {
                fd: WasiFd::from_raw(fd),
            }
        }
    }
}*/

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream")
            .field("fd", &self.fd.as_raw())
            .finish()
    }
}

pub struct TcpListener {
    fd: WasiFd,
    socket_address: SocketAddr,
}

impl TcpListener {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        //if let SocketAddr::V4(ipv4) = addr? {
        //new:
        let myaddr = addr.unwrap().to_owned();
        let mut wasi_addr = unsafe { to_wasi_addr(&myaddr).unwrap() };
        let wasi_fd = unsafe {
            wasi_experimental_sockets::sock_open(wasi_experimental_sockets::ADDRESS_FAMILY_INET4, wasi_experimental_sockets::SOCK_TYPE_SOCKET_STREAM).unwrap()
        };

        unsafe {
            wasi_experimental_sockets::sock_bind(wasi_fd, &mut wasi_addr as *mut wasi_experimental_sockets::Addr).unwrap();
        };
        let listen_size = 10;
        unsafe { wasi_experimental_sockets::sock_listen(wasi_fd, listen_size).unwrap() };

        //println!("bind for {}: ",myaddr);
        let wasi_fd_raw = unsafe { WasiFd::from_raw(wasi_fd) };
        Ok(Self {
            fd: wasi_fd_raw,
            socket_address: myaddr,
        })

        //old:
        //let addr: u32 = ipv4.ip().clone().into();
        //let port: u16 = ipv4.port();
        //let fd = unsafe { wasi_experimental_sockets::sock_connect(addr, port).unwrap() };
        //Ok(Self { fd: unsafe { WasiFd::from_raw(fd) } })
        //} else {
        //    unimplemented!("bind")
        //}
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.socket_address)
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let wasi_childfd = unsafe { wasi_experimental_sockets::sock_accept(self.fd.as_raw()).unwrap() };
        //let socket_addr = self.socket_addr();
        let remote_addr_wasi = unsafe { sock_addr_remote(wasi_childfd) };
        let remote_addr = unsafe { to_socket_addr(&remote_addr_wasi).unwrap() };
        //println!("Accept from {}: ",remote_addr);

        //let mut wasi_addr = unsafe { to_wasi_addr(addr.unwrap()).unwrap() };
        //let wasi_fd = unsafe { wasi_experimental_sockets::sock_open(wasi_experimental_sockets::ADDRESS_FAMILY_INET4, wasi_experimental_sockets::SOCK_TYPE_SOCKET_STREAM).unwrap() };
        //unsafe { wasi_experimental_sockets::sock_connect(wasi_fd, &mut wasi_addr as *mut wasi_experimental_sockets::Addr).unwrap(); };
        let wasi_childfd_raw = unsafe { WasiFd::from_raw(wasi_childfd) };
        let tcp_stream = TcpStream {
            fd: wasi_childfd_raw,
        };
        //let tcp_stream = TcpStream::connect(Ok(&remote_addr)).unwrap();
        Ok((tcp_stream, remote_addr))
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unimplemented!("duplicate")
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unimplemented!("set_ttl")
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unimplemented!("ttl")
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_only_v6")
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        unimplemented!("only_v6")
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!("take_error")
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        //unimplemented!("set_nonblocking")
        Ok(())
    }

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }
}

/*
impl FromInner<u32> for TcpListener {
    fn from_inner(fd: u32) -> TcpListener {
        let wasi_sock_addr = unsafe { sock_addr_local(fd) };
        let sock_addr = unsafe { to_socket_addr(&wasi_sock_addr).unwrap() };
        unsafe {
            TcpListener {
                fd: WasiFd::from_raw(fd),
                socket_address: sock_addr,
            }
        }
    }
}
*/

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpListener")
            .field("fd", &self.fd.as_raw())
            .finish()
    }
}

pub struct UdpSocket {
    fd: WasiFd,
}

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unimplemented!("bind")
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!("peer_addr")
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!("socket_addr")
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unimplemented!("recv_from")
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unimplemented!("peek_from")
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        unimplemented!("send_to")
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unimplemented!("duplicate")
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        //unimplemented!("set_read_timeout")
        Ok(())
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        //unimplemented!("set_write_timeout")
        Ok(())
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!("read_timeout")
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unimplemented!("write_timeout")
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_broadcast")
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        unimplemented!("broadcast")
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_multicast_loop_v4")
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        unimplemented!("multicast_loop_v4")
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        unimplemented!("set_multicast_ttl_v4")
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unimplemented!("multicast_ttl_v4")
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        unimplemented!("set_multicast_loop_v6")
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        unimplemented!("multicast_loop_v6")
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unimplemented!("join_multicast_v4")
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unimplemented!("join_multicast_v6")
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unimplemented!("leave_multicast_v4")
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unimplemented!("leave_multicast_v6")
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

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        unimplemented!("recv")
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unimplemented!("peek")
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        unimplemented!("send")
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        unimplemented!("connect")
    }

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }
}

/*
impl FromInner<u32> for UdpSocket {
    fn from_inner(fd: u32) -> UdpSocket {
        unsafe {
            UdpSocket {
                fd: WasiFd::from_raw(fd),
            }
        }
    }
}
*/

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpSocket")
            .field("fd", &self.fd.as_raw())
            .finish()
    }
}

pub struct LookupHost {
    //original: *mut c::addrinfo,
    //cur: *mut c::addrinfo,
    //original: *mut wasi_experimental_sockets::Addr,
    cur: *mut wasi_experimental_sockets::Addr,
    has_itered: bool,
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
        loop {
            unsafe {
                if !self.has_itered {
                    let cur = self.cur.as_ref()?;
                    self.has_itered = true;
                    match to_socket_addr(cur) {
                        //match sockaddr_to_addr(mem::transmute(cur.ai_addr), cur.ai_addrlen as usize) {
                        Ok(addr) => return Some(addr),
                        Err(_) => continue,
                    }
                } else {
                    return None;
                }
                //self.cur = cur.ai_next;
                /*match to_socket_addr(cur) {
                //match sockaddr_to_addr(mem::transmute(cur.ai_addr), cur.ai_addrlen as usize) {
                    Ok(addr) => return Some(addr),
                    Err(_) => continue,
                }*/
            }
        }
    }
}

//impl<'a> TryFrom<&'a str> for LookupHost {
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
        // let c_host = CString::new(host)?;
        // let mut hints: c::addrinfo = unsafe { mem::zeroed() };
        // hints.ai_socktype = c::SOCK_STREAM;
        // let mut res = ptr::null_mut();
        // unsafe {
        //     cvt_gai(c::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints, &mut res))
        //         .map(|_| LookupHost { original: res, cur: res, port })
        // }

        /*let wasi_addr = wasi_experimental_sockets::Addr {
            tag: wasi_experimental_sockets::ADDR_TYPE_IP4,
            u: wasi_experimental_sockets::AddrU {
                ip4: wasi_experimental_sockets::AddrIp4Port {
                    addr: wasi_experimental_sockets::AddrIp4 {
                        n0: 0,
                        n1: 0,
                        h0: 0,
                        h1: 0,
                    },
                    port: port,
                }
            },
        };*/

        let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
        //wasi_experimental_sockets::sock_addr_remote(fd,  buf.as_mut_ptr(), BUF_LEN).unwrap();
        let addr_ptr = buf.as_ptr() as *mut wasi_experimental_sockets::Addr;
        // TODO: handle error
        let _res = unsafe {
            wasi_experimental_sockets::addr_resolve(&host, port, buf.as_mut_ptr(), BUF_LEN).unwrap();
        };
        //let addr = addr_ptr.read_unaligned();

        //let ret = LookupHost{original: addr_ptr, cur: addr_ptr, port: port};
        let ret = LookupHost {
            cur: addr_ptr,
            has_itered: false,
            port: port,
        };
        return io::Result::Ok(ret);
        // pub unsafe fn addr_resolve(
        //     host: &str,
        //     port: IpPort,
        //     buf: *mut u8,
        //     buf_len: Size,
        // ) -> Result<Size> {
    }
}

/*
pub fn sockaddr_to_addr(storage: &c::sockaddr_storage, len: usize) -> io::Result<SocketAddr> {
    match storage.ss_family as c_int {
        c::AF_INET => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in>());
            Ok(SocketAddr::V4(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in)
            })))
        }
        c::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in6>());
            Ok(SocketAddr::V6(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in6)
            })))
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "invalid argument")),
    }
}
*/
#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_INET: u8 = 0;
    pub const AF_INET6: u8 = 1;
    pub type sa_family_t = u8;

    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: u16,
        pub sin_addr: in_addr,
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr {}

    pub type socklen_t = usize;
}

/*
impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = unsafe { wasi_experimental_sockets::sock_close(self.fd().as_raw()) };
    }
}*/
