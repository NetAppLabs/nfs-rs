use crate::{Result, Time};

/// Trait which defines the procedures that can be performed on an NFS mount.
/// 
/// NFS version agnostic.  However, since NFSv4 introduces procedures that are
/// not present in NFSv3, invoking those procedures will return an error when
/// relevant [`Mount`] is NFSv3.
pub trait Mount: std::fmt::Debug + Send + Sync {
    /// Procedure NULL does not do any work. It is made available to allow server response testing and timing.
    fn null(&mut self) -> Result<()>;

    /// Procedure ACCESS determines the access rights that a user, as identified by the credentials in the request, has with respect to a file system object.
    fn access(&mut self, fh: &Vec<u8>, mode: u32) -> Result<u32>;

    /// Same as [`Mount::access`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn access_path(&mut self, path: &str, mode: u32) -> Result<u32>;

    /// The CLOSE operation releases share reservations for the regular or named attribute file as specified by the current filehandle.
    fn close(&mut self, seqid: u32, stateid: u64) -> Result<()>; // FIXME: correct params + return type

    /// Procedure COMMIT forces or flushes data to stable storage that was previously written with a WRITE procedure call with the stable field set to UNSTABLE.
    fn commit(&mut self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()>;

    /// Same as [`Mount::commit`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn commit_path(&mut self, path: &str, offset: u64, count: u32) -> Result<()>;

    /// Procedure CREATE creates a regular file.
    fn create(&mut self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<ObjRes>;

    /// Same as [`Mount::create`] but instead of taking in directory file handle and filename, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn create_path(&mut self, path: &str, mode: u32) -> Result<ObjRes>;

    /// Purges all of the delegations awaiting recovery for a given client.
    fn delegpurge(&mut self, clientid: u64) -> Result<()>; // FIXME: validate params + return type

    /// Returns the delegation represented by the current filehandle and stateid.
    fn delegreturn(&mut self, stateid: u64) -> Result<()>; // FIXME: correct params + return type

    /// Procedure GETATTR retrieves the attributes for a specified file system object.
    fn getattr(&mut self, fh: &Vec<u8>) -> Result<Attr>;

    /// Same as [`Mount::getattr`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn getattr_path(&mut self, path: &str) -> Result<Attr>;

    /// Procedure SETATTR changes one or more of the attributes of a file system object on the server.
    fn setattr(&mut self, fh: &Vec<u8>, guard_ctime: Option<Time>, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()>;

    /// Same as [`Mount::setattr`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    /// Also, instead of taking in optional guard ctime, takes in a boolean which determines whether to specify guard in SETATTR procedure or not, using ctime from LOOKUP.
    fn setattr_path(&mut self, path: &str, specify_guard: bool, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()>;

    /// This operation returns the current filehandle value.
    fn getfh(&mut self) -> Result<()>; // FIXME: missing params + return type

    /// Procedure LINK creates a hard link.
    fn link(&mut self, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<Attr>;

    /// Same as [`Mount::link`] but instead of taking in a source file handle, destination directory file handle, and destination filename, takes in a source path for
    /// which file handle is obtained by performing LOOKUP procedure and destination path for which directory file handle is obtained by performing LOOKUP procedure.
    fn link_path(&mut self, src_path: &str, dst_path: &str) -> Result<Attr>;

    /// Procedure SYMLINK creates a new symbolic link.
    fn symlink(&mut self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<ObjRes>;

    /// Same as [`Mount::symlink`] but instead of taking in a destination directory file handle and destination filename, takes in a  destination path for which
    /// directory file handle is obtained by performing LOOKUP procedure.
    fn symlink_path(&mut self, src_path: &str, dst_path: &str) -> Result<ObjRes>;

    /// Procedure READLINK reads the data associated with a symbolic link.
    fn readlink(&mut self, fh: &Vec<u8>) -> Result<String>;

    /// Same as [`Mount::readlink`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn readlink_path(&mut self, path: &str) -> Result<String>;

    /// Procedure LOOKUP searches a directory for a specific name and returns the file handle and attributes for the corresponding file system object.
    fn lookup(&mut self, dir_fh: &Vec<u8>, dirname: &str) -> Result<ObjRes>;

    /// Same as [`Mount::lookup`] but instead of taking in a directory file handle and filename, takes in a path for which directory file handle is obtained by performing LOOKUP procedure
    /// for each directory in the path, in turn.
    fn lookup_path(&mut self, path: &str) -> Result<ObjRes>;

    /// Procedure PATHCONF retrieves the pathconf information for a file or directory.
    fn pathconf(&mut self, fh: &Vec<u8>) -> Result<Pathconf>;

    /// Same as [`Mount::pathconf`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn pathconf_path(&mut self, path: &str) -> Result<Pathconf>;

    /// Procedure READ reads data from a file.
    fn read(&mut self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>>;

    /// Same as [`Mount::read`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn read_path(&mut self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>>;

    /// Procedure WRITE writes data to a file.
    fn write(&mut self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32>;

    /// Same as [`Mount::write`] but instead of taking in a file handle, takes in a path for which file handle is obtained by performing LOOKUP procedure.
    fn write_path(&mut self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32>;

    /// Procedure READDIR retrieves a variable number of entries, in sequence, from a directory and returns the name and file identifier for each,
    /// with information to allow the client to request additional directory entries in a subsequent READDIR request.
    fn readdir(&mut self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirEntry>>;

    /// Same as [`Mount::readdir`] but instead of taking in a directory file handle, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn readdir_path(&mut self, dir_path: &str) -> Result<Vec<ReaddirEntry>>;

    /// Procedure READDIRPLUS retrieves a variable number of entries from a file system directory and returns complete information about each along with information
    /// to allow the client to request additional directory entries in a subsequent READDIRPLUS.  READDIRPLUS differs from READDIR only in the amount of information
    /// returned for each entry.  In READDIR, each entry returns the filename and the fileid.  In READDIRPLUS, each entry returns the name, the fileid, attributes
    /// (including the fileid), and file handle.
    fn readdirplus(&mut self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirplusEntry>>;

    /// Same as [`Mount::readdirplus`] but instead of taking in a directory file handle, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn readdirplus_path(&mut self, dir_path: &str) -> Result<Vec<ReaddirplusEntry>>;

    /// Procedure MKDIR creates a new subdirectory.
    fn mkdir(&mut self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes>;

    /// Same as [`Mount::mkdir`] but instead of taking in directory file handle and dirname, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn mkdir_path(&mut self, path: &str, mode: u32) -> Result<ObjRes>;

    /// Procedure REMOVE removes (deletes) an entry from a directory.
    fn remove(&mut self, dir_fh: &Vec<u8>, filename: &str) -> Result<()>;

    /// Same as [`Mount::remove`] but instead of taking in a directory file handle and filename, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn remove_path(&mut self, path: &str) -> Result<()>;

    /// Procedure RMDIR removes (deletes) a subdirectory from a directory.
    fn rmdir(&mut self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()>;

    /// Same as [`Mount::rmdir`] but instead of taking in a directory file handle and directory name, takes in a path for which directory file handle is obtained by performing LOOKUP procedure.
    fn rmdir_path(&mut self, path: &str) -> Result<()>;

    // Procedure RENAME renames an entry.
    fn rename(&mut self, from_dir_fh: &Vec<u8>, from_filename: &str, to_dir_fh: &Vec<u8>, to_filename: &str) -> Result<()>;

    /// Same as [`Mount::rename`] but instead of taking in a from directory file handle, from filename, to directory file handle, and to filename, takes in a from path for
    /// which directory file handle is obtained by performing LOOKUP procedure and to path for which directory file handle is obtained by performing LOOKUP procedure.
    fn rename_path(&mut self, from_path: &str, to_path: &str) -> Result<()>;

    // Procedure UMOUNT unmounts the mount itself.
    fn umount(&mut self) -> Result<()>;
}

/// Struct describing attributes for an NFS entry.
#[derive(Debug, Default, PartialEq)]
pub struct Attr {
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

/// Struct describing an NFS entry response as returned by various operations.
#[derive(Debug, Default, PartialEq)]
pub struct ObjRes {
    pub fh: Vec<u8>,
    pub attr: Option<Attr>,
}

/// Struct describing path configuration for an NFS entry as returned by [`Mount::pathconf`] and [`Mount::pathconf_path`].
#[derive(Debug, Default, PartialEq)]
pub struct Pathconf {
    pub attr: Option<Attr>,
    pub linkmax: u32,
    pub name_max: u32,
    pub no_trunc: bool,
    pub chown_restricted: bool,
    pub case_insensitive: bool,
    pub case_preserving: bool,
}

/// Struct describing a single NFS entry as returned by [`Mount::readdir`] and [`Mount::readdir_path`].
#[derive(Debug)]
pub struct ReaddirEntry {
    pub fileid: u64,
    pub file_name: String,
    pub cookie: u64,
}

/// Struct describing a single NFS entry as returned by [`Mount::readdirplus`] and [`Mount::readdirplus_path`].
#[derive(Debug)]
pub struct ReaddirplusEntry {
    pub fileid: u64,
    pub file_name: String,
    pub cookie: u64,
    pub attr: Option<Attr>,
    pub handle: Vec<u8>,
}
