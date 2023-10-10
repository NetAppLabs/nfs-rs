mod access;
mod commit;
mod create;
mod fsinfo;
mod fsstat;
mod getattr;
mod link;
mod lookup;
mod mkdir;
mod mknod;
mod mount;
mod null;
mod pathconf;
mod read;
mod readdir;
mod readdirplus;
mod readlink;
mod remove;
mod rename;
mod rmdir;
mod setattr;
mod symlink;
mod umount;
mod write;

pub(crate) use mount::mount;

use crate::{Auth, Time, Result, Error, ErrorKind, rpc};

enum MountProc3 {
    Null = 0,
    Mount = 1,
    // Dump = 2,
    Umount = 3,
    // UmountAll = 4,
    // Export = 5,
}

enum NFSProc3 {
    Null = 0,
    GetAttr = 1,
    SetAttr = 2,
    Lookup = 3,
    Access = 4,
    Readlink = 5,
    Read = 6,
    Write = 7,
    Create = 8,
    Mkdir = 9,
    Symlink = 10,
    Mknod = 11,
    Remove = 12,
    Rmdir = 13,
    Rename = 14,
    Link = 15,
    Readdir = 16,
    Readdirplus = 17,
    FSStat = 18,
    FSInfo = 19,
    Pathconf = 20,
    Commit = 21,
}

#[derive(Debug)]
pub struct Mount {
    rpc: rpc::Client,
    auth: Auth,
    fh: Vec<u8>,
    dir: String,
    dircount: u32,
    maxcount: u32,
}

impl Mount {
    fn pack_nfs3<Out: xdr_codec::Write>(&self, proc: NFSProc3, args: &dyn xdr_codec::Pack<Out>, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(rpc_header(rpc::NFS3_PROG, rpc::NFS3_VERSION, proc as u32, &self.auth).pack(out)? + args.pack(out)?)
    }

    fn pack_mount3<Out: xdr_codec::Write>(&self, proc: MountProc3, args: &dyn xdr_codec::Pack<Out>, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(rpc_header(rpc::MOUNT3_PROG, rpc::MOUNT3_VERSION, proc as u32, &self.auth).pack(out)? + args.pack(out)?)
    }
}

fn rpc_header(prog: u32, vers: u32, proc: u32, cred: &Auth) -> rpc::Header {
    rpc::Header::new(rpc::RPC_VERSION, prog, vers, proc, cred, &Auth::new_null())
}

fn split_path(path: &str) -> Result<(String, String)> {
    let cleaned = &path_clean::clean(path);
    let dir = cleaned.parent().map_or("/".to_string(), |x| x.to_string_lossy().to_string());
    let name = cleaned.file_name().unwrap_or_default().to_string_lossy().to_string();
    Ok((dir, name))
}

#[allow(unused, non_camel_case_types)]
mod nfs3xdr;

#[allow(unused, non_camel_case_types)]
mod mount3xdr;

use xdr_codec::Pack;
use nfs3xdr::{TRUE, FALSE, createmode3, createverf3, diropargs3, nfs_fh3, nfspath3, post_op_fh3, sattrguard3, size3, set_mode3, set_uid3, set_gid3, set_atime, set_mtime};

fn from_post_op_fh3(pofh: post_op_fh3) -> Result<Vec<u8>> {
    match pofh {
        post_op_fh3::TRUE(fh) => Ok(fh.data),
        _ => Err(Error::new(ErrorKind::Other, "bad file handle")),
    }
}

#[allow(unused)]
pub(crate) use nfs3xdr::nfsstat3 as ErrorCode;

