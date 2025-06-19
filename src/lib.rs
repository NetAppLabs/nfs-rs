// Copyright 2025 NetApp Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

#[cfg(target_os = "wasi")]
#[allow(static_mut_refs,unused)]
mod bindings;
#[cfg(target_os = "wasi")]
mod component;
#[cfg(target_os = "wasi")]
#[allow(unused)]
mod wasi_ext;

#[cfg(target_os = "wasi")]
struct Component;

#[cfg(target_os = "wasi")]
pub struct NfsMount {
    id: u32,
}

#[cfg(target_os = "wasi")]
bindings::export!(Component with_types_in bindings);

#[cfg(target_os = "wasi")]
use component::{add_resource, get_resource, remove_resource};

#[cfg(not(target_os = "wasi"))]
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
#[cfg(target_os = "wasi")]
use wasi_ext::{SocketAddr, TcpStream, ToSocketAddrs};

static SOCKET_ADDRESSES: LazyLock<Arc<RwLock<HashMap<u32, SocketAddr>>>> = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));
static TCP_STREAMS: LazyLock<Arc<RwLock<HashMap<u32, Arc<RwLock<TcpStream>>>>>> = LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

fn get_socket_address(id: &u32) -> Result<SocketAddr> {
    let socket_addresses = SOCKET_ADDRESSES.read().unwrap();
    let socket_address = socket_addresses.get(id);
    match socket_address {
        Some(addr) => Ok(addr.clone()),
        _ => Err(Error::new(ErrorKind::NotFound, "socket address not found")),
    }
}

fn using_tcp_stream<T>(id: &u32, func: fn(&mut TcpStream) -> Result<T>) -> Result<T> {
    let tcp_streams = TCP_STREAMS.read().unwrap();
    let tcp_stream = tcp_streams.get(id);
    match tcp_stream {
        Some(stream) => func(&mut stream.write().unwrap()),
        _ => Err(Error::new(ErrorKind::NotFound, "TCP stream not found")),
    }
}

fn using_tcp_stream_with_buffer<T>(
    id: &u32,
    buf: &Vec<u8>,
    func: fn(&mut TcpStream, &Vec<u8>) -> Result<T>,
) -> Result<T> {
    let tcp_streams = TCP_STREAMS.read().unwrap();
    let tcp_stream = tcp_streams.get(id);
    match tcp_stream {
        Some(stream) => func(&mut stream.write().unwrap(), buf),
        _ => Err(Error::new(ErrorKind::NotFound, "TCP stream not found")),
    }
}

fn add_tcp_stream(addr: &SocketAddr, stream: TcpStream) -> Result<u32> {
    let mut tcp_streams = TCP_STREAMS.write().unwrap();
    let mut socket_addresses = SOCKET_ADDRESSES.write().unwrap();
    let mut id: u32 = rand::random();
    while id == 0 || tcp_streams.contains_key(&id) {
        id = rand::random();
    }
    tcp_streams.insert(id, Arc::new(RwLock::new(stream)));
    socket_addresses.insert(id, addr.clone());
    Ok(id)
}

fn replace_tcp_stream(id: u32, stream: TcpStream) -> Result<()> {
    let mut tcp_streams = TCP_STREAMS.write().unwrap();
    tcp_streams.insert(id, Arc::new(RwLock::new(stream)));
    Ok(())
}

fn remove_tcp_stream(id: &u32) {
    let mut tcp_streams = TCP_STREAMS.write().unwrap();
    let mut socket_addresses = SOCKET_ADDRESSES.write().unwrap();
    tcp_streams.remove(id);
    socket_addresses.remove(id);
}

fn connect_tcp_stream(addr: &SocketAddr) -> Result<u32> {
    let stream = TcpStream::connect(addr)?;
    add_tcp_stream(addr, stream)
}

fn reconnect_tcp_stream(id: u32) -> Result<()> {
    let addr = get_socket_address(&id)?;
    let stream = TcpStream::connect(&addr)?;
    replace_tcp_stream(id, stream)
}

mod mount;
mod nfs3;
mod nfs4p1;
mod rpc;
mod shared;

pub use mount::{Attr, Mount, NFSVersion, ObjRes, Pathconf, ReaddirEntry, ReaddirplusEntry};
pub use shared::Time;
pub use std::io::Error;

use rpc::auth::{Auth, AuthFlavor};
use std::collections::HashMap;
use std::io::{ErrorKind, Result};
use std::sync::{Arc, LazyLock, RwLock};
use url::Url;

