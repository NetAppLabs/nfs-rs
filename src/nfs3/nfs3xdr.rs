// GENERATED CODE
//
// Generated from nfs.x by xdrgen.
//
// DO NOT EDIT

pub const ACCESS3_DELETE: i64 = 16i64;

pub const ACCESS3_EXECUTE: i64 = 32i64;

pub const ACCESS3_EXTEND: i64 = 8i64;

pub const ACCESS3_LOOKUP: i64 = 2i64;

pub const ACCESS3_MODIFY: i64 = 4i64;

pub const ACCESS3_READ: i64 = 1i64;

pub const FALSE: i64 = 0i64;

pub const FHSIZE2: i64 = 32i64;

pub const FSF3_CANSETTIME: i64 = 16i64;

pub const FSF3_HOMOGENEOUS: i64 = 8i64;

pub const FSF3_LINK: i64 = 1i64;

pub const FSF3_SYMLINK: i64 = 2i64;

pub const MAXNAMLEN2: i64 = 255i64;

pub const MAXPATHLEN2: i64 = 1024i64;

pub const NFS3_COOKIEVERFSIZE: i64 = 8i64;

pub const NFS3_CREATEVERFSIZE: i64 = 8i64;

pub const NFS3_FHSIZE: i64 = 64i64;

pub const NFS3_WRITEVERFSIZE: i64 = 8i64;

pub const NFSCOOKIESIZE2: i64 = 4i64;

pub const NFSMAXDATA2: i64 = 8192i64;

pub const TRUE: i64 = 1i64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ACCESS3args {
    pub object: nfs_fh3,
    pub access: u32,
}

pub enum ACCESS3res {
    NFS3_OK(ACCESS3resok),
    default(Box<ACCESS3resfail>),
}

pub struct ACCESS3resfail {
    pub obj_attributes: post_op_attr,
}

pub struct ACCESS3resok {
    pub obj_attributes: post_op_attr,
    pub access: u32,
}

pub struct COMMIT3args {
    pub file: nfs_fh3,
    pub offset: offset3,
    pub count: count3,
}

pub enum COMMIT3res {
    NFS3_OK(COMMIT3resok),
    default(Box<COMMIT3resfail>),
}

pub struct COMMIT3resfail {
    pub file_wcc: wcc_data,
}