impl std::error::Error for ErrorCode {}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::NFS3_OK => write!(f, "call completed successfully"),
            ErrorCode::NFS3ERR_PERM => write!(f, "permission denied"),
            ErrorCode::NFS3ERR_NOENT => write!(f, "no such file or directory"),
            ErrorCode::NFS3ERR_IO => write!(f, "i/o error occurred while processing the requested operation"),
            ErrorCode::NFS3ERR_NXIO => write!(f, "i/o error - no such device or address"),
            ErrorCode::NFS3ERR_ACCES => write!(f, "permission denied"), // FIXME: should message be different from nfsstat3::NFS3ERR_PERM?
            ErrorCode::NFS3ERR_EXIST => write!(f, "file exists"),
            ErrorCode::NFS3ERR_XDEV => write!(f, "cross-device hard link not allowed"),
            ErrorCode::NFS3ERR_NODEV => write!(f, "no such device"),
            ErrorCode::NFS3ERR_NOTDIR => write!(f, "not a directory"),
            ErrorCode::NFS3ERR_ISDIR => write!(f, "is a directory"),
            ErrorCode::NFS3ERR_INVAL => write!(f, "invalid or unsupported argument"),
            ErrorCode::NFS3ERR_FBIG => write!(f, "file too large"),
            ErrorCode::NFS3ERR_NOSPC => write!(f, "no space left on device"),
            ErrorCode::NFS3ERR_ROFS => write!(f, "read-only file system"),
            ErrorCode::NFS3ERR_MLINK => write!(f, "too many hard links"),
            ErrorCode::NFS3ERR_NAMETOOLONG => write!(f, "name is too long"),
            ErrorCode::NFS3ERR_NOTEMPTY => write!(f, "directory not empty"),
            ErrorCode::NFS3ERR_DQUOT => write!(f, "resource (quota) hard limit exceeded"),
            ErrorCode::NFS3ERR_STALE => write!(f, "invalid file handle"),
            ErrorCode::NFS3ERR_REMOTE => write!(f, "too many levels of remote in path"),
            ErrorCode::NFS3ERR_BADHANDLE => write!(f, "illegal NFS file handle"),
            ErrorCode::NFS3ERR_NOT_SYNC => write!(f, "update synchronization mismatch"),
            ErrorCode::NFS3ERR_BAD_COOKIE => write!(f, "cookie is stale"),
            ErrorCode::NFS3ERR_NOTSUPP => write!(f, "operation is not supported"),
            ErrorCode::NFS3ERR_TOOSMALL => write!(f, "buffer or request is too small"),
            ErrorCode::NFS3ERR_SERVERFAULT => write!(f, "internal server error"),
            ErrorCode::NFS3ERR_BADTYPE => write!(f, "type not supported by server"),
            ErrorCode::NFS3ERR_JUKEBOX => write!(f, "try again"),
        }
    }
}

#[allow(unused)]
pub(crate) use mount3xdr::mountstat3 as MountErrorCode;

impl std::error::Error for MountErrorCode {}

impl std::fmt::Display for MountErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MountErrorCode::MNT3_OK => write!(f, "call completed successfully"),
            MountErrorCode::MNT3ERR_PERM => write!(f, "permission denied"),
            MountErrorCode::MNT3ERR_NOENT => write!(f, "no such file or directory"),
            MountErrorCode::MNT3ERR_IO => write!(f, "i/o error occurred while processing the requested operation"),
            MountErrorCode::MNT3ERR_ACCES => write!(f, "permission denied"), // FIXME: should message be different from mountstat3::MNT3ERR_PERM?
            MountErrorCode::MNT3ERR_NOTDIR => write!(f, "not a directory"),
            MountErrorCode::MNT3ERR_INVAL => write!(f, "invalid or unsupported argument"),
            MountErrorCode::MNT3ERR_NAMETOOLONG => write!(f, "name is too long"),
            MountErrorCode::MNT3ERR_NOTSUPP => write!(f, "operation is not supported"),
            MountErrorCode::MNT3ERR_SERVERFAULT => write!(f, "internal server error"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct MOUNT3args {
    header: rpc::Header,
    dirpath: mount3xdr::dirpath,
}

impl<Out: xdr_codec::Write> Pack<Out> for MOUNT3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.header.pack(out)? + self.dirpath.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct UMOUNT3args {
    dirpath: mount3xdr::dirpath,
}

impl<Out: xdr_codec::Write> Pack<Out> for UMOUNT3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dirpath.pack(out)?)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct FSInfo {
	pub attr:       Option<Fattr>,
	pub rtmax:      u32,
	pub rtpref:     u32,
	pub rtmult:     u32,
	pub wtmax:      u32,
	pub wtpref:     u32,
	pub wtmult:     u32,
	pub dtpref:     u32,
	pub size:       u64,
	pub time_delta: Time,
	pub properties: u32,
}

