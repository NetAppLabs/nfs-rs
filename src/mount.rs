use crate::{Result, Time};

/// Trait which defines the procedures that can be performed on an NFS mount.
///
/// NFS version agnostic.  However, since NFSv4 introduces procedures that are not present in NFSv3, invoking those
/// procedures will return an error when relevant [`Mount`] is NFSv3.
pub trait Mount: std::fmt::Debug + Send + Sync {
    /// Procedure NULL does not do any work. It is made available to allow server response testing and timing.
    ///
    /// # Example
    ///
    /// ```
    /// fn is_mount_alive(mount: &dyn nfs_rs::Mount) -> std::io::Result<bool> {
    ///     mount.null().map(|_| true)
    /// }
    /// ```
    fn null(&self) -> Result<()>;

    /// Procedure ACCESS determines the access rights that a user, as identified by the credentials in the request,
    /// has with respect to a file system object.
    ///
    /// # Example
    ///
    /// ```
    /// fn check_access(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, mode: u32) -> std::io::Result<bool> {
    ///     mount.access(fh, mode).map(|access| mode == access)
    /// }
    /// ```
    fn access(&self, fh: &Vec<u8>, mode: u32) -> Result<u32>;

    /// Same as [`Mount::access`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn check_access_for_path(mount: &dyn nfs_rs::Mount, path: &str, mode: u32) -> std::io::Result<bool> {
    ///     mount.access_path(path, mode).map(|access| mode == access)
    /// }
    /// ```
    fn access_path(&self, path: &str, mode: u32) -> Result<u32>;

    /// Procedure CLOSE releases share reservations for the regular or named attribute file as specified by the current
    /// filehandle.
    // TODO: code doc example for CLOSE procedure
    fn close(&self, seqid: u32, stateid: u64) -> Result<()>; // FIXME: correct params + return type

    /// Procedure COMMIT forces or flushes data to stable storage that was previously written with a WRITE procedure
    /// call with the stable field set to UNSTABLE.
    ///
    /// # Example
    ///
    /// ```
    /// fn write_and_flush(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> std::io::Result<()> {
    ///     let count = mount.write(fh, offset, data)?;
    ///     mount.commit(fh, offset, count as u32) // safe since `write` returns error if data.len() > u32::MAX
    /// }
    /// ```
    fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()>;

    /// Same as [`Mount::commit`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn write_to_path_and_flush(mount: &dyn nfs_rs::Mount, path: &str, offset: u64, data: &Vec<u8>) -> std::io::Result<()> {
    ///     let count = mount.write_path(path, offset, data)?;
    ///     mount.commit_path(path, offset, count as u32) // safe since `write` returns error if data.len() > u32::MAX
    /// }
    /// ```
    fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()>;

