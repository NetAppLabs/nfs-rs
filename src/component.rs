use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{
    Error,
    Mount,
    Attr,
    Pathconf,
    ReaddirEntry,
    ReaddirplusEntry,
    Time,
    parse_url_and_mount,
};
use crate::bindings::exports::component::nfs_rs::nfs::{
    Guest as WitNFS,
    Error as WitError,
    GuestNfsMount as WitMount,
    OwnNfsMount as WitNFSMount,
    Attr as WitAttr,
    Time as WitTime,
    PathConf as WitPathconf,
    ReaddirEntry as WitReaddirEntry,
    ReaddirplusEntry as WitReaddirplusEntry,
};

static mut MOUNTS: Option<HashMap<u32, Arc<RwLock<Box<dyn Mount>>>>> = None;

fn get_mounts() -> &'static mut HashMap<u32, Arc<RwLock<Box<dyn Mount>>>> {
    unsafe {
        if MOUNTS.is_none() {
            MOUNTS = Some(HashMap::new());
        }
        MOUNTS.as_mut().unwrap()
    }
}

fn get_mount(mnt: u32) -> Result<&'static Arc<RwLock<Box<dyn Mount>>>, WitError> {
    let mounts = get_mounts();
    let mount = mounts.get(&mnt);
    if mount.is_none() {
        return Err(WitError{
            nfs_error_code: None,
            message: "mount not found".to_string(),
        });
    }
    Ok(mount.unwrap())
}

fn add_mount(mount: Box<dyn Mount>) -> u32 {
    let mounts = get_mounts();
    let mut mnt = rand::random::<u32>();
    while mnt == 0 || mounts.contains_key(&mnt) {
        mnt = rand::random::<u32>();
    }
    mounts.insert(mnt, Arc::new(RwLock::new(mount)));
    mnt
}

fn remove_mount(mnt: u32) {
    let mounts = get_mounts();
    mounts.remove(&mnt);
}

fn from_wit_time(time: WitTime) -> Time {
    Time{
        seconds: time.seconds,
        nseconds: time.nseconds,
    }
}

fn into_wit_time(time: Time) -> WitTime {
    WitTime{
        seconds: time.seconds,
        nseconds: time.nseconds,
    }
}

fn into_wit_attr(attr: Attr) -> WitAttr {
    WitAttr{
        attr_type: attr.type_,
        file_mode: attr.file_mode,
        nlink: attr.nlink,
        uid: attr.uid,
        gid: attr.gid,
        filesize: attr.filesize,
        used: attr.used,
        spec_data: (attr.spec_data[0], attr.spec_data[1]),
        fsid: attr.fsid,
        fileid: attr.fileid,
        atime: into_wit_time(attr.atime),
        mtime: into_wit_time(attr.mtime),
        ctime: into_wit_time(attr.ctime),
    }
}

fn into_wit_path_conf(conf: Pathconf) -> WitPathconf {
    WitPathconf{
        attr: conf.attr.map(into_wit_attr),
        linkmax: conf.linkmax,
        name_max: conf.name_max,
        no_trunc: conf.no_trunc,
        chown_restricted: conf.chown_restricted,
        case_insensitive: conf.case_insensitive,
        case_preserving: conf.case_preserving,
    }
}

fn into_wit_readdir_entries(entries: Vec<ReaddirEntry>) -> Vec<WitReaddirEntry> {
    let mut ret = Vec::new();
    for entry in entries {
        ret.push(into_wit_readdir_entry(entry));
    }
    ret
}

fn into_wit_readdir_entry(entry: ReaddirEntry) -> WitReaddirEntry {
    WitReaddirEntry{
        fileid: entry.fileid,
        file_name: entry.file_name,
        cookie: entry.cookie,
    }
}

fn into_wit_readdirplus_entries(entries: Vec<ReaddirplusEntry>) -> Vec<WitReaddirplusEntry> {
    let mut ret = Vec::new();
    for entry in entries {
        ret.push(into_wit_readdirplus_entry(entry));
    }
    ret
}

fn into_wit_readdirplus_entry(entry: ReaddirplusEntry) -> WitReaddirplusEntry {
    WitReaddirplusEntry{
        fileid: entry.fileid,
        file_name: entry.file_name,
        cookie: entry.cookie,
        attr: entry.attr.map(into_wit_attr),
        handle: entry.handle,
    }
}