#[derive(Debug)]
struct MountArgs {
    versions: Vec<NFSVersion>,
    host: String,
    dirpath: String,
    mountport: u16,
    nfsport: u16,
    uid: u32,
    gid: u32,
    dircount: u32,
    maxcount: u32,
    rsize: u32,
    wsize: u32,
}

/// Parses the specified URL and attempts to mount the relevant NFS export
///
/// # Example
///
/// ```no_run
/// let mount = nfs_rs::parse_url_and_mount("nfs://127.0.0.1/some/export?version=4.1,4,3").unwrap();
/// if let Some(res) = mount.create_path("nfs-rs.txt", 0o664).ok() {
///     let contents = "hello rust".as_bytes().to_vec();
///     let num_bytes_written = mount.write(&res.fh, 0, &contents).unwrap_or_default();
///     assert_eq!(num_bytes_written as usize, contents.len());
///     let bytes_read = mount.read(&res.fh, 0, 16).unwrap_or_default();
///     assert_eq!(&bytes_read, &contents);
/// }
/// ```
pub fn parse_url_and_mount(url: &str) -> Result<Box<dyn Mount>> {
    mount(parse_url(url)?)
}

fn get_uid_gid() -> (u32, u32) {
    #[cfg(not(unix))]
    let uid_gid = || (65534, 65534);
    #[cfg(unix)]
    let uid_gid = || unsafe { (nix::libc::getuid(), nix::libc::getgid()) };
    uid_gid()
}

fn parse_url(url: &str) -> Result<MountArgs> {
    let res = Url::parse_with_params(url, &[("version", "3"), ("readdir-buffer", "8192,8192")]);
    if res.is_err() {
        return Err(Error::new(ErrorKind::InvalidInput, res.unwrap_err()));
    }
    let mut parsed_url = res.unwrap();
    if parsed_url.scheme() != "nfs" {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "specified URL does not have scheme nfs",
        ));
    }
    if !parsed_url.has_host() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "specified URL does not contain a host",
        ));
    }
    let addr_port = parsed_url.port();
    let _ = parsed_url.set_port(None).unwrap();
    let params = parsed_url.query_pairs();
    let version_str = params
        .filter(|(name, _)| name == "version")
        .next()
        .unwrap()
        .1;
    let mut versions = Vec::new();
    for v in version_str.split(',') {
        let version: NFSVersion = v.into();
        match version {
            NFSVersion::Unknown => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "specified URL contains bad NFS version",
                ))
            }
            _ => versions.push(version),
        }
    }
    if versions.is_empty() {
        versions.push(NFSVersion::NFSv4p1);
        versions.push(NFSVersion::NFSv3);
    }
    let (uid_def, gid_def) = get_uid_gid();
    let uid = get_url_query_param(&params, "uid", uid_def, "specified URL contains bad UID")?;
    let gid = get_url_query_param(&params, "gid", gid_def, "specified URL contains bad GID")?;
    let readdir_buffer_str = params
        .filter(|(name, _)| name == "readdir-buffer")
        .next()
        .unwrap()
        .1;
    let (dircount, maxcount): (u32, u32) = parse_readdir_buffer_query_param(&readdir_buffer_str)?;
    let nfsport = get_url_query_param(
        &params,
        "nfsport",
        addr_port.unwrap_or_default(),
        "specified URL contains bad NFS port",
    )?;
    let mountport = get_url_query_param(
        &params,
        "mountport",
        Default::default(),
        "specified URL contains bad mount port",
    )?;
    let txsize_def: u32 = 1048576; // XXX: mimic libnfs default of 1 MiB
    let rsize = get_url_query_param(&params, "rsize", txsize_def, "specified URL contains bad max read size value")?;
    let wsize = get_url_query_param(&params, "wsize", txsize_def, "specified URL contains bad max write size value")?;
    let host = parsed_url.host_str().unwrap_or_default().to_string();
    Ok(MountArgs {
        versions,
        host,
        mountport,
        nfsport,
        dirpath: parsed_url.path().to_string(),
        uid,
        gid,
        dircount,
        maxcount,
        rsize,
        wsize,
    })
}

fn get_url_query_param<T: std::str::FromStr>(
    params: &url::form_urlencoded::Parse,
    name: &str,
    def: T,
    err_msg: &str,
) -> Result<T> {
    if let Some(val) = params.filter(|(n, _)| n == name).next() {
        val.1
            .parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, err_msg))
    } else {
        Ok(def)
    }
}