impl From<nfs3xdr::FSINFO3resok> for FSInfo {
    fn from(ok: nfs3xdr::FSINFO3resok) -> Self {
        Self{
            attr: match ok.obj_attributes {
                nfs3xdr::post_op_attr::TRUE(a) => Some(a.into()),
                nfs3xdr::post_op_attr::FALSE => None,
            },
            rtmax: ok.rtmax,
            rtpref: ok.rtpref,
            rtmult: ok.rtmult,
            wtmax: ok.wtmax,
            wtpref: ok.wtpref,
            wtmult: ok.wtmult,
            dtpref: ok.dtpref,
            size: ok.maxfilesize,
            time_delta: Time{seconds: ok.time_delta.seconds, nseconds: ok.time_delta.nseconds},
            properties: ok.properties,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct FSStat {
    pub attr: Option<Fattr>,
    pub tbytes: u64,
    pub fbytes: u64,
    pub abytes: u64,
    pub tfiles: u64,
    pub ffiles: u64,
    pub afiles: u64,
    pub invarsec: u32,
}

impl From<nfs3xdr::FSSTAT3resok> for FSStat {
    fn from(ok: nfs3xdr::FSSTAT3resok) -> Self {
        Self {
            attr: match ok.obj_attributes {
                nfs3xdr::post_op_attr::TRUE(a) => Some(a.into()),
                nfs3xdr::post_op_attr::FALSE => None,
            },
            tbytes: ok.tbytes,
            fbytes: ok.fbytes,
            abytes: ok.abytes,
            tfiles: ok.tfiles,
            ffiles: ok.ffiles,
            afiles: ok.afiles,
            invarsec: ok.invarsec,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Fattr {
    pub type_: u32,
    pub file_mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub filesize: u64,
    pub used: u64,
    pub spec_data: [u32; 2],
    pub fsid: u64,
    pub fileid: u64,
    pub atime: Time,
    pub mtime: Time,
    pub ctime: Time,
}

impl From<nfs3xdr::fattr3> for Fattr {
    fn from(attr: nfs3xdr::fattr3) -> Self {
        Self{
            type_: attr.type_ as u32,
            file_mode: attr.mode,
            nlink: attr.nlink,
            uid: attr.uid,
            gid: attr.gid,
            filesize: attr.size,
            used: attr.used,
            spec_data: [attr.rdev.specdata1, attr.rdev.specdata2],
            fsid: attr.fsid,
            fileid: attr.fileid,
            atime: Time{seconds: attr.atime.seconds, nseconds: attr.atime.nseconds},
            mtime: Time{seconds: attr.mtime.seconds, nseconds: attr.mtime.nseconds},
            ctime: Time{seconds: attr.ctime.seconds, nseconds: attr.ctime.nseconds},
        }
    }
}

impl From<Fattr> for crate::mount::Attr {
    fn from(attr: Fattr) -> Self {
        (&attr).into()
    }
}

impl From<&Fattr> for crate::mount::Attr {
    fn from(attr: &Fattr) -> Self {
        Self{
            type_: attr.type_,
            file_mode: attr.file_mode,
            nlink: attr.nlink,
            uid: attr.uid,
            gid: attr.gid,
            filesize: attr.filesize,
            used: attr.used,
            spec_data: attr.spec_data,
            fsid: attr.fsid,
            fileid: attr.fileid,
            atime: Time{seconds: attr.atime.seconds, nseconds: attr.atime.nseconds},
            mtime: Time{seconds: attr.mtime.seconds, nseconds: attr.mtime.nseconds},
            ctime: Time{seconds: attr.ctime.seconds, nseconds: attr.ctime.nseconds},
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Pathconf {
    pub attr: Option<Fattr>,
    pub linkmax: u32,
    pub name_max: u32,
    pub no_trunc: bool,
    pub chown_restricted: bool,
    pub case_insensitive: bool,
    pub case_preserving: bool,
}

impl From<nfs3xdr::PATHCONF3resok> for Pathconf {
    fn from(ok: nfs3xdr::PATHCONF3resok) -> Self {
        Self{
            attr: match ok.obj_attributes {
                nfs3xdr::post_op_attr::TRUE(a) => Some(a.into()),
                nfs3xdr::post_op_attr::FALSE => None,
            },
            linkmax: ok.linkmax,
            name_max: ok.name_max,
            no_trunc: ok.no_trunc,
            chown_restricted: ok.chown_restricted,
            case_insensitive: ok.case_insensitive,
            case_preserving: ok.case_preserving,
        }
    }
}

impl From<Pathconf> for crate::mount::Pathconf {
    fn from(pathconf: Pathconf) -> Self {
        Self{
            attr: pathconf.attr.map(|pc| pc.into()),
            linkmax: pathconf.linkmax,
            name_max: pathconf.name_max,
            no_trunc: pathconf.no_trunc,
            chown_restricted: pathconf.chown_restricted,
            case_insensitive: pathconf.case_insensitive,
            case_preserving: pathconf.case_preserving,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum set_size3 {
    TRUE(size3),
    default,
}

impl<Out: xdr_codec::Write> Pack<Out> for set_size3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_size3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &set_size3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
struct sattr3 {
    mode: set_mode3,
    uid: set_uid3,
    gid: set_gid3,
    size: set_size3,
    atime: set_atime,
    mtime: set_mtime,
}

impl Default for sattr3 {
    fn default() -> Self {
        Self{
            mode: set_mode3::default,
            uid: set_uid3::default,
            gid: set_gid3::default,
            size: set_size3::default,
            atime: set_atime::default,
            mtime: set_mtime::default,
        }
    }
}

impl<Out: xdr_codec::Write> Pack<Out> for sattr3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        let mut sz = 0;
        match self.mode {
            set_mode3::TRUE(_) => sz += self.mode.pack(out)?,
            set_mode3::default => sz += (FALSE as i32).pack(out)?,
        };
        match self.uid {
            set_uid3::TRUE(_) => sz += self.uid.pack(out)?,
            set_uid3::default => sz += (FALSE as i32).pack(out)?,
        };
        match self.gid {
            set_gid3::TRUE(_) => sz += self.gid.pack(out)?,
            set_gid3::default => sz += (FALSE as i32).pack(out)?,
        };
        match self.size {
            set_size3::TRUE(_) => sz += self.size.pack(out)?,
            set_size3::default => sz += (FALSE as i32).pack(out)?,
        };
        match self.atime {
            set_atime::SET_TO_CLIENT_TIME(_) => sz += self.atime.pack(out)?,
            set_atime::default => sz += (FALSE as i32).pack(out)?,
        };
        match self.mtime {
            set_mtime::SET_TO_CLIENT_TIME(_) => sz += self.mtime.pack(out)?,
            set_mtime::default => sz += (FALSE as i32).pack(out)?,
        };
        Ok(sz)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum createhow3 {
    UNCHECKED(sattr3),
    #[allow(dead_code)]
    GUARDED(sattr3),
    #[allow(dead_code)]
    EXCLUSIVE(createverf3),
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for createhow3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &createhow3::UNCHECKED(ref val) => {
                (createmode3::UNCHECKED as i32).pack(out)? + val.pack(out)?
            }
            &createhow3::GUARDED(ref val) => {
                (createmode3::GUARDED as i32).pack(out)? + val.pack(out)?
            }
            &createhow3::EXCLUSIVE(ref val) => {
                (createmode3::EXCLUSIVE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
struct specdata3 {
    specdata1: u32,
    specdata2: u32,
}

impl<Out: xdr_codec::Write> Pack<Out> for specdata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.specdata1.pack(out)? + self.specdata2.pack(out)?)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
struct devicedata3 {
    dev_attributes: sattr3,
    spec: specdata3,
}

impl<Out: xdr_codec::Write> Pack<Out> for devicedata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dev_attributes.pack(out)? + self.spec.pack(out)?)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum mknoddata3 {
    NF3CHR(devicedata3),
    NF3BLK(devicedata3),
    NF3SOCK(sattr3),
    NF3FIFO(sattr3),
    #[allow(unused)]
    default,
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mknoddata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &mknoddata3::NF3CHR(ref val) => {
                (mknoddata3::NF3CHR as i32).pack(out)? + val.pack(out)?
            }
            &mknoddata3::NF3BLK(ref val) => {
                (mknoddata3::NF3BLK as i32).pack(out)? + val.pack(out)?
            }
            &mknoddata3::NF3SOCK(ref val) => {
                (mknoddata3::NF3SOCK as i32).pack(out)? + val.pack(out)?
            }
            &mknoddata3::NF3FIFO(ref val) => {
                (mknoddata3::NF3FIFO as i32).pack(out)? + val.pack(out)?
            }
            &mknoddata3::default => 0
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
struct symlinkdata3 {
    symlink_attributes: sattr3,
    symlink_data: nfspath3,
}

impl<Out: xdr_codec::Write> Pack<Out> for symlinkdata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.symlink_attributes.pack(out)? + self.symlink_data.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct CREATE3args {
    where_: diropargs3,
    how: createhow3,
}

impl<Out: xdr_codec::Write> Pack<Out> for CREATE3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.how.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct MKDIR3args {
    where_: diropargs3,
    attrs: sattr3,
}

impl<Out: xdr_codec::Write> Pack<Out> for MKDIR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.attrs.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct MKNOD3args {
    where_: diropargs3,
    what: mknoddata3,
}

impl<Out: xdr_codec::Write> Pack<Out> for MKNOD3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.what.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct NULL3args {}

impl<Out: xdr_codec::Write> Pack<Out> for NULL3args {
    fn pack(&self, _out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
struct SETATTR3args {
    object: nfs_fh3,
    new_attributes: sattr3,
    guard: sattrguard3,
}

impl<Out: xdr_codec::Write> Pack<Out> for SETATTR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + self.new_attributes.pack(out)? + self.guard.pack(out)?)
    }
}

#[derive(Debug, PartialEq)]
struct SYMLINK3args {
    where_: diropargs3,
    symlink: symlinkdata3,
}

impl<Out: xdr_codec::Write> Pack<Out> for SYMLINK3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.symlink.pack(out)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpc_header_util() {
        let auth = crate::Auth::new_unix("machinist", 123, 987);
        let header = rpc_header(9, 8, 7, &auth);
        let expected = rpc::Header::new(rpc::RPC_VERSION, 9, 8, 7, &auth, &crate::Auth::new_null());
        assert_eq!(header, expected);
    }

    #[test]
    fn split_path_util() {
        let path = "/first/place/1999.txt";
        let res = split_path(path);
        assert!(res.is_ok());
        let (dir, name) = res.unwrap();
        assert_eq!(dir, "/first/place".to_string());
        assert_eq!(name, "1999.txt".to_string());
    }
}