fn into_wit_err(mut err: Error) -> WitError {
    if let Some(inner_err) = err.get_mut() {
        if inner_err.is::<crate::nfs3::ErrorCode>() {
            let nfs_error_code = inner_err.downcast_mut::<crate::nfs3::ErrorCode>().unwrap();
            return WitError{
                nfs_error_code: Some(*nfs_error_code as i32),
                message: nfs_error_code.to_string(),
            }
        }
        if inner_err.is::<crate::nfs3::MountErrorCode>() {
            let mount_error_code = inner_err.downcast_mut::<crate::nfs3::MountErrorCode>().unwrap();
            return WitError{
                nfs_error_code: Some(*mount_error_code as i32), // XXX: MOUNT error code values match those of NFS error codes but should we really do this?
                message: mount_error_code.to_string(),
            }
        }
    }
    WitError{
        nfs_error_code: None,
        message: err.to_string(),
    }
}

impl WitNFS for crate::Component {
    fn parse_url_and_mount(url: String,) -> Result<WitNFSMount, WitError> {
        let ret = parse_url_and_mount(&url);
        if ret.is_err() {
            return Err(into_wit_err(ret.unwrap_err()));
        }

        let id = add_mount(ret.unwrap());
        Ok(WitNFSMount::new(crate::NfsMount{id}))
    }
}