fn parse_readdir_buffer_query_param(param: &str) -> Result<(u32, u32)> {
    if let Some((dircount_str, maxcount_str)) = param.split_once(',') {
        let dircount: u32 = dircount_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidInput,
                "specified URL contains bad readdir-buffer value",
            )
        })?;
        let maxcount: u32 = maxcount_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidInput,
                "specified URL contains bad readdir-buffer value",
            )
        })?;
        Ok((dircount, maxcount))
    } else {
        let count: u32 = param.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidInput,
                "specified URL contains bad readdir-buffer value",
            )
        })?;
        Ok((count, count))
    }
}

fn mount(args: MountArgs) -> Result<Box<dyn Mount>> {
    let mut errs: Vec<Error> = Vec::new();
    for version in &args.versions {
        let res: Result<Box<dyn Mount>> = match version {
            NFSVersion::NFSv3 => nfs3::mount(&args),
            NFSVersion::NFSv4p1 => nfs4p1::mount(&args),
            NFSVersion::NFSv4 => Err(Error::new(ErrorKind::Unsupported, "NFSv4 is not supported")),
            NFSVersion::NFSv4p2 => Err(Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported")),
            _ => unreachable!(),
        };
        if res.is_ok() {
            return res;
        }
        errs.push(res.unwrap_err());
    }
    Err(squash_mount_errors(errs))
}

fn squash_mount_errors(errs: Vec<Error>) -> Error {
    let mut unsupported_err = "".to_string();
    let errs: Vec<Error> = errs.into_iter()
        .map(|err| {
            if err.kind() == ErrorKind::Unsupported {
                if unsupported_err.is_empty() {
                    unsupported_err = err.to_string();
                } else if unsupported_err != err.to_string() {
                    unsupported_err = "NFSv4 and NFSv4.2 are not supported".to_string();
                }
                None
            } else {
                Some(err)
            }
        })
        .flatten()
        .collect();
    if errs.len() == 0 {
        return Error::new(ErrorKind::Unsupported, unsupported_err);
    }
    let kind = errs[0].kind();
    let mut str = errs[0].to_string();
    let mut idx = 1;
    while idx < errs.len() {
        str = format!("{} - {}", str, errs[idx].to_string());
        idx += 1;
    }
    if unsupported_err.len() > 0 {
        str = format!("{} - {}", str, unsupported_err);
    }
    Error::new(kind, str)
}

fn split_path(path: &str) -> Result<(String, String)> {
    let cleaned = path_clean::clean(format!("/=/{}", path));
    if !cleaned.starts_with("/=/") {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid path specified"));
    }
    if cleaned.eq(std::path::Path::new("/=/")) {
        return Ok(("/".to_string(), "".to_string()));
    }
    let dir = cleaned
        .parent()
        .map(|x| x.to_string_lossy()[2..].to_string())
        .unwrap();
    let name = cleaned
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    Ok((dir, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_bad_scheme() {
        for scheme in ["ftp", "scp", "ssh"] {
            let res = parse_url(&format!("{}://localhost/some/export/path", scheme));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(err.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                err.to_string(),
                "specified URL does not have scheme nfs".to_string()
            );
        }
    }

    #[test]
    fn parse_url_missing_host() {
        let res = parse_url("nfs:///some/export/path");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL does not contain a host".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_version() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?version=5");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad NFS version".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_uid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?uid=nobody");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad UID".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_gid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?gid=wheel");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad GID".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_nfsport() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?nfsport=default");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad NFS port".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_mountport() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?mountport=nfsport");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad mount port".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_single_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=unlimited");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad readdir-buffer value".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_pair_first_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=unlimited,4096");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad readdir-buffer value".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_pair_second_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=4096,unlimited");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad readdir-buffer value".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_triple_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=2048,4096,8192");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad readdir-buffer value".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_rsize() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?rsize=sizable");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad max read size value".to_string()
        );
    }

    #[test]
    fn parse_url_with_bad_wsize() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?wsize=4mib");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "specified URL contains bad max write size value".to_string()
        );
    }

    #[test]
    fn parse_url_without_uid_and_gid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "127.0.0.1".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_uid_and_gid_and_multi_version() {
        let res = parse_url("nfs://localhost/some/export/path?version=4.1,4,3&uid=616&gid=666");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(
            args.versions,
            vec![NFSVersion::NFSv4p1, NFSVersion::NFSv4, NFSVersion::NFSv3]
        );
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), (616, 666));
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_port() {
        let res = parse_url("nfs://localhost:20490/some/export/path");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 20490);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_nfsport() {
        let res = parse_url("nfs://localhost/some/export/path?nfsport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 20490);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_mountport() {
        let res = parse_url("nfs://localhost/some/export/path?mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_port_and_mountport() {
        let res = parse_url("nfs://localhost:20389/some/export/path?mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 20389);
        assert_eq!(args.mountport, 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_nfsport_and_mountport() {
        let res = parse_url("nfs://localhost/some/export/path?nfsport=20389&mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 20389);
        assert_eq!(args.mountport, 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_port_nfsport_and_mountport() {
        let res = parse_url("nfs://localhost:20388/some/export/path?nfsport=20389&mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "localhost".to_string());
        assert_eq!(args.nfsport, 20389);
        assert_eq!(args.mountport, 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_rsize() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?rsize=16384");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "127.0.0.1".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (16384, 1048576));
    }

    #[test]
    fn parse_url_with_wsize() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?wsize=16384");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "127.0.0.1".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
        assert_eq!((args.rsize, args.wsize), (1048576, 16384));
    }

    #[test]
    fn parse_url_with_readdir_buffer_single_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=4096");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "127.0.0.1".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (4096, 4096));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn parse_url_with_readdir_buffer_pair_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=2048,4096");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec![NFSVersion::NFSv3]);
        assert_eq!(args.host, "127.0.0.1".to_string());
        assert_eq!(args.nfsport, 0);
        assert_eq!(args.mountport, 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (2048, 4096));
        assert_eq!((args.rsize, args.wsize), (1048576, 1048576));
    }

    #[test]
    fn mount_with_only_v4() {
        let args = MountArgs {
            versions: vec![NFSVersion::NFSv4],
            host: Default::default(),
            mountport: Default::default(),
            nfsport: Default::default(),
            dirpath: Default::default(),
            gid: Default::default(),
            uid: Default::default(),
            dircount: Default::default(),
            maxcount: Default::default(),
            rsize: Default::default(),
            wsize: Default::default(),
        };
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4 is not supported".to_string());
    }

    #[test]
    fn mount_with_only_v4_2() {
        let args = MountArgs {
            versions: vec![NFSVersion::NFSv4p2],
            host: Default::default(),
            mountport: Default::default(),
            nfsport: Default::default(),
            dirpath: Default::default(),
            gid: Default::default(),
            uid: Default::default(),
            dircount: Default::default(),
            maxcount: Default::default(),
            rsize: Default::default(),
            wsize: Default::default(),
        };
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4.2 is not supported".to_string());
    }

    #[test]
    fn mount_with_only_v4_and_v4_2() {
        let args = MountArgs {
            versions: vec![NFSVersion::NFSv4, NFSVersion::NFSv4p2],
            host: Default::default(),
            mountport: Default::default(),
            nfsport: Default::default(),
            dirpath: Default::default(),
            gid: Default::default(),
            uid: Default::default(),
            dircount: Default::default(),
            maxcount: Default::default(),
            rsize: Default::default(),
            wsize: Default::default(),
        };
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(
            err.to_string(),
            "NFSv4 and NFSv4.2 are not supported".to_string()
        );
    }

    #[test]
    fn squash_mount_errors_with_only_non_unsupported_err() {
        let errs = vec![
            Error::new(ErrorKind::Other, "some error"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error".to_string());
    }

    #[test]
    fn squash_mount_errors_with_only_non_unsupported_errs() {
        let errs = vec![
            Error::new(ErrorKind::Other, "some error"),
            Error::new(ErrorKind::InvalidInput, "some other error"),
            Error::new(ErrorKind::InvalidInput, "some final error"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error - some other error - some final error".to_string());
    }

    #[test]
    fn squash_mount_errors_with_only_unsupported_err() {
        let errs = vec![
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"),
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"), // XXX: same NFS version ignored
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4.2 is not supported".to_string());
    }

    #[test]
    fn squash_mount_errors_with_only_unsupported_errs() {
        let errs = vec![
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"),
            Error::new(ErrorKind::Unsupported, "NFSv4 is not supported"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4 and NFSv4.2 are not supported".to_string());
    }

    #[test]
    fn squash_mount_errors_with_unsupported_err_and_non_unsupported_err() {
        let errs = vec![
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"),
            Error::new(ErrorKind::Other, "some error"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error - NFSv4.2 is not supported".to_string());
    }

    #[test]
    fn squash_mount_errors_with_unsupported_errs_and_non_unsupported_err() {
        let errs = vec![
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"),
            Error::new(ErrorKind::Other, "some error"),
            Error::new(ErrorKind::Unsupported, "NFSv4 is not supported"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error - NFSv4 and NFSv4.2 are not supported".to_string());
    }

    #[test]
    fn squash_mount_errors_with_unsupported_err_and_non_unsupported_errs() {
        let errs = vec![
            Error::new(ErrorKind::Other, "some error"),
            Error::new(ErrorKind::Unsupported, "NFSv4 is not supported"),
            Error::new(ErrorKind::InvalidInput, "some other error"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error - some other error - NFSv4 is not supported".to_string());
    }

    #[test]
    fn squash_mount_errors_with_unsupported_errs_and_non_unsupported_errs() {
        let errs = vec![
            Error::new(ErrorKind::Other, "some error"),
            Error::new(ErrorKind::Unsupported, "NFSv4 is not supported"),
            Error::new(ErrorKind::InvalidInput, "some other error"),
            Error::new(ErrorKind::Unsupported, "NFSv4.2 is not supported"),
        ];
        let err = squash_mount_errors(errs);
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "some error - some other error - NFSv4 and NFSv4.2 are not supported".to_string());
    }

    #[test]
    fn split_path_empty_path() {
        let path = "";
        let res = split_path(path);
        assert!(res.is_ok());
        let (dir, name) = res.unwrap();
        assert_eq!(dir, "/".to_string());
        assert_eq!(name, "".to_string());
    }

    #[test]
    fn split_path_root_path() {
        let path = "/";
        let res = split_path(path);
        assert!(res.is_ok());
        let (dir, name) = res.unwrap();
        assert_eq!(dir, "/".to_string());
        assert_eq!(name, "".to_string());
    }

    #[test]
    fn split_path_sneaky_one() {
        let path = "..";
        let res = split_path(path);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "invalid path specified".to_string());
    }

    #[test]
    fn split_path_sneaky_two() {
        let path = "/first/../..";
        let res = split_path(path);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "invalid path specified".to_string());
    }

    #[test]
    fn split_path_path_depth_one() {
        let path = "/first/place/";
        let res = split_path(path);
        assert!(res.is_ok());
        let (dir, name) = res.unwrap();
        assert_eq!(dir, "/first".to_string());
        assert_eq!(name, "place".to_string());
    }

    #[test]
    fn split_path_path_depth_two() {
        let path = "/first/place/1999.txt";
        let res = split_path(path);
        assert!(res.is_ok());
        let (dir, name) = res.unwrap();
        assert_eq!(dir, "/first/place".to_string());
        assert_eq!(name, "1999.txt".to_string());
    }

    #[ignore]
    #[test]
    fn nfs3_works() {
        // this unit test was written to verify that the RPC communication was working correctly
        // it has been run against a go-nfs server that was serving a mount created via below shell script:
        /*
        #!/bin/bash

        set -e

        NFS_BASE=/Users/Shared/nfs
        NFS_UID=$1
        NFS_GID=$2
        if [ -z $NFS_UID ]; then
            NFS_UID=nobody
        fi
        if [ -z $NFS_GID ]; then
            NFS_GID=nogroup
        fi

        mkdir -p $NFS_BASE/first $NFS_BASE/quatre
        echo -n "In order to make sure that this file is exactly 123 bytes in size, I have written this text while watching its chars count." > $NFS_BASE/annar
        touch $NFS_BASE/3 $NFS_BASE/first/comment $NFS_BASE/quatre/points
        chmod 555 $NFS_BASE/quatre
        chmod 775 $NFS_BASE/first
        chmod 664 $NFS_BASE/annar
        chmod 444 $NFS_BASE/3
        chown -R $NFS_UID:$NFS_GID $NFS_BASE
        */
        let mount_result =
            parse_url_and_mount("nfs://localhost/Users/Shared/nfs/?nfsport=20490&mountport=20490");
        assert!(mount_result.is_ok(), "err = {}", mount_result.unwrap_err());
        let mount = mount_result.unwrap();
        let res = mount.access_path("/3", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let three_access = res.unwrap();
        assert_eq!(three_access, 1 | 2 | 4 | 8 | 16 | 32); // XXX: since /3 has access 444, shouldn't response have access 1|2|32?
        let res = mount.access_path("/annar", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let annar_access = res.unwrap();
        assert_eq!(annar_access, 1 | 2 | 4 | 8 | 16 | 32);
        let res = mount.access_path("/first", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let first_access = res.unwrap();
        assert_eq!(first_access, 1 | 2 | 4 | 8 | 16 | 32);
        let res = mount.access_path("/quatre", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_access = res.unwrap();
        assert_eq!(quatre_access, 1 | 2 | 4 | 8 | 16 | 32); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
        let res = mount.access_path("/quatre/points", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_points_access = res.unwrap();
        assert_eq!(quatre_points_access, 1 | 2 | 4 | 8 | 16 | 32); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut initial_names = Vec::new();
        for entry in res.unwrap() {
            initial_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut initial_names_plus = Vec::new();
        for entry in res.unwrap() {
            if entry.file_name == "." || entry.file_name == ".." {
                assert!(entry.attr.is_none());
            } else {
                assert!(entry.attr.is_some());
            }
            initial_names_plus.push(entry.file_name);
        }
        let expected_initial_names = vec![".".to_string(), "..".to_string(), "comment".to_string()];
        assert_eq!(initial_names, expected_initial_names);
        assert_eq!(initial_names_plus, expected_initial_names);
        let res = mount.mkdir_path("/first/place", 0o775);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_mkdir_names = Vec::new();
        for entry in res.unwrap() {
            post_mkdir_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_mkdir_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_mkdir_names_plus.push(entry.file_name);
        }
        let expected_post_mkdir_names = vec![
            ".".to_string(),
            "..".to_string(),
            "comment".to_string(),
            "place".to_string(),
        ];
        assert_eq!(post_mkdir_names, expected_post_mkdir_names);
        assert_eq!(post_mkdir_names_plus, expected_post_mkdir_names);
        let mut expected_post_create_names = vec![".".to_string(), "..".to_string()];
        for i in 0..100 {
            let name = format!("19{:02}.txt", i);
            let res = mount.create_path(&format!("/first/place/{}", name), 0o664);
            assert!(res.is_ok(), "err = {}", res.unwrap_err());
            expected_post_create_names.push(name);
        }
        let res = mount.readdir_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_create_names = Vec::new();
        for entry in res.unwrap() {
            post_create_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_create_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_create_names_plus.push(entry.file_name);
        }
        post_create_names.sort();
        post_create_names_plus.sort();
        expected_post_create_names.sort();
        assert_eq!(post_create_names, expected_post_create_names);
        assert_eq!(post_create_names_plus, expected_post_create_names);
        for name in expected_post_create_names {
            if name != "." && name != ".." {
                let res = mount.remove_path(&format!("/first/place/{}", name));
                assert!(res.is_ok(), "err = {}", res.unwrap_err());
            }
        }
        let expected_post_remove_names = vec![".".to_string(), "..".to_string()];
        let res = mount.readdir_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_remove_names = Vec::new();
        for entry in res.unwrap() {
            post_remove_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_remove_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_remove_names_plus.push(entry.file_name);
        }
        assert_eq!(post_remove_names, expected_post_remove_names);
        assert_eq!(post_remove_names_plus, expected_post_remove_names);
        let res = mount.rmdir_path("/first/place");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_rmdir_names = Vec::new();
        for entry in res.unwrap() {
            post_rmdir_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_rmdir_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_rmdir_names_plus.push(entry.file_name);
        }
        assert_eq!(post_rmdir_names, expected_initial_names);
        assert_eq!(post_rmdir_names_plus, expected_initial_names);
        let res = mount.create_path("/pleading-the-fifth", 0o664);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let data = "On my lawyer's council, I plead the fifth"
            .as_bytes()
            .to_vec();
        let res = mount.write_path("/pleading-the-fifth", 0, &data);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        assert_eq!(res.unwrap(), data.len() as u32);
        let res = mount.commit_path("/pleading-the-fifth", 0, 0);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.read_path("/pleading-the-fifth", 0, 256);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let read_data = res.unwrap();
        assert_eq!(&read_data, &data);
        let res = mount.getattr_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let attrs = res.unwrap();
        assert_eq!(attrs.filesize, data.len() as u64);
        assert_eq!(attrs.file_mode, 0o664);
        let res = mount.pathconf_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let pathconf = res.unwrap();
        assert!(pathconf.attr.is_some());
        assert!(pathconf.no_trunc);
        assert!(pathconf.case_preserving);
        assert!(!pathconf.case_insensitive);
        assert!(!pathconf.chown_restricted);
        assert_eq!(pathconf.linkmax, 1);
        assert_eq!(pathconf.name_max, 255);
        let res = mount.setattr_path(
            "/pleading-the-fifth",
            true,
            Some(0o666),
            None,
            None,
            Some(attrs.filesize / 2),
            None,
            None,
        );
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.getattr_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let post_set_attrs = res.unwrap();
        assert_eq!(post_set_attrs.filesize, (data.len() / 2) as u64);
        assert_eq!(post_set_attrs.file_mode, 0o666);
        let res = mount.rename_path("/pleading-the-fifth", "/first/time-testifying");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.rename_path("/first/time-testifying", "/./first/./cross-examination");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.symlink_path("/first/cross-examination", "/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readlink_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let readlink_value = res.unwrap();
        assert_eq!(readlink_value, "/first/cross-examination".to_string());
        let res = mount.remove_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.remove_path("/first/cross-examination");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.null();
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.umount();
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
    }

    #[ignore]
    #[test]
    fn nfs4p1_works() {
        // this unit test was written to verify that the RPC communication was working correctly
        // it has been run against an NFSv4.1 server that was serving a mount created via below shell script:
        /*
        #!/bin/bash

        set -e

        NFS_BASE=/srv/nfs4share
        NFS_UID=$1
        NFS_GID=$2
        if [ -z $NFS_UID ]; then
            NFS_UID=nobody
        fi
        if [ -z $NFS_GID ]; then
            NFS_GID=nogroup
        fi

        mkdir -p $NFS_BASE/first $NFS_BASE/quatre
        echo -n "In order to make sure that this file is exactly 123 bytes in size, I have written this text while watching its chars count." > $NFS_BASE/annar
        touch $NFS_BASE/3 $NFS_BASE/first/comment $NFS_BASE/quatre/points
        chmod 555 $NFS_BASE/quatre
        chmod 775 $NFS_BASE/first
        chmod 664 $NFS_BASE/annar
        chmod 444 $NFS_BASE/3
        chown -R $NFS_UID:$NFS_GID $NFS_BASE
        */
        let mount_result =
            parse_url_and_mount("nfs://192.168.64.2/srv/nfs4share/?version=4.1&uid=0&gid=0");
        assert!(mount_result.is_ok(), "err = {}", mount_result.unwrap_err());
        let mount = mount_result.unwrap();
        let res = mount.access_path("/3", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let three_access = res.unwrap();
        assert_eq!(three_access, 1 | 4 | 8); // XXX: since /3 has access 444, shouldn't response have access 1|2|32?
        let res = mount.access_path("/annar", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let annar_access = res.unwrap();
        assert_eq!(annar_access, 1 | 4 | 8);
        let res = mount.access_path("/first", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let first_access = res.unwrap();
        assert_eq!(first_access, 1 | 2 | 4 | 8 | 16);
        let res = mount.access_path("/quatre", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_access = res.unwrap();
        assert_eq!(quatre_access, 1 | 2 | 4 | 8 | 16); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
        let res = mount.access_path("/quatre/points", 1 | 2 | 4 | 8 | 16 | 32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_points_access = res.unwrap();
        assert_eq!(quatre_points_access, 1 | 4 | 8); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut initial_names = Vec::new();
        for entry in res.unwrap() {
            initial_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut initial_names_plus = Vec::new();
        for entry in res.unwrap() {
            if entry.file_name == "." || entry.file_name == ".." {
                assert!(entry.attr.is_none());
            } else {
                assert!(entry.attr.is_some());
            }
            initial_names_plus.push(entry.file_name);
        }
        let expected_initial_names = vec!["comment".to_string()];
        assert_eq!(initial_names, expected_initial_names);
        assert_eq!(initial_names_plus, expected_initial_names);
        let res = mount.mkdir_path("/first/place", 0o775);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_mkdir_names = Vec::new();
        for entry in res.unwrap() {
            post_mkdir_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_mkdir_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_mkdir_names_plus.push(entry.file_name);
        }
        let expected_post_mkdir_names = vec![
            "comment".to_string(),
            "place".to_string(),
        ];
        assert_eq!(post_mkdir_names, expected_post_mkdir_names);
        assert_eq!(post_mkdir_names_plus, expected_post_mkdir_names);
        let mut expected_post_create_names = Vec::new();
        for i in 0..100 {
            let name = format!("19{:02}.txt", i);
            let res = mount.create_path(&format!("/first/place/{}", name), 0o664);
            assert!(res.is_ok(), "err = {}", res.unwrap_err());
            expected_post_create_names.push(name);
        }
        let res = mount.readdir_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_create_names = Vec::new();
        for entry in res.unwrap() {
            post_create_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_create_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_create_names_plus.push(entry.file_name);
        }
        post_create_names.sort();
        post_create_names_plus.sort();
        expected_post_create_names.sort();
        assert_eq!(post_create_names, expected_post_create_names);
        assert_eq!(post_create_names_plus, expected_post_create_names);
        for name in expected_post_create_names {
            if name != "." && name != ".." {
                let res = mount.remove_path(&format!("/first/place/{}", name));
                assert!(res.is_ok(), "err = {}", res.unwrap_err());
            }
        }
        let expected_post_remove_names: Vec<String> = Vec::new();
        let res = mount.readdir_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_remove_names = Vec::new();
        for entry in res.unwrap() {
            post_remove_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/place/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_remove_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_remove_names_plus.push(entry.file_name);
        }
        assert_eq!(post_remove_names, expected_post_remove_names);
        assert_eq!(post_remove_names_plus, expected_post_remove_names);
        let res = mount.rmdir_path("/first/place");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readdir_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_rmdir_names = Vec::new();
        for entry in res.unwrap() {
            post_rmdir_names.push(entry.file_name);
        }
        let res = mount.readdirplus_path("/first/");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let mut post_rmdir_names_plus = Vec::new();
        for entry in res.unwrap() {
            post_rmdir_names_plus.push(entry.file_name);
        }
        assert_eq!(post_rmdir_names, expected_initial_names);
        assert_eq!(post_rmdir_names_plus, expected_initial_names);
        let res = mount.create_path("/pleading-the-fifth", 0o664);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let data = "On my lawyer's council, I plead the fifth"
            .as_bytes()
            .to_vec();
        let res = mount.write_path("/pleading-the-fifth", 0, &data);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        assert_eq!(res.unwrap(), data.len() as u32);
        let res = mount.commit_path("/pleading-the-fifth", 0, 0);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.read_path("/pleading-the-fifth", 0, 256);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let read_data = res.unwrap();
        assert_eq!(&read_data, &data);
        let res = mount.getattr_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let attrs = res.unwrap();
        assert_eq!(attrs.filesize, data.len() as u64);
        assert_eq!(attrs.file_mode, 0o664);
        assert_eq!(attrs.uid, 0);
        assert_eq!(attrs.gid, 0);
        let res = mount.pathconf_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let pathconf = res.unwrap();
        assert!(pathconf.attr.is_some());
        assert!(pathconf.no_trunc);
        assert!(pathconf.case_preserving);
        assert!(!pathconf.case_insensitive);
        assert!(pathconf.chown_restricted);
        assert_eq!(pathconf.linkmax, 255);
        assert_eq!(pathconf.name_max, 255);
        let res = mount.setattr_path(
            "/pleading-the-fifth",
            true,
            Some(0o666),
            Some(65534),
            Some(65534),
            Some(attrs.filesize / 2),
            None,
            None,
        );
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.getattr_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let post_set_attrs = res.unwrap();
        assert_eq!(post_set_attrs.filesize, (data.len() / 2) as u64);
        assert_eq!(post_set_attrs.file_mode, 0o666);
        assert_eq!(post_set_attrs.uid, 65534);
        assert_eq!(post_set_attrs.gid, 65534);
        let res = mount.rename_path("/pleading-the-fifth", "/first/time-testifying");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.rename_path("/first/time-testifying", "/./first/./cross-examination");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.symlink_path("/first/cross-examination", "/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.readlink_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let readlink_value = res.unwrap();
        assert_eq!(readlink_value, "/first/cross-examination".to_string());
        let res = mount.remove_path("/pleading-the-fifth");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.remove_path("/first/cross-examination");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let res = mount.null();
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        // TODO: implement umount for NFSv4.1
        // let res = mount.umount();
        // assert!(res.is_ok(), "err = {}", res.unwrap_err());
    }
}
