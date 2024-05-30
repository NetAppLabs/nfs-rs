
use std::io;

use xdr_codec::{Pack, Unpack};
use super::{Mount, Error, ErrorKind, Result, MOUNT3args, Time, ObjRes};
use super::mount3xdr::{dirpath, mountres3};
use crate::{SocketAddr, TcpStream, ToSocketAddrs, nfs3, rpc};

// const MNT_PATH_LEN: u32 = 1024;

// enum MountStat3 {
//     Mnt3OK = 0,
//     Mnt3ErrPerm = 1,
//     Mnt3ErrNoent = 2,
//     Mnt3ErrIo = 5,
//     Mnt3ErrAccess = 13,
//     Mnt3ErrNotdir = 20,
//     Mnt3ErrInval = 22,
//     Mnt3ErrNametoolong = 63,
//     Mnt3ErrNotsupp = 10004,
//     Mnt3ErrServerfault = 10006,
// }

#[derive(Debug)]
struct Mount3 {
    m: Mount,
}

impl crate::Mount for Mount3 {
    fn null(&mut self) -> Result<()> {
        self.m.null()
    }

    // fn fsinfo(&self) -> Result<nfs3::FSInfo> {
    //     self.m.fsinfo()
    // }

    // fn fsstat(&self) -> Result<nfs3::FSStat> {
    //     self.m.fsstat()
    // }

    fn access(&mut self, fh: &Vec<u8>, mode: u32) -> Result<u32> {
        self.m.access(fh, mode)
    }

    fn access_path(&mut self, path: &str, mode: u32) -> Result<u32> {
        self.m.access_path(path, mode)
    }