impl WitMount for crate::NfsMount {
    fn null_op(&self) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.null()
            .map_err(into_wit_err)
    }

    fn access(&self, fh: Vec<u8>, mode: u32) -> Result<u32, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.access(&fh, mode)
            .map_err(into_wit_err)
    }

    fn access_path(&self, path: String, mode: u32) -> Result<u32, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.access_path(&path, mode)
            .map_err(into_wit_err)
    }

    fn close(&self, seqid: u32, stateid: u64) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.close(seqid, stateid)
            .map_err(into_wit_err)
    }

    fn commit(&self, fh: Vec<u8>, offset: u64, count: u32) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.commit(&fh, offset, count)
            .map_err(into_wit_err)
    }

    fn commit_path(&self, path: String, offset: u64, count: u32) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.commit_path(&path, offset, count)
            .map_err(into_wit_err)
    }

    fn create(&self, dir_fh: Vec<u8>, filename: String, mode: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.create(&dir_fh, &filename, mode)
            .map_err(into_wit_err)
    }

    fn create_path(&self, path: String, mode: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.create_path(&path, mode)
            .map_err(into_wit_err)
    }

    fn delegpurge(&self, clientid: u64) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.delegpurge(clientid)
            .map_err(into_wit_err)
    }

    fn delegreturn(&self, stateid: u64) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.delegreturn(stateid)
            .map_err(into_wit_err)
    }

    fn getattr(&self, fh: Vec<u8>) -> Result<WitAttr, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.getattr(&fh)
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn getattr_path(&self, path: String) -> Result<WitAttr, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.getattr_path(&path)
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn setattr(
        &self,
        fh: Vec<u8>,
        guard_ctime: Option<WitTime>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<WitTime>,
        mtime: Option<WitTime>,
    ) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.setattr(
            &fh,
            guard_ctime.map(from_wit_time),
            mode,
            uid,
            gid,
            size,
            atime.map(from_wit_time),
            mtime.map(from_wit_time),
        )
            .map_err(into_wit_err)
    }

    fn setattr_path(
        &self,
        path: String,
        specify_guard: bool,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<WitTime>,
        mtime: Option<WitTime>,
    ) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.setattr_path(
            &path,
            specify_guard,
            mode,
            uid,
            gid,
            size,
            atime.map(from_wit_time),
            mtime.map(from_wit_time),
        )
            .map_err(into_wit_err)
    }

    fn getfh(&self) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.getfh()
            .map_err(into_wit_err)
    }

    fn link(&self, src_fh: Vec<u8>, dst_dir_fh: Vec<u8>, dst_filename: String) -> Result<WitAttr, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.link(&src_fh, &dst_dir_fh, &dst_filename)
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn link_path(&self, src_path: String, dst_path: String) -> Result<WitAttr, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.link_path(&src_path, &dst_path)
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn symlink(&self, src_path: String, dst_dir_fh: Vec<u8>, dst_filename: String) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.symlink(&src_path, &dst_dir_fh, &dst_filename)
            .map_err(into_wit_err)
    }

    fn symlink_path(&self, src_path: String, dst_path: String) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.symlink_path(&src_path, &dst_path)
            .map_err(into_wit_err)
    }

    fn readlink(&self, fh: Vec<u8>) -> Result<String, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readlink(&fh)
            .map_err(into_wit_err)
    }

    fn readlink_path(&self, path: String) -> Result<String, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readlink_path(&path)
            .map_err(into_wit_err)
    }

    fn lookup(&self, dir_fh: Vec<u8>, filename: String) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.lookup(&dir_fh, &filename)
            .map_err(into_wit_err)
    }

    fn lookup_path(&self, path: String) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.lookup_path(&path)
            .map_err(into_wit_err)
    }

    fn pathconf(&self, fh: Vec<u8>) -> Result<WitPathconf, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.pathconf(&fh)
            .map(into_wit_path_conf)
            .map_err(into_wit_err)
    }

    fn pathconf_path(&self, path: String) -> Result<WitPathconf, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.pathconf_path(&path)
            .map(into_wit_path_conf)
            .map_err(into_wit_err)
    }

    fn read(&self, fh: Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.read(&fh, offset, count)
            .map_err(into_wit_err)
    }

    fn read_path(&self, path: String, offset: u64, count: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.read_path(&path, offset, count)
            .map_err(into_wit_err)
    }

    fn write(&self, fh: Vec<u8>, offset: u64, data: Vec<u8>) -> Result<u32, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.write(&fh, offset, &data)
            .map_err(into_wit_err)
    }

    fn write_path(&self, path: String, offset: u64, data: Vec<u8>) -> Result<u32, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.write_path(&path, offset, &data)
            .map_err(into_wit_err)
    }

    fn readdir(&self, dir_fh: Vec<u8>) -> Result<Vec<WitReaddirEntry>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readdir(&dir_fh)
            .map(into_wit_readdir_entries)
            .map_err(into_wit_err)
    }

    fn readdir_path(&self, dir_path: String) -> Result<Vec<WitReaddirEntry>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readdir_path(&dir_path)
            .map(into_wit_readdir_entries)
            .map_err(into_wit_err)
    }

    fn readdirplus(&self, dir_fh: Vec<u8>) -> Result<Vec<WitReaddirplusEntry>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readdirplus(&dir_fh)
            .map(into_wit_readdirplus_entries)
            .map_err(into_wit_err)
    }

    fn readdirplus_path(&self, dir_path: String) -> Result<Vec<WitReaddirplusEntry>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.readdirplus_path(&dir_path)
            .map(into_wit_readdirplus_entries)
            .map_err(into_wit_err)
    }

    fn mkdir(&self, dir_fh: Vec<u8>, dirname: String, mode: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.mkdir(&dir_fh, &dirname, mode)
            .map_err(into_wit_err)
    }

    fn mkdir_path(&self, path: String, mode: u32) -> Result<Vec<u8>, WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.mkdir_path(&path, mode)
            .map_err(into_wit_err)
    }

    fn remove(&self, dir_fh: Vec<u8>, filename: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.remove(&dir_fh, &filename)
            .map_err(into_wit_err)
    }

    fn remove_path(&self, path: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.remove_path(&path)
            .map_err(into_wit_err)
    }

    fn rmdir(&self, dir_fh: Vec<u8>, dirname: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.rmdir(&dir_fh, &dirname)
            .map_err(into_wit_err)
    }

    fn rmdir_path(&self, path: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.rmdir_path(&path)
            .map_err(into_wit_err)
    }

    fn rename(&self, from_dir_fh: Vec<u8>, from_filename: String, to_dir_fh: Vec<u8>, to_filename: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.rename(&from_dir_fh, &from_filename, &to_dir_fh, &to_filename)
            .map_err(into_wit_err)
    }

    fn rename_path(&self, from_path: String, to_path: String) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        mount.rename_path(&from_path, &to_path)
            .map_err(into_wit_err)
    }

    fn umount(&self) -> Result<(), WitError> {
        let mount = get_mount(self.id)?.read().unwrap();
        let ret = mount.umount();
        if ret.is_ok() {
            remove_mount(self.id);
        }
        ret.map_err(into_wit_err)
    }
}
