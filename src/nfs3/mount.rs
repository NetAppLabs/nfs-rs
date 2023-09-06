
#[cfg(target_os = "wasi")]
use crate::wasi_ext::TcpStream;
#[cfg(not(target_os = "wasi"))]
use std::net::TcpStream;

use xdr_codec::{Pack, Unpack};
use super::{Mount, Error, ErrorKind, Result, MOUNT3args, Time};
use super::mount3xdr::{dirpath, mountres3};
use crate::{nfs3, rpc};

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
    fn null(&self) -> Result<()> {
        self.m.null()
    }

    // fn fsinfo(&self) -> Result<nfs3::FSInfo> {
    //     self.m.fsinfo()
    // }

    // fn fsstat(&self) -> Result<nfs3::FSStat> {
    //     self.m.fsstat()
    // }

    fn access(&self, fh: &Vec<u8>, mode: u32) -> Result<u32> {
        self.m.access(fh, mode)
    }

    fn access_path(&self, path: &str, mode: u32) -> Result<u32> {
        self.m.access_path(path, mode)
    }

    fn close(&self, _seqid: u32, _stateid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        self.m.commit(fh, offset, count)
    }

    fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.m.commit_path(path, offset, count)
    }

    fn create(&self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<Vec<u8>> {
        self.m.create(dir_fh, filename, mode)
    }

    fn create_path(&self, path: &str, mode: u32) -> Result<Vec<u8>> {
        self.m.create_path(path, mode)
    }

    fn delegpurge(&self, _clientid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn delegreturn(&self, _stateid: u64) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn getattr(&self, fh: &Vec<u8>) -> Result<crate::mount::Attr> {
        self.m.getattr(fh).map(|res| res.into())
    }

    fn getattr_path(&self, path: &str) -> Result<crate::mount::Attr> {
        self.m.getattr_path(path).map(|res| res.into())
    }

    fn setattr(&self, fh: &Vec<u8>, guard_ctime: Option<Time>, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        self.m.setattr(fh, guard_ctime, mode, uid, gid, size, atime, mtime)
    }

    fn setattr_path(&self, path: &str, specify_guard: bool, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        self.m.setattr_path(path, specify_guard, mode, uid, gid, size, atime, mtime)
    }

    fn getfh(&self) -> Result<()> {
        Err(Error::new(ErrorKind::Unsupported, "not supported"))
    }

    fn link(&self, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<crate::mount::Attr> {
        self.m.link(src_fh, dst_dir_fh, dst_filename).map(|res| res.into())
    }

    fn link_path(&self, src_path: &str, dst_path: &str) -> Result<crate::mount::Attr> {
        self.m.link_path(src_path, dst_path).map(|res| res.into())
    }

    fn symlink(&self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<Vec<u8>> {
        self.m.symlink(src_path, dst_dir_fh, dst_filename)
    }

    fn symlink_path(&self, src_path: &str, dst_path: &str) -> Result<Vec<u8>> {
        self.m.symlink_path(src_path, dst_path)
    }

    fn readlink(&self, fh: &Vec<u8>) -> Result<String> {
        self.m.readlink(fh)
    }

    fn readlink_path(&self, path: &str) -> Result<String> {
        self.m.readlink_path(path)
    }

    fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<Vec<u8>> {
        self.m.lookup(dir_fh, filename)
    }

    fn lookup_path(&self, path: &str) -> Result<Vec<u8>> {
        self.m.lookup_path(path)
    }

    fn pathconf(&self, fh: &Vec<u8>) -> Result<crate::mount::Pathconf> {
        self.m.pathconf(fh).map(|res| res.into())
    }

    fn pathconf_path(&self, path: &str) -> Result<crate::mount::Pathconf> {
        self.m.pathconf_path(path).map(|res| res.into())
    }

    fn read(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.m.read(fh, offset, count)
    }

    fn read_path(&self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.m.read_path(path, offset, count)
    }

    fn write(&self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.m.write(fh, offset, data)
    }

    fn write_path(&self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.m.write_path(path, offset, data)
    }

    fn readdir(&self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirEntry>> {
        Ok(self.m.readdir(dir_fh)?.iter().map(|e| e.into()).collect())
    }

    fn readdir_path(&self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirEntry>> {
        Ok(self.m.readdir_path(dir_path)?.iter().map(|e| e.into()).collect())
    }

    fn readdirplus(&self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        Ok(self.m.readdirplus(dir_fh)?.iter().map(|e| e.into()).collect())
    }

    fn readdirplus_path(&self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        Ok(self.m.readdirplus_path(dir_path)?.iter().map(|e| e.into()).collect())
    }

    fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<Vec<u8>> {
        self.m.mkdir(dir_fh, dirname, mode)
    }

    fn mkdir_path(&self, path: &str, mode: u32) -> Result<Vec<u8>> {
        self.m.mkdir_path(path, mode)
    }

    fn remove(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<()> {
        self.m.remove(dir_fh, filename)
    }

    fn remove_path(&self, path: &str) -> Result<()> {
        self.m.remove_path(path)
    }

    fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        self.m.rmdir(dir_fh, dirname)
    }

    fn rmdir_path(&self, path: &str) -> Result<()> {
        self.m.rmdir_path(path)
    }

    fn rename(&self, from_dir_fh: &Vec<u8>, from_filename: &str, to_dir_fh: &Vec<u8>, to_filename: &str) -> Result<()> {
        self.m.rename(from_dir_fh, from_filename, to_dir_fh, to_filename)
    }

    fn rename_path(&self, from_path: &str, to_path: &str) -> Result<()> {
        self.m.rename_path(from_path, to_path)
    }

    fn umount(&self) -> Result<()> {
        self.m.umount()
    }
}

fn ensure_port(host: &String, port: u16, prog: u32, vers: u32, auth: &crate::Auth) -> Result<u16> {
    if port != 0 {
        return Ok(port);
    }
    rpc::portmap(host, prog, vers, auth)
}

pub(crate) fn mount(args: crate::MountArgs) -> Result<Box<dyn crate::Mount>> {
    let dir = args.dirpath;
    let (dircount, maxcount) = (args.dircount, args.maxcount);
    let auth = crate::Auth::new_unix("nfs-rs", args.uid, args.gid);
    let nfsport = ensure_port(&args.host, args.nfsport, rpc::NFS3_PROG, rpc::NFS3_VERSION, &auth)?;
    let mountport = ensure_port(&args.host, args.mountport, rpc::MOUNT3_PROG, rpc::MOUNT3_VERSION, &auth)?;
    let nfs_conn = TcpStream::connect((args.host.as_str(), nfsport))?;
    let nfs_addr = nfs_conn.peer_addr()?;
    let mount_conn = if mountport != nfs_addr.port() {
        TcpStream::connect((args.host.as_str(), mountport))?
    } else {
        nfs_conn.try_clone()?
    };
    let client = rpc::Client::new(Some(nfs_conn), Some(mount_conn));

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

    let m = Mount{rpc: client, auth, fh: res.fhandle.0, dir, dircount, maxcount};
    let _ = m.null()?;
    let _ = m.fsinfo()?; // XXX: use returned FS info for something? github.com/sahlberg/libnfs must be calling this for something...

    Ok(Box::new(Mount3{m}))
}