    /// Procedure CREATE creates a regular file.
    ///
    /// # Examples
    ///
    /// ```
    /// fn create_txt(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, name: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     let mode = 0o640;
    ///     let filename = format!("{}.txt", name);
    ///     mount.create(dir_fh, &filename, mode)
    /// }
    ///
    /// fn create_sh(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, name: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     let mode = 0o750;
    ///     let filename = format!("{}.sh", name);
    ///     mount.create(dir_fh, &filename, mode)
    /// }
    /// ```
    fn create(&self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<ObjRes>;

    /// Same as [`Mount::create`] but instead of taking in directory file handle and filename, takes in a path for
    /// which directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Examples
    ///
    /// ```
    /// fn create_txt(mount: &dyn nfs_rs::Mount, dir: &str, name: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     let mode = 0o640;
    ///     let path = format!("{}/{}.txt", dir, name);
    ///     mount.create_path(&path, mode)
    /// }
    ///
    /// fn create_sh(mount: &dyn nfs_rs::Mount, dir: &str, name: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     let mode = 0o750;
    ///     let path = format!("{}/{}.sh", dir, name);
    ///     mount.create_path(&path, mode)
    /// }
    /// ```
    fn create_path(&self, path: &str, mode: u32) -> Result<ObjRes>;

    /// Procedure DELEGPURGE purges all of the delegations awaiting recovery for a given client.
    // TODO: code doc example for DELEGPURGE procedure
    fn delegpurge(&self, clientid: u64) -> Result<()>; // FIXME: validate params + return type

    /// Procedure DELEGRETURN returns the delegation represented by the current filehandle and stateid.
    // TODO: code doc example for DELEGRETURN procedure
    fn delegreturn(&self, stateid: u64) -> Result<()>; // FIXME: correct params + return type

    /// Procedure GETATTR retrieves the attributes for a specified file system object.
    ///
    /// # Example
    ///
    /// ```
    /// fn has_changed_since(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, time: &nfs_rs::Time) -> std::io::Result<bool> {
    ///     mount.getattr(fh).map(|attr| attr.mtime.seconds != time.seconds || attr.mtime.nseconds != time.nseconds)
    /// }
    /// ```
    fn getattr(&self, fh: &Vec<u8>) -> Result<Attr>;

    /// Same as [`Mount::getattr`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn has_path_changed_since(mount: &dyn nfs_rs::Mount, path: &str, time: &nfs_rs::Time) -> std::io::Result<bool> {
    ///     mount.getattr_path(path).map(|attr| attr.mtime.seconds != time.seconds || attr.mtime.nseconds != time.nseconds)
    /// }
    /// ```
    fn getattr_path(&self, path: &str) -> Result<Attr>;

    /// Procedure SETATTR changes one or more of the attributes of a file system object on the server.
    ///
    /// # Examples
    ///
    /// ```
    /// fn chmod(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, mode: u32, guard: Option<nfs_rs::Time>) -> std::io::Result<()> {
    ///     mount.setattr(fh, guard, Some(mode), None, None, None, None, None)
    /// }
    ///
    /// fn chown(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, uid: u32, gid: u32, guard: Option<nfs_rs::Time>) -> std::io::Result<()> {
    ///     mount.setattr(fh, guard, None, Some(uid), Some(gid), None, None, None)
    /// }
    ///
    /// fn truncate(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, size: u64, guard: Option<nfs_rs::Time>) -> std::io::Result<()> {
    ///     mount.setattr(fh, guard, None, None, None, Some(size), None, None)
    /// }
    ///
    /// fn touch(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, now: nfs_rs::Time, guard: Option<nfs_rs::Time>) -> std::io::Result<()> {
    ///     let now_clone = now.clone();
    ///     mount.setattr(fh, guard, None, None, None, None, Some(now), Some(now_clone))
    /// }
    /// ```
    fn setattr(
        &self,
        fh: &Vec<u8>,
        guard_ctime: Option<Time>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Time>,
        mtime: Option<Time>,
    ) -> Result<()>;

    /// Same as [`Mount::setattr`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.  Also, instead of taking in optional guard ctime, takes
    /// in a boolean which determines whether to specify guard in SETATTR procedure or not, using ctime from LOOKUP.
    ///
    /// # Examples
    ///
    /// ```
    /// fn chmod_path(mount: &dyn nfs_rs::Mount, path: &str, mode: u32, guard: bool) -> std::io::Result<()> {
    ///     mount.setattr_path(path, guard, Some(mode), None, None, None, None, None)
    /// }
    ///
    /// fn chown_path(mount: &dyn nfs_rs::Mount, path: &str, uid: u32, gid: u32, guard: bool) -> std::io::Result<()> {
    ///     mount.setattr_path(path, guard, None, Some(uid), Some(gid), None, None, None)
    /// }
    ///
    /// fn truncate_path(mount: &dyn nfs_rs::Mount, path: &str, size: u64, guard: bool) -> std::io::Result<()> {
    ///     mount.setattr_path(path, guard, None, None, None, Some(size), None, None)
    /// }
    ///
    /// fn touch_path(mount: &dyn nfs_rs::Mount, path: &str, now: nfs_rs::Time, guard: bool) -> std::io::Result<()> {
    ///     let now_clone = now.clone();
    ///     mount.setattr_path(path, guard, None, None, None, None, Some(now), Some(now_clone))
    /// }
    /// ```
    fn setattr_path(
        &self,
        path: &str,
        specify_guard: bool,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Time>,
        mtime: Option<Time>,
    ) -> Result<()>;

    /// Procedure GETFH returns the current filehandle value.
    // TODO: code doc example for GETFH procedure
    fn getfh(&self) -> Result<()>; // FIXME: missing params + return type

    /// Procedure LINK creates a hard link.
    ///
    /// # Example
    ///
    /// ```
    /// fn create_link(mount: &dyn nfs_rs::Mount, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> std::io::Result<nfs_rs::Attr> {
    ///     mount.link(src_fh, dst_dir_fh, dst_filename)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.link directly)
    fn link(&self, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<Attr>;

    /// Same as [`Mount::link`] but instead of taking in a source file handle, destination directory file handle,
    /// and destination filename, takes in a source path for which file handle is obtained by performing one or more
    /// LOOKUP procedures and destination path for which directory file handle is obtained by performing one or more
    /// LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn create_link_path(mount: &dyn nfs_rs::Mount, src_path: &str, dst_path: &str) -> std::io::Result<nfs_rs::Attr> {
    ///     mount.link_path(src_path, dst_path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.link_path directly)
    fn link_path(&self, src_path: &str, dst_path: &str) -> Result<Attr>;

    /// Procedure SYMLINK creates a new symbolic link.
    ///
    /// # Example
    ///
    /// ```
    /// fn create_symlink(mount: &dyn nfs_rs::Mount, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     mount.symlink(src_path, dst_dir_fh, dst_filename)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.symlink directly)
    fn symlink(&self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<ObjRes>;

    /// Same as [`Mount::symlink`] but instead of taking in a destination directory file handle and destination
    /// filename, takes in a  destination path for which directory file handle is obtained by performing one or more
    /// LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn create_symlink(mount: &dyn nfs_rs::Mount, src_path: &str, dst_path: &str) -> std::io::Result<nfs_rs::ObjRes> {
    ///     mount.symlink_path(src_path, dst_path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.symlink_path directly)
    fn symlink_path(&self, src_path: &str, dst_path: &str) -> Result<ObjRes>;

    /// Procedure READLINK reads the data associated with a symbolic link.
    ///
    /// # Example
    ///
    /// ```
    /// fn get_symlink_src_path(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>) -> std::io::Result<String> {
    ///     mount.readlink(fh)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.readlink directly)
    fn readlink(&self, fh: &Vec<u8>) -> Result<String>;

    /// Same as [`Mount::readlink`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn get_symlink_src_path_for_path(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<String> {
    ///     mount.readlink_path(path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.readlink_path directly)
    fn readlink_path(&self, path: &str) -> Result<String>;

    /// Procedure LOOKUP searches a directory for a specific name and returns the file handle and attributes for the
    /// corresponding file system object.
    ///
    /// # Example
    ///
    /// ```
    /// fn is_directory(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, filename: &str) -> std::io::Result<bool> {
    ///     const TYPE_DIR: u32 = 2;
    ///     mount.lookup(dir_fh, filename).map(|res| res.attr.map_or(false, |attr| attr.type_ == TYPE_DIR))
    /// }
    /// ```
    fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes>;

    /// Same as [`Mount::lookup`] but instead of taking in a directory file handle and filename, takes in a path for
    /// which directory file handle is obtained by performing one or more LOOKUP procedures for each directory in the
    /// path, in turn.
    ///
    /// # Example
    ///
    /// ```
    /// fn is_path_directory(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<bool> {
    ///     const TYPE_DIR: u32 = 2;
    ///     mount.lookup_path(path).map(|res| res.attr.map_or(false, |attr| attr.type_ == TYPE_DIR))
    /// }
    /// ```
    fn lookup_path(&self, path: &str) -> Result<ObjRes>;

    /// Procedure PATHCONF retrieves the pathconf information for a file or directory.
    ///
    /// # Example
    ///
    /// ```
    /// fn chown_allowed(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>) -> std::io::Result<bool> {
    ///     mount.pathconf(fh).map(|conf| !conf.chown_restricted)
    /// }
    /// ```
    fn pathconf(&self, fh: &Vec<u8>) -> Result<Pathconf>;

    /// Same as [`Mount::pathconf`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn chown_allowed_for_path(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<bool> {
    ///     mount.pathconf_path(path).map(|conf| !conf.chown_restricted)
    /// }
    /// ```
    fn pathconf_path(&self, path: &str) -> Result<Pathconf>;

    /// Procedure READ reads data from a file.
    ///
    /// # Example
    ///
    /// ```
    /// fn read_exact(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, mut offset: u64, mut count: u32) -> std::io::Result<Vec<u8>> {
    ///     let mut ret = Vec::new();
    ///     loop {
    ///         let mut chunk = mount.read(fh, offset, count)?;
    ///         let chunk_len = chunk.len();
    ///         count -= chunk_len as u32;
    ///         ret.append(&mut chunk);
    ///         if count == 0 {
    ///             return Ok(ret);
    ///         }
    ///         offset += chunk_len as u64;
    ///     }
    /// }
    /// ```
    fn read(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>>;

    /// Same as [`Mount::read`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn read_exact_from_path(mount: &dyn nfs_rs::Mount, path: &str, mut offset: u64, mut count: u32) -> std::io::Result<Vec<u8>> {
    ///     let mut ret = Vec::new();
    ///     loop {
    ///         let mut chunk = mount.read_path(path, offset, count)?;
    ///         let chunk_len = chunk.len();
    ///         count -= chunk_len as u32;
    ///         ret.append(&mut chunk);
    ///         if count == 0 {
    ///             return Ok(ret);
    ///         }
    ///         offset += chunk_len as u64;
    ///     }
    /// }
    /// ```
    fn read_path(&self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>>;

    /// Procedure WRITE writes data to a file.
    ///
    /// # Example
    ///
    /// ```
    /// fn write_all(mount: &dyn nfs_rs::Mount, fh: &Vec<u8>, mut offset: u64, mut data: Vec<u8>) -> std::io::Result<()> {
    ///     loop {
    ///         let written_bytes = mount.write(fh, offset, &data)? as usize;
    ///         data.drain(0..written_bytes);
    ///         if data.is_empty() {
    ///             return Ok(());
    ///         }
    ///     }
    /// }
    /// ```
    fn write(&self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32>;

    /// Same as [`Mount::write`] but instead of taking in a file handle, takes in a path for which file handle is
    /// obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn write_all_to_path(mount: &dyn nfs_rs::Mount, path: &str, mut offset: u64, mut data: Vec<u8>) -> std::io::Result<()> {
    ///     loop {
    ///         let written_bytes = mount.write_path(path, offset, &data)? as usize;
    ///         data.drain(0..written_bytes);
    ///         if data.is_empty() {
    ///             return Ok(());
    ///         }
    ///     }
    /// }
    /// ```
    fn write_path(&self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32>;

    /// Procedure READDIR retrieves a variable number of entries, in sequence, from a directory and returns the name
    /// and file identifier for each, with information to allow the client to request additional directory entries in
    /// a subsequent READDIR request.
    ///
    /// # Example
    ///
    /// ```
    /// fn list_entries(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>) -> std::io::Result<()> {
    ///     for entry in mount.readdir(dir_fh)? {
    ///         println!("{}", entry.file_name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn readdir(&self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirEntry>>;

    /// Same as [`Mount::readdir`] but instead of taking in a directory file handle, takes in a path for which
    /// directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn list_entries_in_path(mount: &dyn nfs_rs::Mount, dir_path: &str) -> std::io::Result<()> {
    ///     for entry in mount.readdir_path(dir_path)? {
    ///         println!("{}", entry.file_name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn readdir_path(&self, dir_path: &str) -> Result<Vec<ReaddirEntry>>;

    /// Procedure READDIRPLUS retrieves a variable number of entries from a file system directory and returns complete
    /// information about each along with information to allow the client to request additional directory entries in a
    /// subsequent READDIRPLUS.  READDIRPLUS differs from READDIR only in the amount of information returned for each
    /// entry.  In READDIR, each entry returns the filename and the fileid.  In READDIRPLUS, each entry returns the
    /// name, the fileid, attributes (including the fileid), and file handle.
    ///
    /// # Example
    ///
    /// ```
    /// fn list_detailed_entries(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>) -> std::io::Result<()> {
    ///     println!("{:>5} {:>25} {}", "mode", "size", "name");
    ///     for entry in mount.readdirplus(dir_fh)? {
    ///         let (mode, size) = entry.attr.map_or((String::new(), String::new()), |attr| {
    ///             (
    ///                 format!("{:o}", attr.file_mode),
    ///                 format!("{}", attr.filesize),
    ///             )
    ///         });
    ///         println!("{:>5} {:>25} {}", mode, size, entry.file_name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn readdirplus(&self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirplusEntry>>;

    /// Same as [`Mount::readdirplus`] but instead of taking in a directory file handle, takes in a path for which
    /// directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn list_detailed_entries_in_path(mount: &dyn nfs_rs::Mount, dir_path: &str) -> std::io::Result<()> {
    ///     println!("{:>5} {:>25} {}", "mode", "size", "name");
    ///     for entry in mount.readdirplus_path(dir_path)? {
    ///         let (mode, size) = entry.attr.map_or((String::new(), String::new()), |attr| {
    ///             (
    ///                 format!("{:o}", attr.file_mode),
    ///                 format!("{}", attr.filesize),
    ///             )
    ///         });
    ///         println!("{:>5} {:>25} {}", mode, size, entry.file_name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn readdirplus_path(&self, dir_path: &str) -> Result<Vec<ReaddirplusEntry>>;

    /// Procedure MKDIR creates a new subdirectory.
    ///
    /// # Example
    ///
    /// ```
    /// fn ensure_directory_exists(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, dirname: &str) -> std::io::Result<()> {
    ///     let mode = 0o750;
    ///     let res = mount.mkdir(dir_fh, dirname, mode);
    ///     if res.is_err() {
    ///         let err = res.unwrap_err();
    ///         if err.to_string() != "file exists" {
    ///             return Err(err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes>;

    /// Same as [`Mount::mkdir`] but instead of taking in directory file handle and dirname, takes in a path for which
    /// directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn ensure_directory_exists_for_path(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<()> {
    ///     let mode = 0o750;
    ///     let res = mount.mkdir_path(path, mode);
    ///     if res.is_err() {
    ///         let err = res.unwrap_err();
    ///         if err.to_string() != "file exists" {
    ///             return Err(err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn mkdir_path(&self, path: &str, mode: u32) -> Result<ObjRes>;

    /// Procedure REMOVE removes (deletes) an entry from a directory.
    ///
    /// # Example
    ///
    /// ```
    /// fn delete_file(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, filename: &str) -> std::io::Result<()> {
    ///     mount.remove(dir_fh, filename)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.remove directly)
    fn remove(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<()>;

    /// Same as [`Mount::remove`] but instead of taking in a directory file handle and filename, takes in a path for
    /// which directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn delete_file_path(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<()> {
    ///     mount.remove_path(path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.remove_path directly)
    fn remove_path(&self, path: &str) -> Result<()>;

    /// Procedure RMDIR removes (deletes) a subdirectory from a directory.
    ///
    /// # Example
    ///
    /// ```
    /// fn delete_directory(mount: &dyn nfs_rs::Mount, dir_fh: &Vec<u8>, dirname: &str) -> std::io::Result<()> {
    ///     mount.rmdir(dir_fh, dirname)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.rmdir directly)
    fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()>;

    /// Same as [`Mount::rmdir`] but instead of taking in a directory file handle and directory name, takes in a path
    /// for which directory file handle is obtained by performing one or more LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn delete_directory_path(mount: &dyn nfs_rs::Mount, path: &str) -> std::io::Result<()> {
    ///     mount.rmdir_path(path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.rmdir_path directly)
    fn rmdir_path(&self, path: &str) -> Result<()>;

    // Procedure RENAME renames an entry.
    ///
    /// # Example
    ///
    /// ```
    /// fn mv(
    ///     mount: &dyn nfs_rs::Mount,
    ///     from_dir_fh: &Vec<u8>,
    ///     from_filename: &str,
    ///     to_dir_fh: &Vec<u8>,
    ///     to_filename: &str,
    /// ) -> std::io::Result<()> {
    ///     mount.rename(from_dir_fh, from_filename, to_dir_fh, to_filename)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.rename directly)
    fn rename(
        &self,
        from_dir_fh: &Vec<u8>,
        from_filename: &str,
        to_dir_fh: &Vec<u8>,
        to_filename: &str,
    ) -> Result<()>;

    /// Same as [`Mount::rename`] but instead of taking in a from directory file handle, from filename, to directory
    /// file handle, and to filename, takes in a from path for which directory file handle is obtained by performing
    /// one or more LOOKUP procedures and to path for which directory file handle is obtained by performing one or more
    /// LOOKUP procedures.
    ///
    /// # Example
    ///
    /// ```
    /// fn mv_path(mount: &dyn nfs_rs::Mount, from_path: &str, to_path: &str) -> std::io::Result<()> {
    ///     mount.rename_path(from_path, to_path)
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.rename_path directly)
    fn rename_path(&self, from_path: &str, to_path: &str) -> Result<()>;

    /// Procedure UMOUNT unmounts the mount itself.
    ///
    /// # Example
    ///
    /// ```
    /// fn unmount(mount: &dyn nfs_rs::Mount) -> std::io::Result<()> {
    ///     mount.umount()
    /// }
    /// ```
    // FIXME: better code doc example? (example function is redundant -- could just call mount.umount directly)
    fn umount(&self) -> Result<()>;

    /// Return NFS version
    ///
    /// # Example
    ///
    /// ```
    /// fn list_unsupported_procedures(mount: &dyn nfs_rs::Mount) -> Option<Vec<String>> {
    ///     match mount.version() {
    ///         nfs_rs::NFSVersion::NFSv3 => Some(vec![
    ///             String::from("close"),
    ///             String::from("delegpurge"),
    ///             String::from("delegreturn"),
    ///             String::from("getfh"),
    ///         ]),
    ///         _ => None,
    ///     }
    /// }
    /// ```
    // FIXME: verify that NFSv4 and NFSv4.1 have no unsupported procedures and update code doc if either/both do
    fn version(&self) -> NFSVersion;
}

#[derive(Debug, Eq, PartialEq)]
pub enum NFSVersion {
    Unknown,
    NFSv3,
    NFSv4,
    NFSv4p1,
}

impl Into<NFSVersion> for &str {
    fn into(self) -> NFSVersion {
        match self {
            "3" => NFSVersion::NFSv3,
            "4" => NFSVersion::NFSv4,
            "4.1" => NFSVersion::NFSv4p1,
            _ => NFSVersion::Unknown,
        }
    }
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

/// Struct describing an NFS entry response as returned by various procedures.
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
}

/// Struct describing a single NFS entry as returned by [`Mount::readdirplus`] and [`Mount::readdirplus_path`].
#[derive(Debug)]
pub struct ReaddirplusEntry {
    pub fileid: u64,
    pub file_name: String,
    pub attr: Option<Attr>,
    pub handle: Vec<u8>,
}