    fn close(&mut self, _seqid: u32, _stateid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn commit(&mut self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        self.m.commit(fh, offset, count)
    }

    fn commit_path(&mut self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.m.commit_path(path, offset, count)
    }

    fn create(&mut self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<ObjRes> {
        self.m.create(dir_fh, filename, mode)
    }

    fn create_path(&mut self, path: &str, mode: u32) -> Result<ObjRes> {
        self.m.create_path(path, mode)
    }

    fn delegpurge(&mut self, _clientid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn delegreturn(&mut self, _stateid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn getattr(&mut self, fh: &Vec<u8>) -> Result<crate::mount::Attr> {
        self.m.getattr(fh).map(|res| res.into())
    }

    fn getattr_path(&mut self, path: &str) -> Result<crate::mount::Attr> {
        self.m.getattr_path(path).map(|res| res.into())
    }

    fn setattr(&mut self, fh: &Vec<u8>, guard_ctime: Option<Time>, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        self.m.setattr(fh, guard_ctime, mode, uid, gid, size, atime, mtime)
    }

    fn setattr_path(&mut self, path: &str, specify_guard: bool, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        self.m.setattr_path(path, specify_guard, mode, uid, gid, size, atime, mtime)
    }

    fn getfh(&mut self) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn link(&mut self, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<crate::mount::Attr> {
        self.m.link(src_fh, dst_dir_fh, dst_filename).map(|res| res.into())
    }

    fn link_path(&mut self, src_path: &str, dst_path: &str) -> Result<crate::mount::Attr> {
        self.m.link_path(src_path, dst_path).map(|res| res.into())
    }

    fn symlink(&mut self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<ObjRes> {
        self.m.symlink(src_path, dst_dir_fh, dst_filename)
    }

    fn symlink_path(&mut self, src_path: &str, dst_path: &str) -> Result<ObjRes> {
        self.m.symlink_path(src_path, dst_path)
    }

    fn readlink(&mut self, fh: &Vec<u8>) -> Result<String> {
        self.m.readlink(fh)
    }

    fn readlink_path(&mut self, path: &str) -> Result<String> {
        self.m.readlink_path(path)
    }

    fn lookup(&mut self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes> {
        self.m.lookup(dir_fh, filename)
    }

    fn lookup_path(&mut self, path: &str) -> Result<ObjRes> {
        self.m.lookup_path(path)
    }

    fn pathconf(&mut self, fh: &Vec<u8>) -> Result<crate::mount::Pathconf> {
        self.m.pathconf(fh).map(|res| res.into())
    }

    fn pathconf_path(&mut self, path: &str) -> Result<crate::mount::Pathconf> {
        self.m.pathconf_path(path).map(|res| res.into())
    }

    fn read(&mut self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.m.read(fh, offset, count)
    }

    fn read_path(&mut self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.m.read_path(path, offset, count)
    }

    fn write(&mut self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.m.write(fh, offset, data)
    }

    fn write_path(&mut self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.m.write_path(path, offset, data)
    }

    fn readdir(&mut self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirEntry>> {
        Ok(self.m.readdir(dir_fh)?.iter().map(|e| e.into()).collect())
    }

    fn readdir_path(&mut self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirEntry>> {
        Ok(self.m.readdir_path(dir_path)?.iter().map(|e| e.into()).collect())
    }

    fn readdirplus(&mut self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        Ok(self.m.readdirplus(dir_fh)?.iter().map(|e| e.into()).collect())
    }

    fn readdirplus_path(&mut self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        Ok(self.m.readdirplus_path(dir_path)?.iter().map(|e| e.into()).collect())
    }

    fn mkdir(&mut self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes> {
        self.m.mkdir(dir_fh, dirname, mode)
    }

    fn mkdir_path(&mut self, path: &str, mode: u32) -> Result<ObjRes> {
        self.m.mkdir_path(path, mode)
    }

    fn remove(&mut self, dir_fh: &Vec<u8>, filename: &str) -> Result<()> {
        self.m.remove(dir_fh, filename)
    }

    fn remove_path(&mut self, path: &str) -> Result<()> {
        self.m.remove_path(path)
    }

    fn rmdir(&mut self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        self.m.rmdir(dir_fh, dirname)
    }

    fn rmdir_path(&mut self, path: &str) -> Result<()> {
        self.m.rmdir_path(path)
    }

    fn rename(&mut self, from_dir_fh: &Vec<u8>, from_filename: &str, to_dir_fh: &Vec<u8>, to_filename: &str) -> Result<()> {
        self.m.rename(from_dir_fh, from_filename, to_dir_fh, to_filename)
    }

    fn rename_path(&mut self, from_path: &str, to_path: &str) -> Result<()> {
        self.m.rename_path(from_path, to_path)
    }

    fn umount(&mut self) -> Result<()> {
        self.m.umount()
    }
}

fn ensure_port(addrs: &Vec<SocketAddr>, port: u16, prog: u32, vers: u32, auth: &crate::Auth) -> Result<u16> {
    if port != 0 {
        return Ok(port);
    }
    rpc::portmap(addrs, prog, vers, auth)
}

pub(crate) fn mount(args: crate::MountArgs) -> Result<Box<dyn crate::Mount>> {
    // start by resolving host address and assigning portmapper port to each resolved address
    let addrs = (args.host.as_str(), rpc::PORTMAP_PORT).to_socket_addrs()?.collect();
    let auth = crate::Auth::new_unix("nfs-rs", args.uid, args.gid);
    let nfsport = ensure_port(&addrs, args.nfsport, rpc::NFS3_PROG, rpc::NFS3_VERSION, &auth)?;
    let mountport = ensure_port(&addrs, args.mountport, rpc::MOUNT3_PROG, rpc::MOUNT3_VERSION, &auth)?;
    for mut addr in addrs {
        addr.set_port(nfsport); // replace portmapper port with NFS port obtained above
        let res = mount_on_addr(&addr, &args, &auth, mountport);
        if res.is_ok() {
            return Ok(res.unwrap());
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "no valid socket address"))
}

fn mount_on_addr(addr: &SocketAddr, args: &crate::MountArgs, auth: &crate::Auth, mountport: u16) -> Result<Box<dyn crate::Mount>> {
    let nfs_conn = TcpStream::connect(addr)?;
    let nfs_addr = nfs_conn.peer_addr()?;
    let dir: String = args.dirpath.to_owned();
    let (dircount, maxcount) = (args.dircount, args.maxcount);
    let mount_conn = if mountport != nfs_addr.port() {
        let mut mount_addr = addr.clone();
        mount_addr.set_port(mountport);
        TcpStream::connect(&mount_addr)?
    } else {
        nfs_conn.try_clone()?
    };
    let mut client = rpc::Client::new(Some(nfs_conn), Some(mount_conn));

    let args = nfs3::rpc_header(rpc::MOUNT3_PROG, rpc::MOUNT3_VERSION, nfs3::MountProc3::Null as u32, &auth);
    let mut buf = Vec::<u8>::new();
    let res = args.pack(&mut buf);
    if res.is_err() {
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }
    let _ = client.call(buf)?;

    let args = MOUNT3args{
        header: nfs3::rpc_header(rpc::MOUNT3_PROG, rpc::MOUNT3_VERSION, nfs3::MountProc3::Mount as u32, &auth),
        dirpath: dirpath(dir.trim_end_matches('/').to_string()),
    };
    let mut buf = Vec::<u8>::new();
    let res = args.pack(&mut buf);
    if res.is_err() {
        return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
    }

    let res = client.call(buf)?;
    let mut r = res.as_slice();
    let x = mountres3::unpack(&mut r);
    if x.is_err() {
        return Err(Error::new(ErrorKind::Other, x.unwrap_err()));
    }

    let res = match x.unwrap().0 {
        mountres3::MNT3_OK(ok) => Ok(ok),
        mountres3::default(e) => Err(Error::new(ErrorKind::Other, e)),
    }?;

    let mut m = Mount{rpc: client, auth: auth.clone(), fh: res.fhandle.0, dir, dircount, maxcount};
    let _ = m.null()?;
    let _ = m.fsinfo()?; // XXX: use returned FS info for something? github.com/sahlberg/libnfs must be calling this for something...

    Ok(Box::new(Mount3{m}))
}
