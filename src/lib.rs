mod rpc;
mod nfs3;
mod mount;
mod shared;
#[allow(unused)]
mod wasi_ext;

pub use mount::Mount;
pub use shared::Time;
pub use std::io::Error;

use std::io::{Result, ErrorKind};
use url::Url;
use rpc::auth::Auth;

#[derive(Debug)]
struct MountArgs {
    versions: Vec<String>,
    nfs_addrs: Vec<std::net::SocketAddr>,
    mount_addrs: Vec<std::net::SocketAddr>,
    dirpath: String,
    uid: u32,
    gid: u32,
    dircount: u32,
    maxcount: u32,
}

/// Parses the specified URL and attempts to mount the relevant NFS export
pub fn parse_url_and_mount(url: &str) -> Result<Box<dyn Mount>> {
    mount(parse_url(url)?)
}

fn get_uid_gid() -> (u32, u32) {
    #[cfg(not(unix))]
    let uid_gid = || { (65534, 65534) };
    #[cfg(unix)]
    let uid_gid = || {
        let uid = nix::unistd::getuid();
        let gid = nix::unistd::getgid();
        (uid.into(), gid.into())
    };
    uid_gid()
}

fn parse_url(url: &str) -> Result<MountArgs> {
    let res = Url::parse_with_params(url, &[("version", "3"), ("readdir-buffer", "8192,8192")]);
    if res.is_err() {
        return Err(Error::new(ErrorKind::InvalidInput, res.unwrap_err()));
    }
    let mut parsed_url = res.unwrap();
    if parsed_url.scheme() != "nfs" {
        return Err(Error::new(ErrorKind::InvalidInput, "specified URL does not have scheme nfs"));
    }
    if !parsed_url.has_host() {
        return Err(Error::new(ErrorKind::InvalidInput, "specified URL does not contain a host"));
    }
    let addr_port = parsed_url.port();
    let _ = parsed_url.set_port(None).unwrap();
    let params = parsed_url.query_pairs();
    let version_str = params.filter(|(name, _)| name == "version").next().unwrap().1;
    let mut versions = Vec::new();
    for v in version_str.split(',') {
        match v {
            "3"|"4"|"4.1" => versions.push(v.to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "specified URL contains bad NFS version")),
        }
    }
    let (uid_def, gid_def) = get_uid_gid();
    let uid = get_url_query_param(&params, "uid", uid_def, "specified URL contains bad UID")?;
    let gid = get_url_query_param(&params, "gid", gid_def, "specified URL contains bad GID")?;
    let readdir_buffer_str = params.filter(|(name, _)| name == "readdir-buffer").next().unwrap().1;
    let (dircount, maxcount): (u32, u32) = parse_readdir_buffer_query_param(&readdir_buffer_str)?;
    let nfsport = get_url_query_param(&params, "nfsport", addr_port.unwrap_or_default(), "specified URL contains bad NFS port")?;
    let mountport = get_url_query_param(&params, "mountport", Default::default(), "specified URL contains bad mount port")?;
    let nfs_addrs = parsed_url.socket_addrs(|| Some(nfsport))?;
    let mount_addrs = parsed_url.socket_addrs(|| Some(mountport))?;
    Ok(MountArgs{versions, nfs_addrs, mount_addrs, dirpath: parsed_url.path().to_string(), uid, gid, dircount, maxcount})
}

fn get_url_query_param<T: std::str::FromStr>(params: &url::form_urlencoded::Parse, name: &str, def: T, err_msg: &str) -> Result<T> {
    if let Some(val) = params.filter(|(n, _)| n == name).next() {
        val.1.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, err_msg))
    } else {
        Ok(def)
    }
}

fn parse_readdir_buffer_query_param(param: &str) -> Result<(u32, u32)> {
    if let Some((dircount_str, maxcount_str)) = param.split_once(',') {
        let dircount: u32 = dircount_str.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "specified URL contains bad readdir-buffer value"))?;
        let maxcount: u32 = maxcount_str.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "specified URL contains bad readdir-buffer value"))?;
        Ok((dircount, maxcount))
    } else {
        let count: u32 = param.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "specified URL contains bad readdir-buffer value"))?;
        Ok((count, count))
    }
}