pub struct COMMIT3resok {
    pub file_wcc: wcc_data,
    pub verf: writeverf3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CREATE2args {
    pub where_: diropargs2,
    pub attributes: sattr2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CREATE2res {
    NFS3_OK(CREATE2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CREATE2resok {
    pub file: fhandle2,
    pub attributes: fattr2,
}

pub struct CREATE3args {
    pub where_: diropargs3,
    pub how: createhow3,
}

pub enum CREATE3res {
    NFS3_OK(CREATE3resok),
    default(Box<CREATE3resfail>),
}

pub struct CREATE3resfail {
    pub dir_wcc: wcc_data,
}

pub struct CREATE3resok {
    pub obj: post_op_fh3,
    pub obj_attributes: post_op_attr,
    pub dir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FSINFO3args {
    pub fsroot: nfs_fh3,
}

pub enum FSINFO3res {
    NFS3_OK(FSINFO3resok),
    default(Box<FSINFO3resfail>),
}

pub struct FSINFO3resfail {
    pub obj_attributes: post_op_attr,
}

pub struct FSINFO3resok {
    pub obj_attributes: post_op_attr,
    pub rtmax: u32,
    pub rtpref: u32,
    pub rtmult: u32,
    pub wtmax: u32,
    pub wtpref: u32,
    pub wtmult: u32,
    pub dtpref: u32,
    pub maxfilesize: size3,
    pub time_delta: nfstime3,
    pub properties: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FSSTAT3args {
    pub fsroot: nfs_fh3,
}

pub enum FSSTAT3res {
    NFS3_OK(FSSTAT3resok),
    default(Box<FSSTAT3resfail>),
}

pub struct FSSTAT3resfail {
    pub obj_attributes: post_op_attr,
}

pub struct FSSTAT3resok {
    pub obj_attributes: post_op_attr,
    pub tbytes: size3,
    pub fbytes: size3,
    pub abytes: size3,
    pub tfiles: size3,
    pub ffiles: size3,
    pub afiles: size3,
    pub invarsec: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GETATTR2args {
    pub fhandle: fhandle2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GETATTR2res {
    NFS3_OK(GETATTR2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GETATTR2resok {
    pub attributes: fattr2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GETATTR3args {
    pub object: nfs_fh3,
}

pub enum GETATTR3res {
    NFS3_OK(GETATTR3resok),
    default,
}

pub struct GETATTR3resok {
    pub obj_attributes: fattr3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LINK2args {
    pub from: fhandle2,
    pub to: diropargs2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LINK2res {
    pub status: nfsstat3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LINK3args {
    pub file: nfs_fh3,
    pub link: diropargs3,
}

pub enum LINK3res {
    NFS3_OK(LINK3resok),
    default(Box<LINK3resfail>),
}

pub struct LINK3resfail {
    pub file_attributes: post_op_attr,
    pub linkdir_wcc: wcc_data,
}

pub struct LINK3resok {
    pub file_attributes: post_op_attr,
    pub linkdir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LOOKUP2args {
    pub what: diropargs2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LOOKUP2res {
    NFS3_OK(LOOKUP2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LOOKUP2resok {
    pub file: fhandle2,
    pub attributes: fattr2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LOOKUP3args {
    pub what: diropargs3,
}

pub enum LOOKUP3res {
    NFS3_OK(LOOKUP3resok),
    default(Box<LOOKUP3resfail>),
}

pub struct LOOKUP3resfail {
    pub dir_attributes: post_op_attr,
}

pub struct LOOKUP3resok {
    pub object: nfs_fh3,
    pub obj_attributes: post_op_attr,
    pub dir_attributes: post_op_attr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MKDIR2args {
    pub where_: diropargs2,
    pub attributes: sattr2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MKDIR2res {
    NFS3_OK(MKDIR2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MKDIR2resok {
    pub file: fhandle2,
    pub attributes: fattr2,
}

pub struct MKDIR3args {
    pub where_: diropargs3,
    pub attributes: sattr3,
}

pub enum MKDIR3res {
    NFS3_OK(MKDIR3resok),
    default(Box<MKDIR3resfail>),
}

pub struct MKDIR3resfail {
    pub dir_wcc: wcc_data,
}

pub struct MKDIR3resok {
    pub obj: post_op_fh3,
    pub obj_attributes: post_op_attr,
    pub dir_wcc: wcc_data,
}

pub struct MKNOD3args {
    pub where_: diropargs3,
    pub what: mknoddata3,
}

pub enum MKNOD3res {
    NFS3_OK(MKNOD3resok),
    default(Box<MKNOD3resfail>),
}

pub struct MKNOD3resfail {
    pub dir_wcc: wcc_data,
}

pub struct MKNOD3resok {
    pub obj: post_op_fh3,
    pub obj_attributes: post_op_attr,
    pub dir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PATHCONF3args {
    pub object: nfs_fh3,
}

pub enum PATHCONF3res {
    NFS3_OK(PATHCONF3resok),
    default(Box<PATHCONF3resfail>),
}

pub struct PATHCONF3resfail {
    pub obj_attributes: post_op_attr,
}

pub struct PATHCONF3resok {
    pub obj_attributes: post_op_attr,
    pub linkmax: u32,
    pub name_max: u32,
    pub no_trunc: bool,
    pub chown_restricted: bool,
    pub case_insensitive: bool,
    pub case_preserving: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct READ2args {
    pub file: fhandle2,
    pub offset: u32,
    pub count: u32,
    pub totalcount: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum READ2res {
    NFS3_OK(READ2resok),
    default,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct READ2resok {
    pub attributes: fattr2,
    pub data: nfsdata2,
}

pub struct READ3args {
    pub file: nfs_fh3,
    pub offset: offset3,
    pub count: count3,
}

pub enum READ3res {
    NFS3_OK(READ3resok),
    default(Box<READ3resfail>),
}

pub struct READ3resfail {
    pub file_attributes: post_op_attr,
}

pub struct READ3resok {
    pub file_attributes: post_op_attr,
    pub count: count3,
    pub eof: bool,
    pub data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct READDIR2args {
    pub dir: fhandle2,
    pub cookie: nfscookie2,
    pub count: u32,
}

pub enum READDIR2res {
    NFS3_OK(READDIR2resok),
    default,
}

pub struct READDIR2resok {
    pub entries: Option<Box<entry2>>,
    pub eof: bool,
}

pub struct READDIR3args {
    pub dir: nfs_fh3,
    pub cookie: cookie3,
    pub cookieverf: cookieverf3,
    pub count: count3,
}

pub enum READDIR3res {
    NFS3_OK(READDIR3resok),
    default(Box<READDIR3resfail>),
}

pub struct READDIR3resfail {
    pub dir_attributes: post_op_attr,
}

pub struct READDIR3resok {
    pub dir_attributes: post_op_attr,
    pub cookieverf: cookieverf3,
    pub reply: dirlist3,
}

pub struct READDIRPLUS3args {
    pub dir: nfs_fh3,
    pub cookie: cookie3,
    pub cookieverf: cookieverf3,
    pub dircount: count3,
    pub maxcount: count3,
}

pub enum READDIRPLUS3res {
    NFS3_OK(READDIRPLUS3resok),
    default(Box<READDIRPLUS3resfail>),
}

pub struct READDIRPLUS3resfail {
    pub dir_attributes: post_op_attr,
}

pub struct READDIRPLUS3resok {
    pub dir_attributes: post_op_attr,
    pub cookieverf: cookieverf3,
    pub reply: dirlistplus3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct READLINK2args {
    pub file: fhandle2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum READLINK2res {
    NFS3_OK(READLINK2resok),
    default,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct READLINK2resok {
    pub data: path2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct READLINK3args {
    pub symlink: nfs_fh3,
}

pub enum READLINK3res {
    NFS3_OK(READLINK3resok),
    default(Box<READLINK3resfail>),
}

pub struct READLINK3resfail {
    pub symlink_attributes: post_op_attr,
}

pub struct READLINK3resok {
    pub symlink_attributes: post_op_attr,
    pub data: nfspath3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct REMOVE2args {
    pub what: diropargs2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct REMOVE2res {
    pub status: nfsstat3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct REMOVE3args {
    pub object: diropargs3,
}

pub enum REMOVE3res {
    NFS3_OK(REMOVE3resok),
    default(Box<REMOVE3resfail>),
}

pub struct REMOVE3resfail {
    pub dir_wcc: wcc_data,
}

pub struct REMOVE3resok {
    pub dir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RENAME2args {
    pub from: diropargs2,
    pub to: diropargs2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RENAME2res {
    pub status: nfsstat3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RENAME3args {
    pub from: diropargs3,
    pub to: diropargs3,
}

pub enum RENAME3res {
    NFS3_OK(RENAME3resok),
    default(Box<RENAME3resfail>),
}

pub struct RENAME3resfail {
    pub fromdir_wcc: wcc_data,
    pub todir_wcc: wcc_data,
}

pub struct RENAME3resok {
    pub fromdir_wcc: wcc_data,
    pub todir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RMDIR2args {
    pub what: diropargs2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RMDIR2res {
    pub status: nfsstat3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RMDIR3args {
    pub object: diropargs3,
}

pub enum RMDIR3res {
    NFS3_OK(RMDIR3resok),
    default(Box<RMDIR3resfail>),
}

pub struct RMDIR3resfail {
    pub dir_wcc: wcc_data,
}

pub struct RMDIR3resok {
    pub dir_wcc: wcc_data,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SETATTR2args {
    pub fhandle: fhandle2,
    pub attributes: sattr2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SETATTR2res {
    NFS3_OK(SETATTR2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SETATTR2resok {
    pub attributes: fattr2,
}

pub struct SETATTR3args {
    pub object: nfs_fh3,
    pub new_attributes: sattr3,
    pub guard: sattrguard3,
}

pub enum SETATTR3res {
    NFS3_OK(SETATTR3resok),
    default(Box<SETATTR3resfail>),
}

pub struct SETATTR3resfail {
    pub obj_wcc: wcc_data,
}

pub struct SETATTR3resok {
    pub obj_wcc: wcc_data,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct STATFS2args {
    pub dir: fhandle2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum STATFS2res {
    NFS3_OK(STATFS2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct STATFS2resok {
    pub tsize: u32,
    pub bsize: u32,
    pub blocks: u32,
    pub bfree: u32,
    pub bavail: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SYMLINK2args {
    pub from: diropargs2,
    pub to: path2,
    pub attributes: sattr2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SYMLINK2res {
    pub status: nfsstat3,
}

pub struct SYMLINK3args {
    pub where_: diropargs3,
    pub symlink: symlinkdata3,
}

pub enum SYMLINK3res {
    NFS3_OK(SYMLINK3resok),
    default(Box<SYMLINK3resfail>),
}

pub struct SYMLINK3resfail {
    pub dir_wcc: wcc_data,
}

pub struct SYMLINK3resok {
    pub obj: post_op_fh3,
    pub obj_attributes: post_op_attr,
    pub dir_wcc: wcc_data,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WRITE2args {
    pub file: fhandle2,
    pub beginoffset: u32,
    pub offset: u32,
    pub totalcount: u32,
    pub data: nfsdata2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WRITE2res {
    NFS3_OK(WRITE2resok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WRITE2resok {
    pub attributes: fattr2,
}

pub struct WRITE3args {
    pub file: nfs_fh3,
    pub offset: offset3,
    pub count: count3,
    pub stable: stable_how,
    pub data: Vec<u8>,
}

pub enum WRITE3res {
    NFS3_OK(WRITE3resok),
    default(Box<WRITE3resfail>),
}

pub struct WRITE3resfail {
    pub file_wcc: wcc_data,
}

pub struct WRITE3resok {
    pub file_wcc: wcc_data,
    pub count: count3,
    pub committed: stable_how,
    pub verf: writeverf3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct cookieverf3(pub [u8; NFS3_COOKIEVERFSIZE as usize]);

pub enum createhow3 {
    UNCHECKED(sattr3),
    GUARDED(sattr3),
    EXCLUSIVE(createverf3),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum createmode3 {
    UNCHECKED = 0isize,
    GUARDED = 1isize,
    EXCLUSIVE = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct createverf3(pub [u8; NFS3_CREATEVERFSIZE as usize]);

pub struct devicedata3 {
    pub dev_attributes: sattr3,
    pub spec: specdata3,
}

pub struct dirlist3 {
    pub entries: Option<Box<entry3>>,
    pub eof: bool,
}

pub struct dirlistplus3 {
    pub entries: Option<Box<entryplus3>>,
    pub eof: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct diropargs2 {
    pub dir: fhandle2,
    pub name: filename2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct diropargs3 {
    pub dir: nfs_fh3,
    pub name: filename3,
}

pub struct entry2 {
    pub fileid: u32,
    pub name: filename2,
    pub cookie: nfscookie2,
    pub nextentry: Option<Box<entry2>>,
}

pub struct entry3 {
    pub fileid: fileid3,
    pub name: filename3,
    pub cookie: cookie3,
    pub nextentry: Option<Box<entry3>>,
}

pub struct entryplus3 {
    pub fileid: fileid3,
    pub name: filename3,
    pub cookie: cookie3,
    pub name_attributes: post_op_attr,
    pub name_handle: post_op_fh3,
    pub nextentry: Option<Box<entryplus3>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct fattr2 {
    pub type_: ftype2,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub blocksize: u32,
    pub rdev: u32,
    pub blocks: u32,
    pub fsid: u32,
    pub fileid: u32,
    pub atime: nfstime3,
    pub mtime: nfstime3,
    pub ctime: nfstime3,
}

pub struct fattr3 {
    pub type_: ftype3,
    pub mode: mode3,
    pub nlink: u32,
    pub uid: uid3,
    pub gid: gid3,
    pub size: size3,
    pub used: size3,
    pub rdev: specdata3,
    pub fsid: u64,
    pub fileid: fileid3,
    pub atime: nfstime3,
    pub mtime: nfstime3,
    pub ctime: nfstime3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct fhandle2(pub [u8; FHSIZE2 as usize]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct filename2(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct filename3(pub String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ftype2 {
    NF2NON = 0isize,
    NF2REG = 1isize,
    NF2DIR = 2isize,
    NF2BLK = 3isize,
    NF2CHR = 4isize,
    NF2LNK = 5isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ftype3 {
    NF3REG = 1isize,
    NF3DIR = 2isize,
    NF3BLK = 3isize,
    NF3CHR = 4isize,
    NF3LNK = 5isize,
    NF3SOCK = 6isize,
    NF3FIFO = 7isize,
}

pub enum mknoddata3 {
    NF3CHR(devicedata3),
    NF3BLK(devicedata3),
    NF3SOCK(sattr3),
    NF3FIFO(sattr3),
    default,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct nfs_fh3 {
    pub data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct nfscookie2(pub [u8; NFSCOOKIESIZE2 as usize]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct nfsdata2(pub Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct nfspath3(pub String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfsstat3 {
    NFS3_OK = 0isize,
    NFS3ERR_PERM = 1isize,
    NFS3ERR_NOENT = 2isize,
    NFS3ERR_IO = 5isize,
    NFS3ERR_NXIO = 6isize,
    NFS3ERR_ACCES = 13isize,
    NFS3ERR_EXIST = 17isize,
    NFS3ERR_XDEV = 18isize,
    NFS3ERR_NODEV = 19isize,
    NFS3ERR_NOTDIR = 20isize,
    NFS3ERR_ISDIR = 21isize,
    NFS3ERR_INVAL = 22isize,
    NFS3ERR_FBIG = 27isize,
    NFS3ERR_NOSPC = 28isize,
    NFS3ERR_ROFS = 30isize,
    NFS3ERR_MLINK = 31isize,
    NFS3ERR_NAMETOOLONG = 63isize,
    NFS3ERR_NOTEMPTY = 66isize,
    NFS3ERR_DQUOT = 69isize,
    NFS3ERR_STALE = 70isize,
    NFS3ERR_REMOTE = 71isize,
    NFS3ERR_BADHANDLE = 10001isize,
    NFS3ERR_NOT_SYNC = 10002isize,
    NFS3ERR_BAD_COOKIE = 10003isize,
    NFS3ERR_NOTSUPP = 10004isize,
    NFS3ERR_TOOSMALL = 10005isize,
    NFS3ERR_SERVERFAULT = 10006isize,
    NFS3ERR_BADTYPE = 10007isize,
    NFS3ERR_JUKEBOX = 10008isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct nfstime3 {
    pub seconds: u32,
    pub nseconds: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct path2(pub String);

pub enum post_op_attr {
    TRUE(fattr3),
    FALSE,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum post_op_fh3 {
    TRUE(nfs_fh3),
    FALSE,
}

pub enum pre_op_attr {
    TRUE(wcc_attr),
    FALSE,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct sattr2 {
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub atime: nfstime3,
    pub mtime: nfstime3,
}

pub struct sattr3 {
    pub mode: set_mode3,
    pub uid: set_uid3,
    pub gid: set_gid3,
    pub size: set_size3,
    pub atime: set_atime,
    pub mtime: set_mtime,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sattrguard3 {
    TRUE(nfstime3),
    FALSE,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum set_atime {
    SET_TO_CLIENT_TIME(nfstime3),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum set_gid3 {
    TRUE(gid3),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum set_mode3 {
    TRUE(mode3),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum set_mtime {
    SET_TO_CLIENT_TIME(nfstime3),
    default,
}

pub enum set_size3 {
    TRUE(size3),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum set_uid3 {
    TRUE(uid3),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct specdata3 {
    pub specdata1: u32,
    pub specdata2: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stable_how {
    UNSTABLE = 0isize,
    DATA_SYNC = 1isize,
    FILE_SYNC = 2isize,
}

pub struct symlinkdata3 {
    pub symlink_attributes: sattr3,
    pub symlink_data: nfspath3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum time_how {
    DONT_CHANGE = 0isize,
    SET_TO_SERVER_TIME = 1isize,
    SET_TO_CLIENT_TIME = 2isize,
}

pub struct wcc_attr {
    pub size: size3,
    pub mtime: nfstime3,
    pub ctime: nfstime3,
}

pub struct wcc_data {
    pub before: pre_op_attr,
    pub after: post_op_attr,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct writeverf3(pub [u8; NFS3_WRITEVERFSIZE as usize]);

pub type cookie3 = u64;

pub type count3 = u32;

pub type fileid3 = u64;

pub type gid3 = u32;

pub type mode3 = u32;

pub type offset3 = u64;

pub type size3 = u64;

pub type uid3 = u32;

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ACCESS3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + self.access.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ACCESS3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ACCESS3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &ACCESS3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ACCESS3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ACCESS3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + self.access.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for COMMIT3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.offset.pack(out)? + self.count.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for COMMIT3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &COMMIT3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &COMMIT3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for COMMIT3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for COMMIT3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_wcc.pack(out)? + self.verf.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &CREATE2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &CREATE2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.how.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &CREATE3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &CREATE3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CREATE3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj.pack(out)? + self.obj_attributes.pack(out)? + self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSINFO3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fsroot.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSINFO3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &FSINFO3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &FSINFO3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSINFO3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSINFO3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)?
            + self.rtmax.pack(out)?
            + self.rtpref.pack(out)?
            + self.rtmult.pack(out)?
            + self.wtmax.pack(out)?
            + self.wtpref.pack(out)?
            + self.wtmult.pack(out)?
            + self.dtpref.pack(out)?
            + self.maxfilesize.pack(out)?
            + self.time_delta.pack(out)?
            + self.properties.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSSTAT3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fsroot.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSSTAT3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &FSSTAT3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &FSSTAT3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSSTAT3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for FSSTAT3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)?
            + self.tbytes.pack(out)?
            + self.fbytes.pack(out)?
            + self.abytes.pack(out)?
            + self.tfiles.pack(out)?
            + self.ffiles.pack(out)?
            + self.afiles.pack(out)?
            + self.invarsec.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fhandle.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &GETATTR2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &GETATTR2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &GETATTR3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &GETATTR3res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for GETATTR3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.from.pack(out)? + self.to.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.status.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.link.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LINK3res::NFS3_OK(ref val) => (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?,
            &LINK3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_attributes.pack(out)? + self.linkdir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LINK3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_attributes.pack(out)? + self.linkdir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.what.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LOOKUP2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &LOOKUP2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.what.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LOOKUP3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &LOOKUP3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LOOKUP3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)?
            + self.obj_attributes.pack(out)?
            + self.dir_attributes.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &MKDIR2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &MKDIR2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &MKDIR3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &MKDIR3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKDIR3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj.pack(out)? + self.obj_attributes.pack(out)? + self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKNOD3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.what.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKNOD3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &MKNOD3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &MKNOD3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKNOD3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MKNOD3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj.pack(out)? + self.obj_attributes.pack(out)? + self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PATHCONF3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PATHCONF3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PATHCONF3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &PATHCONF3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PATHCONF3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PATHCONF3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_attributes.pack(out)?
            + self.linkmax.pack(out)?
            + self.name_max.pack(out)?
            + self.no_trunc.pack(out)?
            + self.chown_restricted.pack(out)?
            + self.case_insensitive.pack(out)?
            + self.case_preserving.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)?
            + self.offset.pack(out)?
            + self.count.pack(out)?
            + self.totalcount.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READ2res::NFS3_OK(ref val) => (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?,
            &READ2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.attributes.pack(out)? + self.data.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + self.offset.pack(out)? + self.count.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READ3res::NFS3_OK(ref val) => (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?,
            &READ3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READ3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_attributes.pack(out)?
            + self.count.pack(out)?
            + self.eof.pack(out)?
            + xdr_codec::pack_opaque_flex(&self.data, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)? + self.cookie.pack(out)? + self.count.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READDIR2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &READDIR2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.entries.pack(out)? + self.eof.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)?
            + self.cookie.pack(out)?
            + self.cookieverf.pack(out)?
            + self.count.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READDIR3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &READDIR3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIR3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_attributes.pack(out)? + self.cookieverf.pack(out)? + self.reply.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIRPLUS3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)?
            + self.cookie.pack(out)?
            + self.cookieverf.pack(out)?
            + self.dircount.pack(out)?
            + self.maxcount.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIRPLUS3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READDIRPLUS3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &READDIRPLUS3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIRPLUS3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READDIRPLUS3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_attributes.pack(out)? + self.cookieverf.pack(out)? + self.reply.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READLINK2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &READLINK2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.data.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.symlink.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &READLINK3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &READLINK3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.symlink_attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for READLINK3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.symlink_attributes.pack(out)? + self.data.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.what.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.status.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &REMOVE3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &REMOVE3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for REMOVE3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.from.pack(out)? + self.to.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.status.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.from.pack(out)? + self.to.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &RENAME3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &RENAME3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fromdir_wcc.pack(out)? + self.todir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RENAME3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fromdir_wcc.pack(out)? + self.todir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.what.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.status.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &RMDIR3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &RMDIR3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for RMDIR3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fhandle.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SETATTR2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &SETATTR2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.object.pack(out)? + self.new_attributes.pack(out)? + self.guard.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SETATTR3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &SETATTR3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SETATTR3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for STATFS2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for STATFS2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &STATFS2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &STATFS2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for STATFS2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.tsize.pack(out)?
            + self.bsize.pack(out)?
            + self.blocks.pack(out)?
            + self.bfree.pack(out)?
            + self.bavail.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.from.pack(out)? + self.to.pack(out)? + self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.status.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.where_.pack(out)? + self.symlink.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SYMLINK3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &SYMLINK3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SYMLINK3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.obj.pack(out)? + self.obj_attributes.pack(out)? + self.dir_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE2args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)?
            + self.beginoffset.pack(out)?
            + self.offset.pack(out)?
            + self.totalcount.pack(out)?
            + self.data.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE2res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &WRITE2res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &WRITE2res::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE2resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.attributes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE3args {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file.pack(out)?
            + self.offset.pack(out)?
            + self.count.pack(out)?
            + self.stable.pack(out)?
            + xdr_codec::pack_opaque_flex(&self.data, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE3res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &WRITE3res::NFS3_OK(ref val) => {
                (nfsstat3::NFS3_OK as i32).pack(out)? + val.pack(out)?
            }
            &WRITE3res::default(_) => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE3resfail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_wcc.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for WRITE3resok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.file_wcc.pack(out)?
            + self.count.pack(out)?
            + self.committed.pack(out)?
            + self.verf.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for cookieverf3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
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

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for createmode3 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for createverf3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for devicedata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dev_attributes.pack(out)? + self.spec.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for dirlist3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.entries.pack(out)? + self.eof.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for dirlistplus3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.entries.pack(out)? + self.eof.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for diropargs2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)? + self.name.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for diropargs3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dir.pack(out)? + self.name.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for entry2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fileid.pack(out)?
            + self.name.pack(out)?
            + self.cookie.pack(out)?
            + self.nextentry.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for entry3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fileid.pack(out)?
            + self.name.pack(out)?
            + self.cookie.pack(out)?
            + self.nextentry.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for entryplus3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fileid.pack(out)?
            + self.name.pack(out)?
            + self.cookie.pack(out)?
            + self.name_attributes.pack(out)?
            + self.name_handle.pack(out)?
            + self.nextentry.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for fattr2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.type_.pack(out)?
            + self.mode.pack(out)?
            + self.nlink.pack(out)?
            + self.uid.pack(out)?
            + self.gid.pack(out)?
            + self.size.pack(out)?
            + self.blocksize.pack(out)?
            + self.rdev.pack(out)?
            + self.blocks.pack(out)?
            + self.fsid.pack(out)?
            + self.fileid.pack(out)?
            + self.atime.pack(out)?
            + self.mtime.pack(out)?
            + self.ctime.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for fattr3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.type_.pack(out)?
            + self.mode.pack(out)?
            + self.nlink.pack(out)?
            + self.uid.pack(out)?
            + self.gid.pack(out)?
            + self.size.pack(out)?
            + self.used.pack(out)?
            + self.rdev.pack(out)?
            + self.fsid.pack(out)?
            + self.fileid.pack(out)?
            + self.atime.pack(out)?
            + self.mtime.pack(out)?
            + self.ctime.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for fhandle2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for filename2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(
            &self.0,
            Some(MAXNAMLEN2 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for filename3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, None, out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ftype2 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ftype3 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mknoddata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &mknoddata3::NF3CHR(ref val) => (ftype3::NF3CHR as i32).pack(out)? + val.pack(out)?,
            &mknoddata3::NF3BLK(ref val) => (ftype3::NF3BLK as i32).pack(out)? + val.pack(out)?,
            &mknoddata3::NF3SOCK(ref val) => (ftype3::NF3SOCK as i32).pack(out)? + val.pack(out)?,
            &mknoddata3::NF3FIFO(ref val) => (ftype3::NF3FIFO as i32).pack(out)? + val.pack(out)?,
            &mknoddata3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfs_fh3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(&self.data, Some(NFS3_FHSIZE as usize), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfscookie2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfsdata2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(NFSMAXDATA2 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfspath3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, None, out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfsstat3 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for nfstime3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.seconds.pack(out)? + self.nseconds.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for path2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(
            &self.0,
            Some(MAXPATHLEN2 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for post_op_attr {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &post_op_attr::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &post_op_attr::FALSE => (FALSE as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for post_op_fh3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &post_op_fh3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &post_op_fh3::FALSE => (FALSE as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for pre_op_attr {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &pre_op_attr::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &pre_op_attr::FALSE => (FALSE as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for sattr2 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.mode.pack(out)?
            + self.uid.pack(out)?
            + self.gid.pack(out)?
            + self.size.pack(out)?
            + self.atime.pack(out)?
            + self.mtime.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for sattr3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.mode.pack(out)?
            + self.uid.pack(out)?
            + self.gid.pack(out)?
            + self.size.pack(out)?
            + self.atime.pack(out)?
            + self.mtime.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for sattrguard3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &sattrguard3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &sattrguard3::FALSE => (FALSE as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_atime {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_atime::SET_TO_CLIENT_TIME(ref val) => {
                (time_how::SET_TO_CLIENT_TIME as i32).pack(out)? + val.pack(out)?
            }
            &set_atime::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_gid3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_gid3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &set_gid3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_mode3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_mode3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &set_mode3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_mtime {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_mtime::SET_TO_CLIENT_TIME(ref val) => {
                (time_how::SET_TO_CLIENT_TIME as i32).pack(out)? + val.pack(out)?
            }
            &set_mtime::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_size3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_size3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &set_size3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for set_uid3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &set_uid3::TRUE(ref val) => (TRUE as i32).pack(out)? + val.pack(out)?,
            &set_uid3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for specdata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.specdata1.pack(out)? + self.specdata2.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for stable_how {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for symlinkdata3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.symlink_attributes.pack(out)? + self.symlink_data.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for time_how {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for wcc_attr {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.size.pack(out)? + self.mtime.pack(out)? + self.ctime.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for wcc_data {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.before.pack(out)? + self.after.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for writeverf3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ACCESS3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ACCESS3args, usize)> {
        let mut sz = 0;
        Ok((
            ACCESS3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                access: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ACCESS3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ACCESS3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ACCESS3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => ACCESS3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ACCESS3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ACCESS3resfail, usize)> {
        let mut sz = 0;
        Ok((
            ACCESS3resfail {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ACCESS3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ACCESS3resok, usize)> {
        let mut sz = 0;
        Ok((
            ACCESS3resok {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                access: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for COMMIT3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(COMMIT3args, usize)> {
        let mut sz = 0;
        Ok((
            COMMIT3args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for COMMIT3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(COMMIT3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => COMMIT3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => COMMIT3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for COMMIT3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(COMMIT3resfail, usize)> {
        let mut sz = 0;
        Ok((
            COMMIT3resfail {
                file_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for COMMIT3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(COMMIT3resok, usize)> {
        let mut sz = 0;
        Ok((
            COMMIT3resok {
                file_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                verf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE2args, usize)> {
        let mut sz = 0;
        Ok((
            CREATE2args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => CREATE2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => CREATE2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE2resok, usize)> {
        let mut sz = 0;
        Ok((
            CREATE2resok {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE3args, usize)> {
        let mut sz = 0;
        Ok((
            CREATE3args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                how: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => CREATE3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => CREATE3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE3resfail, usize)> {
        let mut sz = 0;
        Ok((
            CREATE3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CREATE3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CREATE3resok, usize)> {
        let mut sz = 0;
        Ok((
            CREATE3resok {
                obj: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSINFO3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSINFO3args, usize)> {
        let mut sz = 0;
        Ok((
            FSINFO3args {
                fsroot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSINFO3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSINFO3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => FSINFO3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => FSINFO3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSINFO3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSINFO3resfail, usize)> {
        let mut sz = 0;
        Ok((
            FSINFO3resfail {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSINFO3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSINFO3resok, usize)> {
        let mut sz = 0;
        Ok((
            FSINFO3resok {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                rtmax: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                rtpref: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                rtmult: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                wtmax: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                wtpref: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                wtmult: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dtpref: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxfilesize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                time_delta: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                properties: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSSTAT3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSSTAT3args, usize)> {
        let mut sz = 0;
        Ok((
            FSSTAT3args {
                fsroot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSSTAT3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSSTAT3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => FSSTAT3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => FSSTAT3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSSTAT3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSSTAT3resfail, usize)> {
        let mut sz = 0;
        Ok((
            FSSTAT3resfail {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for FSSTAT3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(FSSTAT3resok, usize)> {
        let mut sz = 0;
        Ok((
            FSSTAT3resok {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                tbytes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fbytes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                abytes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                tfiles: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ffiles: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                afiles: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                invarsec: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR2args, usize)> {
        let mut sz = 0;
        Ok((
            GETATTR2args {
                fhandle: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => GETATTR2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => GETATTR2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR2resok, usize)> {
        let mut sz = 0;
        Ok((
            GETATTR2resok {
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR3args, usize)> {
        let mut sz = 0;
        Ok((
            GETATTR3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => GETATTR3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => GETATTR3res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for GETATTR3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(GETATTR3resok, usize)> {
        let mut sz = 0;
        Ok((
            GETATTR3resok {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK2args, usize)> {
        let mut sz = 0;
        Ok((
            LINK2args {
                from: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                to: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK2res, usize)> {
        let mut sz = 0;
        Ok((
            LINK2res {
                status: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK3args, usize)> {
        let mut sz = 0;
        Ok((
            LINK3args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                link: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LINK3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => LINK3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK3resfail, usize)> {
        let mut sz = 0;
        Ok((
            LINK3resfail {
                file_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                linkdir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LINK3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LINK3resok, usize)> {
        let mut sz = 0;
        Ok((
            LINK3resok {
                file_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                linkdir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP2args, usize)> {
        let mut sz = 0;
        Ok((
            LOOKUP2args {
                what: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LOOKUP2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => LOOKUP2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP2resok, usize)> {
        let mut sz = 0;
        Ok((
            LOOKUP2resok {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP3args, usize)> {
        let mut sz = 0;
        Ok((
            LOOKUP3args {
                what: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LOOKUP3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => LOOKUP3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP3resfail, usize)> {
        let mut sz = 0;
        Ok((
            LOOKUP3resfail {
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LOOKUP3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LOOKUP3resok, usize)> {
        let mut sz = 0;
        Ok((
            LOOKUP3resok {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR2args, usize)> {
        let mut sz = 0;
        Ok((
            MKDIR2args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => MKDIR2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => MKDIR2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR2resok, usize)> {
        let mut sz = 0;
        Ok((
            MKDIR2resok {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR3args, usize)> {
        let mut sz = 0;
        Ok((
            MKDIR3args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => MKDIR3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => MKDIR3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR3resfail, usize)> {
        let mut sz = 0;
        Ok((
            MKDIR3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKDIR3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKDIR3resok, usize)> {
        let mut sz = 0;
        Ok((
            MKDIR3resok {
                obj: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKNOD3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKNOD3args, usize)> {
        let mut sz = 0;
        Ok((
            MKNOD3args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                what: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKNOD3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKNOD3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => MKNOD3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => MKNOD3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKNOD3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKNOD3resfail, usize)> {
        let mut sz = 0;
        Ok((
            MKNOD3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MKNOD3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(MKNOD3resok, usize)> {
        let mut sz = 0;
        Ok((
            MKNOD3resok {
                obj: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PATHCONF3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PATHCONF3args, usize)> {
        let mut sz = 0;
        Ok((
            PATHCONF3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PATHCONF3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PATHCONF3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PATHCONF3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => PATHCONF3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PATHCONF3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PATHCONF3resfail, usize)> {
        let mut sz = 0;
        Ok((
            PATHCONF3resfail {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PATHCONF3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PATHCONF3resok, usize)> {
        let mut sz = 0;
        Ok((
            PATHCONF3resok {
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                linkmax: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name_max: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                no_trunc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                chown_restricted: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                case_insensitive: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                case_preserving: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ2args, usize)> {
        let mut sz = 0;
        Ok((
            READ2args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                totalcount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READ2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READ2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ2resok, usize)> {
        let mut sz = 0;
        Ok((
            READ2resok {
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ3args, usize)> {
        let mut sz = 0;
        Ok((
            READ3args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READ3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READ3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ3resfail, usize)> {
        let mut sz = 0;
        Ok((
            READ3resfail {
                file_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READ3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READ3resok, usize)> {
        let mut sz = 0;
        Ok((
            READ3resok {
                file_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                eof: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::unpack_opaque_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR2args, usize)> {
        let mut sz = 0;
        Ok((
            READDIR2args {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READDIR2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READDIR2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR2resok, usize)> {
        let mut sz = 0;
        Ok((
            READDIR2resok {
                entries: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                eof: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR3args, usize)> {
        let mut sz = 0;
        Ok((
            READDIR3args {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookieverf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READDIR3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READDIR3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR3resfail, usize)> {
        let mut sz = 0;
        Ok((
            READDIR3resfail {
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIR3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIR3resok, usize)> {
        let mut sz = 0;
        Ok((
            READDIR3resok {
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookieverf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                reply: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIRPLUS3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIRPLUS3args, usize)> {
        let mut sz = 0;
        Ok((
            READDIRPLUS3args {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookieverf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dircount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxcount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIRPLUS3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIRPLUS3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READDIRPLUS3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READDIRPLUS3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIRPLUS3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIRPLUS3resfail, usize)> {
        let mut sz = 0;
        Ok((
            READDIRPLUS3resfail {
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READDIRPLUS3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READDIRPLUS3resok, usize)> {
        let mut sz = 0;
        Ok((
            READDIRPLUS3resok {
                dir_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookieverf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                reply: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK2args, usize)> {
        let mut sz = 0;
        Ok((
            READLINK2args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READLINK2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READLINK2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK2resok, usize)> {
        let mut sz = 0;
        Ok((
            READLINK2resok {
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK3args, usize)> {
        let mut sz = 0;
        Ok((
            READLINK3args {
                symlink: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => READLINK3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => READLINK3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK3resfail, usize)> {
        let mut sz = 0;
        Ok((
            READLINK3resfail {
                symlink_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for READLINK3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(READLINK3resok, usize)> {
        let mut sz = 0;
        Ok((
            READLINK3resok {
                symlink_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE2args, usize)> {
        let mut sz = 0;
        Ok((
            REMOVE2args {
                what: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE2res, usize)> {
        let mut sz = 0;
        Ok((
            REMOVE2res {
                status: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE3args, usize)> {
        let mut sz = 0;
        Ok((
            REMOVE3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => REMOVE3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => REMOVE3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE3resfail, usize)> {
        let mut sz = 0;
        Ok((
            REMOVE3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for REMOVE3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(REMOVE3resok, usize)> {
        let mut sz = 0;
        Ok((
            REMOVE3resok {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME2args, usize)> {
        let mut sz = 0;
        Ok((
            RENAME2args {
                from: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                to: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME2res, usize)> {
        let mut sz = 0;
        Ok((
            RENAME2res {
                status: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME3args, usize)> {
        let mut sz = 0;
        Ok((
            RENAME3args {
                from: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                to: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => RENAME3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => RENAME3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME3resfail, usize)> {
        let mut sz = 0;
        Ok((
            RENAME3resfail {
                fromdir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                todir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RENAME3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RENAME3resok, usize)> {
        let mut sz = 0;
        Ok((
            RENAME3resok {
                fromdir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                todir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR2args, usize)> {
        let mut sz = 0;
        Ok((
            RMDIR2args {
                what: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR2res, usize)> {
        let mut sz = 0;
        Ok((
            RMDIR2res {
                status: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR3args, usize)> {
        let mut sz = 0;
        Ok((
            RMDIR3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => RMDIR3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => RMDIR3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR3resfail, usize)> {
        let mut sz = 0;
        Ok((
            RMDIR3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for RMDIR3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(RMDIR3resok, usize)> {
        let mut sz = 0;
        Ok((
            RMDIR3resok {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR2args, usize)> {
        let mut sz = 0;
        Ok((
            SETATTR2args {
                fhandle: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SETATTR2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => SETATTR2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR2resok, usize)> {
        let mut sz = 0;
        Ok((
            SETATTR2resok {
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR3args, usize)> {
        let mut sz = 0;
        Ok((
            SETATTR3args {
                object: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                new_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                guard: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SETATTR3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => SETATTR3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR3resfail, usize)> {
        let mut sz = 0;
        Ok((
            SETATTR3resfail {
                obj_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SETATTR3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SETATTR3resok, usize)> {
        let mut sz = 0;
        Ok((
            SETATTR3resok {
                obj_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for STATFS2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(STATFS2args, usize)> {
        let mut sz = 0;
        Ok((
            STATFS2args {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for STATFS2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(STATFS2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => STATFS2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => STATFS2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for STATFS2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(STATFS2resok, usize)> {
        let mut sz = 0;
        Ok((
            STATFS2resok {
                tsize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                bsize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                blocks: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                bfree: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                bavail: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK2args, usize)> {
        let mut sz = 0;
        Ok((
            SYMLINK2args {
                from: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                to: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK2res, usize)> {
        let mut sz = 0;
        Ok((
            SYMLINK2res {
                status: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK3args, usize)> {
        let mut sz = 0;
        Ok((
            SYMLINK3args {
                where_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                symlink: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SYMLINK3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => SYMLINK3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK3resfail, usize)> {
        let mut sz = 0;
        Ok((
            SYMLINK3resfail {
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SYMLINK3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SYMLINK3resok, usize)> {
        let mut sz = 0;
        Ok((
            SYMLINK3resok {
                obj: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                obj_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dir_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE2args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE2args, usize)> {
        let mut sz = 0;
        Ok((
            WRITE2args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                beginoffset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                totalcount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE2res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE2res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => WRITE2res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => WRITE2res::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE2resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE2resok, usize)> {
        let mut sz = 0;
        Ok((
            WRITE2resok {
                attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE3args {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE3args, usize)> {
        let mut sz = 0;
        Ok((
            WRITE3args {
                file: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                stable: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::unpack_opaque_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE3res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE3res, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => WRITE3res::NFS3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => WRITE3res::default({
                    let (v, csz) = xdr_codec::Unpack::unpack(input)?;
                    sz += csz;
                    v
                }),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE3resfail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE3resfail, usize)> {
        let mut sz = 0;
        Ok((
            WRITE3resfail {
                file_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for WRITE3resok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(WRITE3resok, usize)> {
        let mut sz = 0;
        Ok((
            WRITE3resok {
                file_wcc: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                count: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                committed: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                verf: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for cookieverf3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(cookieverf3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; NFS3_COOKIEVERFSIZE as usize] =
                        unsafe { ::std::mem::zeroed() };
                    let sz = xdr_codec::unpack_opaque_array(
                        input,
                        &mut buf[..],
                        NFS3_COOKIEVERFSIZE as usize,
                    )?;
                    (buf, sz)
                };
                sz = usz;
                cookieverf3(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for createhow3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(createhow3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => createhow3::UNCHECKED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => createhow3::GUARDED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => createhow3::EXCLUSIVE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for createmode3 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(createmode3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == createmode3::UNCHECKED as i32 => createmode3::UNCHECKED,
                    x if x == createmode3::GUARDED as i32 => createmode3::GUARDED,
                    x if x == createmode3::EXCLUSIVE as i32 => createmode3::EXCLUSIVE,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for createverf3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(createverf3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; NFS3_CREATEVERFSIZE as usize] =
                        unsafe { ::std::mem::zeroed() };
                    let sz = xdr_codec::unpack_opaque_array(
                        input,
                        &mut buf[..],
                        NFS3_CREATEVERFSIZE as usize,
                    )?;
                    (buf, sz)
                };
                sz = usz;
                createverf3(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for devicedata3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(devicedata3, usize)> {
        let mut sz = 0;
        Ok((
            devicedata3 {
                dev_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                spec: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for dirlist3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(dirlist3, usize)> {
        let mut sz = 0;
        Ok((
            dirlist3 {
                entries: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                eof: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for dirlistplus3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(dirlistplus3, usize)> {
        let mut sz = 0;
        Ok((
            dirlistplus3 {
                entries: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                eof: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for diropargs2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(diropargs2, usize)> {
        let mut sz = 0;
        Ok((
            diropargs2 {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for diropargs3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(diropargs3, usize)> {
        let mut sz = 0;
        Ok((
            diropargs3 {
                dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for entry2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(entry2, usize)> {
        let mut sz = 0;
        Ok((
            entry2 {
                fileid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nextentry: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for entry3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(entry3, usize)> {
        let mut sz = 0;
        Ok((
            entry3 {
                fileid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nextentry: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for entryplus3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(entryplus3, usize)> {
        let mut sz = 0;
        Ok((
            entryplus3 {
                fileid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cookie: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                name_handle: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nextentry: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for fattr2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(fattr2, usize)> {
        let mut sz = 0;
        Ok((
            fattr2 {
                type_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mode: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nlink: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                uid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                gid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                size: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                blocksize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                rdev: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                blocks: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fsid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fileid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                atime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mtime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ctime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for fattr3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(fattr3, usize)> {
        let mut sz = 0;
        Ok((
            fattr3 {
                type_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mode: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nlink: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                uid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                gid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                size: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                used: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                rdev: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fsid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fileid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                atime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mtime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ctime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for fhandle2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(fhandle2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; FHSIZE2 as usize] = unsafe { ::std::mem::zeroed() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], FHSIZE2 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                fhandle2(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for filename2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(filename2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(MAXNAMLEN2 as usize))?;
                sz = usz;
                filename2(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for filename3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(filename3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, None)?;
                sz = usz;
                filename3(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ftype2 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ftype2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ftype2::NF2NON as i32 => ftype2::NF2NON,
                    x if x == ftype2::NF2REG as i32 => ftype2::NF2REG,
                    x if x == ftype2::NF2DIR as i32 => ftype2::NF2DIR,
                    x if x == ftype2::NF2BLK as i32 => ftype2::NF2BLK,
                    x if x == ftype2::NF2CHR as i32 => ftype2::NF2CHR,
                    x if x == ftype2::NF2LNK as i32 => ftype2::NF2LNK,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ftype3 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ftype3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ftype3::NF3REG as i32 => ftype3::NF3REG,
                    x if x == ftype3::NF3DIR as i32 => ftype3::NF3DIR,
                    x if x == ftype3::NF3BLK as i32 => ftype3::NF3BLK,
                    x if x == ftype3::NF3CHR as i32 => ftype3::NF3CHR,
                    x if x == ftype3::NF3LNK as i32 => ftype3::NF3LNK,
                    x if x == ftype3::NF3SOCK as i32 => ftype3::NF3SOCK,
                    x if x == ftype3::NF3FIFO as i32 => ftype3::NF3FIFO,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mknoddata3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mknoddata3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (4i32 as i32) => mknoddata3::NF3CHR({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => mknoddata3::NF3BLK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (6i32 as i32) => mknoddata3::NF3SOCK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (7i32 as i32) => mknoddata3::NF3FIFO({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => mknoddata3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfs_fh3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfs_fh3, usize)> {
        let mut sz = 0;
        Ok((
            nfs_fh3 {
                data: {
                    let (v, fsz) =
                        xdr_codec::unpack_opaque_flex(input, Some(NFS3_FHSIZE as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfscookie2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfscookie2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; NFSCOOKIESIZE2 as usize] =
                        unsafe { ::std::mem::zeroed() };
                    let sz = xdr_codec::unpack_opaque_array(
                        input,
                        &mut buf[..],
                        NFSCOOKIESIZE2 as usize,
                    )?;
                    (buf, sz)
                };
                sz = usz;
                nfscookie2(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfsdata2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfsdata2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(NFSMAXDATA2 as usize))?;
                sz = usz;
                nfsdata2(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfspath3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfspath3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, None)?;
                sz = usz;
                nfspath3(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfsstat3 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfsstat3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == nfsstat3::NFS3_OK as i32 => nfsstat3::NFS3_OK,
                    x if x == nfsstat3::NFS3ERR_PERM as i32 => nfsstat3::NFS3ERR_PERM,
                    x if x == nfsstat3::NFS3ERR_NOENT as i32 => nfsstat3::NFS3ERR_NOENT,
                    x if x == nfsstat3::NFS3ERR_IO as i32 => nfsstat3::NFS3ERR_IO,
                    x if x == nfsstat3::NFS3ERR_NXIO as i32 => nfsstat3::NFS3ERR_NXIO,
                    x if x == nfsstat3::NFS3ERR_ACCES as i32 => nfsstat3::NFS3ERR_ACCES,
                    x if x == nfsstat3::NFS3ERR_EXIST as i32 => nfsstat3::NFS3ERR_EXIST,
                    x if x == nfsstat3::NFS3ERR_XDEV as i32 => nfsstat3::NFS3ERR_XDEV,
                    x if x == nfsstat3::NFS3ERR_NODEV as i32 => nfsstat3::NFS3ERR_NODEV,
                    x if x == nfsstat3::NFS3ERR_NOTDIR as i32 => nfsstat3::NFS3ERR_NOTDIR,
                    x if x == nfsstat3::NFS3ERR_ISDIR as i32 => nfsstat3::NFS3ERR_ISDIR,
                    x if x == nfsstat3::NFS3ERR_INVAL as i32 => nfsstat3::NFS3ERR_INVAL,
                    x if x == nfsstat3::NFS3ERR_FBIG as i32 => nfsstat3::NFS3ERR_FBIG,
                    x if x == nfsstat3::NFS3ERR_NOSPC as i32 => nfsstat3::NFS3ERR_NOSPC,
                    x if x == nfsstat3::NFS3ERR_ROFS as i32 => nfsstat3::NFS3ERR_ROFS,
                    x if x == nfsstat3::NFS3ERR_MLINK as i32 => nfsstat3::NFS3ERR_MLINK,
                    x if x == nfsstat3::NFS3ERR_NAMETOOLONG as i32 => nfsstat3::NFS3ERR_NAMETOOLONG,
                    x if x == nfsstat3::NFS3ERR_NOTEMPTY as i32 => nfsstat3::NFS3ERR_NOTEMPTY,
                    x if x == nfsstat3::NFS3ERR_DQUOT as i32 => nfsstat3::NFS3ERR_DQUOT,
                    x if x == nfsstat3::NFS3ERR_STALE as i32 => nfsstat3::NFS3ERR_STALE,
                    x if x == nfsstat3::NFS3ERR_REMOTE as i32 => nfsstat3::NFS3ERR_REMOTE,
                    x if x == nfsstat3::NFS3ERR_BADHANDLE as i32 => nfsstat3::NFS3ERR_BADHANDLE,
                    x if x == nfsstat3::NFS3ERR_NOT_SYNC as i32 => nfsstat3::NFS3ERR_NOT_SYNC,
                    x if x == nfsstat3::NFS3ERR_BAD_COOKIE as i32 => nfsstat3::NFS3ERR_BAD_COOKIE,
                    x if x == nfsstat3::NFS3ERR_NOTSUPP as i32 => nfsstat3::NFS3ERR_NOTSUPP,
                    x if x == nfsstat3::NFS3ERR_TOOSMALL as i32 => nfsstat3::NFS3ERR_TOOSMALL,
                    x if x == nfsstat3::NFS3ERR_SERVERFAULT as i32 => nfsstat3::NFS3ERR_SERVERFAULT,
                    x if x == nfsstat3::NFS3ERR_BADTYPE as i32 => nfsstat3::NFS3ERR_BADTYPE,
                    x if x == nfsstat3::NFS3ERR_JUKEBOX as i32 => nfsstat3::NFS3ERR_JUKEBOX,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for nfstime3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nfstime3, usize)> {
        let mut sz = 0;
        Ok((
            nfstime3 {
                seconds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nseconds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for path2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(path2, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(MAXPATHLEN2 as usize))?;
                sz = usz;
                path2(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for post_op_attr {
    fn unpack(input: &mut In) -> xdr_codec::Result<(post_op_attr, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => post_op_attr::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (0i32 as i32) => post_op_attr::FALSE,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for post_op_fh3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(post_op_fh3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => post_op_fh3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (0i32 as i32) => post_op_fh3::FALSE,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for pre_op_attr {
    fn unpack(input: &mut In) -> xdr_codec::Result<(pre_op_attr, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => pre_op_attr::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (0i32 as i32) => pre_op_attr::FALSE,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for sattr2 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sattr2, usize)> {
        let mut sz = 0;
        Ok((
            sattr2 {
                mode: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                uid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                gid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                size: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                atime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mtime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for sattr3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sattr3, usize)> {
        let mut sz = 0;
        Ok((
            sattr3 {
                mode: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                uid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                gid: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                size: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                atime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mtime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for sattrguard3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sattrguard3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => sattrguard3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (0i32 as i32) => sattrguard3::FALSE,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_atime {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_atime, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (2i32 as i32) => set_atime::SET_TO_CLIENT_TIME({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_atime::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_gid3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_gid3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => set_gid3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_gid3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_mode3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_mode3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => set_mode3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_mode3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_mtime {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_mtime, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (2i32 as i32) => set_mtime::SET_TO_CLIENT_TIME({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_mtime::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_size3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_size3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => set_size3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_size3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for set_uid3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(set_uid3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => set_uid3::TRUE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => set_uid3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for specdata3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(specdata3, usize)> {
        let mut sz = 0;
        Ok((
            specdata3 {
                specdata1: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                specdata2: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for stable_how {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(stable_how, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == stable_how::UNSTABLE as i32 => stable_how::UNSTABLE,
                    x if x == stable_how::DATA_SYNC as i32 => stable_how::DATA_SYNC,
                    x if x == stable_how::FILE_SYNC as i32 => stable_how::FILE_SYNC,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for symlinkdata3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(symlinkdata3, usize)> {
        let mut sz = 0;
        Ok((
            symlinkdata3 {
                symlink_attributes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                symlink_data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for time_how {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(time_how, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == time_how::DONT_CHANGE as i32 => time_how::DONT_CHANGE,
                    x if x == time_how::SET_TO_SERVER_TIME as i32 => time_how::SET_TO_SERVER_TIME,
                    x if x == time_how::SET_TO_CLIENT_TIME as i32 => time_how::SET_TO_CLIENT_TIME,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for wcc_attr {
    fn unpack(input: &mut In) -> xdr_codec::Result<(wcc_attr, usize)> {
        let mut sz = 0;
        Ok((
            wcc_attr {
                size: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mtime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ctime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for wcc_data {
    fn unpack(input: &mut In) -> xdr_codec::Result<(wcc_data, usize)> {
        let mut sz = 0;
        Ok((
            wcc_data {
                before: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                after: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for writeverf3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(writeverf3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; NFS3_WRITEVERFSIZE as usize] =
                        unsafe { ::std::mem::zeroed() };
                    let sz = xdr_codec::unpack_opaque_array(
                        input,
                        &mut buf[..],
                        NFS3_WRITEVERFSIZE as usize,
                    )?;
                    (buf, sz)
                };
                sz = usz;
                writeverf3(v)
            },
            sz,
        ))
    }
}