fn mount(args: MountArgs) -> Result<Box<dyn Mount>> {
    let mut v4requested = false;
    let mut v4_1requested = false;
    for version in &args.versions {
        match version.as_str() {
            "3" => return nfs3::mount(args),
            "4" => v4requested = true,
            "4.1" => v4_1requested = true,
            _ => unreachable!(),
        }
    }
    match (v4requested, v4_1requested) {
        (true, true) => Err(Error::new(ErrorKind::Unsupported, "NFSv4 and NFSv4.1 are not supported")),
        (true, false) => Err(Error::new(ErrorKind::Unsupported, "NFSv4 is not supported")),
        (false, true) => Err(Error::new(ErrorKind::Unsupported, "NFSv4.1 is not supported")),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_bad_scheme() {
        for scheme in ["ftp", "scp", "ssh"] {
            let res = parse_url(format!("{}://localhost/some/export/path", scheme).as_str());
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(err.kind(), ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "specified URL does not have scheme nfs".to_string());
        }
    }

    #[test]
    fn parse_url_missing_host() {
        let res = parse_url("nfs:///some/export/path");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL does not contain a host".to_string());
    }

    #[test]
    fn parse_url_with_bad_version() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?version=5");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad NFS version".to_string());
    }

    #[test]
    fn parse_url_with_bad_uid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?uid=nobody");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad UID".to_string());
    }

    #[test]
    fn parse_url_with_bad_gid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?gid=wheel");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad GID".to_string());
    }

    #[test]
    fn parse_url_with_bad_nfsport() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?nfsport=default");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad NFS port".to_string());
    }

    #[test]
    fn parse_url_with_bad_mountport() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?mountport=nfsport");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad mount port".to_string());
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_single_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=unlimited");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad readdir-buffer value".to_string());
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_pair_first_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=unlimited,4096");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad readdir-buffer value".to_string());
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_pair_second_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=4096,unlimited");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad readdir-buffer value".to_string());
    }

    #[test]
    fn parse_url_with_bad_readdir_buffer_triple_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=2048,4096,8192");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "specified URL contains bad readdir-buffer value".to_string());
    }

    #[test]
    fn parse_url_without_uid_and_gid() {
        let res = parse_url("nfs://127.0.0.1/some/export/path");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 1);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs.len(), 1);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_uid_and_gid_and_multi_version() {
        let res = parse_url("nfs://localhost/some/export/path?version=4.1,4,3&uid=616&gid=666");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["4.1".to_string(), "4".to_string(), "3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 0);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 0);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), (616, 666));
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_port() {
        let res = parse_url("nfs://localhost:20490/some/export/path");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 20490);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 20490);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_nfsport() {
        let res = parse_url("nfs://localhost/some/export/path?nfsport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 20490);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 20490);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_mountport() {
        let res = parse_url("nfs://localhost/some/export/path?mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 0);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 0);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 20490);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_port_and_mountport() {
        let res = parse_url("nfs://localhost:20389/some/export/path?mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 20389);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 20389);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 20490);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_nfsport_and_mountport() {
        let res = parse_url("nfs://localhost/some/export/path?nfsport=20389&mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 20389);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 20389);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 20490);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[ignore]
    #[test]
    fn parse_url_with_port_nfsport_and_mountport() {
        let res = parse_url("nfs://localhost:20388/some/export/path?nfsport=20389&mountport=20490");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 2);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 20389);
        assert_eq!(args.nfs_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[1].port(), 20389);
        assert_eq!(args.mount_addrs.len(), 2);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0,0,0,0,0,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 20490);
        assert_eq!(args.mount_addrs[1].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[1].port(), 20490);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (8192, 8192));
    }

    #[test]
    fn parse_url_with_readdir_buffer_single_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=4096");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 1);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs.len(), 1);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (4096, 4096));
    }

    #[test]
    fn parse_url_with_readdir_buffer_pair_value() {
        let res = parse_url("nfs://127.0.0.1/some/export/path?readdir-buffer=2048,4096");
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let args = res.unwrap();
        assert_eq!(args.versions, vec!["3".to_string()]);
        assert_eq!(args.nfs_addrs.len(), 1);
        assert_eq!(args.nfs_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.nfs_addrs[0].port(), 0);
        assert_eq!(args.mount_addrs.len(), 1);
        assert_eq!(args.mount_addrs[0].ip(), std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1)));
        assert_eq!(args.mount_addrs[0].port(), 0);
        assert_eq!(args.dirpath, "/some/export/path".to_string());
        assert_eq!((args.uid, args.gid), get_uid_gid());
        assert_eq!((args.dircount, args.maxcount), (2048, 4096));
    }

    #[test]
    fn mount_with_only_v4() {
        let args = MountArgs{versions: vec!["4".to_string()], nfs_addrs: Default::default(), mount_addrs: Default::default(), dirpath: Default::default(), gid: Default::default(), uid: Default::default(), dircount: Default::default(), maxcount: Default::default()};
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4 is not supported".to_string());
    }

    #[test]
    fn mount_with_only_v4_1() {
        let args = MountArgs{versions: vec!["4.1".to_string()], nfs_addrs: Default::default(), mount_addrs: Default::default(), dirpath: Default::default(), gid: Default::default(), uid: Default::default(), dircount: Default::default(), maxcount: Default::default()};
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4.1 is not supported".to_string());
    }

    #[test]
    fn mount_with_only_v4_and_v4_1() {
        let args = MountArgs{versions: vec!["4".to_string(), "4.1".to_string()], nfs_addrs: Default::default(), mount_addrs: Default::default(), dirpath: Default::default(), gid: Default::default(), uid: Default::default(), dircount: Default::default(), maxcount: Default::default()};
        let res = mount(args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "NFSv4 and NFSv4.1 are not supported".to_string());
    }

    #[ignore]
    #[test]
    fn nfs3_works() {
        let mount_result = parse_url_and_mount("nfs://127.0.0.1/Users/Shared/nfs/?nfsport=20490&mountport=20490");
        assert!(mount_result.is_ok(), "err = {}", mount_result.unwrap_err());
        let mount = mount_result.unwrap();
        // XXX: fsinfo removed from trait as it should (probably?) be internal only
        // let res = mount.fsinfo();
        // assert!(res.is_ok(), "err = {}", res.unwrap_err());
        // let fsinfo = res.unwrap(); // XXX: below assertions for fsinfo use values obtained from a call to fsinfo so don't assume these are actually expected/valid values
        // assert!(fsinfo.attr.is_some());
        // assert_eq!(fsinfo.dtpref, 8192);
        // assert_eq!(fsinfo.wtmult, 4096);
        // assert_eq!(fsinfo.wtpref, 1073741824);
        // assert_eq!(fsinfo.wtmax, 1073741824);
        // assert_eq!(fsinfo.rtmult, 4096);
        // assert_eq!(fsinfo.rtpref, 1073741824);
        // assert_eq!(fsinfo.rtmax, 1073741824);
        // assert_eq!(fsinfo.size, 4611686018427387904);
        // assert_eq!(fsinfo.properties, 27);
        // XXX: fsstat removed from trait as it should (probably?) be internal only
        // let res = mount.fsstat();
        // assert!(res.is_ok(), "err = {}", res.unwrap_err());
        // let fsstat = res.unwrap(); // XXX: below assertions for fsstat use values obtained from a call to fsstat so don't assume these are actually expected/valid values (also, fsstat returns dynamic values as opposed to fsinfo's static values)
        // assert!(fsstat.attr.is_some());
        // assert_eq!(fsstat.tbytes, 4611686018427387904);
        // assert_eq!(fsstat.fbytes, 4611686018427387904);
        // assert_eq!(fsstat.abytes, 4611686018427387904);
        // assert_eq!(fsstat.tfiles, 4611686018427387904);
        // assert_eq!(fsstat.ffiles, 4611686018427387904);
        // assert_eq!(fsstat.afiles, 4611686018427387904);
        // assert_eq!(fsstat.invarsec, 0);
        let res = mount.access_path("/3", 1|2|4|8|16|32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let three_access = res.unwrap();
        assert_eq!(three_access, 1|2|4|8|16|32); // XXX: since /3 has access 444, shouldn't response have access 1|2|32?
        let res = mount.access_path("/annar", 1|2|4|8|16|32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let annar_access = res.unwrap();
        assert_eq!(annar_access, 1|2|4|8|16|32);
        let res = mount.access_path("/first", 1|2|4|8|16|32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let first_access = res.unwrap();
        assert_eq!(first_access, 1|2|4|8|16|32);
        let res = mount.access_path("/quatre", 1|2|4|8|16|32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_access = res.unwrap();
        assert_eq!(quatre_access, 1|2|4|8|16|32); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
        let res = mount.access_path("/quatre/points", 1|2|4|8|16|32);
        assert!(res.is_ok(), "err = {}", res.unwrap_err());
        let quatre_points_access = res.unwrap();
        assert_eq!(quatre_points_access, 1|2|4|8|16|32); // XXX: since /quatre has access 555, shouldn't response have access 1|2|32?
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
        let expected_initial_names = vec![
            ".".to_string(),
            "..".to_string(),
            "comment".to_string(),
        ];
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
        let mut expected_post_create_names = vec![
            ".".to_string(),
            "..".to_string(),
        ];
        for i in 0..100 {
            let name = format!("19{:02}.txt", i);
            let res = mount.create_path(format!("/first/place/{}", name).as_str(), 0o664);
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
                let res = mount.remove_path(format!("/first/place/{}", name).as_str());
                assert!(res.is_ok(), "err = {}", res.unwrap_err());
            }
        }
        let expected_post_remove_names = vec![
            ".".to_string(),
            "..".to_string(),
        ];
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
        let data = "On my lawyer's council, I plead the fifth".as_bytes().to_vec();
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
        let res = mount.setattr_path("/pleading-the-fifth", true, Some(0o666), None, None, Some(attrs.filesize / 2), None, None);
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
        // FIXME: link not supported by go-nfs
        // let res = mount.link_path("/first/cross-examination", "/pleading-the-fifth");
        // assert!(res.is_ok(), "err = {}", res.unwrap_err());
        // let link_attrs = res.unwrap();
        // assert_eq!(link_attrs, crate::nfs3::Fattr::default()); // TODO: determine assertions to make (this one will fail)
        // let res = mount.remove_path("/pleading-the-fifth");
        // assert!(res.is_ok(), "err = {}", res.unwrap_err());
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
}
