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
use bindings::exports::component::nfs_rs_component::nfs::{
    Nfs as WitNFS,
    Mount as WitMount,
    Attr as WitAttr,
    Time as WitTime,
    PathConf as WitPathconf,
    ReaddirEntry as WitReaddirEntry,
    ReaddirplusEntry as WitReaddirplusEntry,
};

static mut MOUNTS: Option<HashMap<WitMount, Arc<RwLock<Box<dyn Mount>>>>> = None;

fn get_mounts() -> &'static mut HashMap<WitMount, Arc<RwLock<Box<dyn Mount>>>> {
    unsafe {
        if MOUNTS.is_none() {
            MOUNTS = Some(HashMap::new());
        }
        MOUNTS.as_mut().unwrap()
    }
}

fn get_mount(mnt: WitMount) -> &'static Arc<RwLock<Box<dyn Mount>>> {
    let mounts = get_mounts();
    let mount = mounts.get(&mnt);
    if mount.is_none() {
        panic!("mount not found");
    }
    mount.unwrap()
}

fn add_mount(mount: Box<dyn Mount>) -> WitMount {
    let mounts = get_mounts();
    let mut mnt = rand::random::<u32>();
    while mnt == 0 || mounts.contains_key(&mnt) {
        mnt = rand::random::<u32>();
    }
    mounts.insert(mnt, Arc::new(RwLock::new(mount)));
    mnt
}

fn remove_mount(mnt: WitMount) {
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

fn into_wit_err(_err: Error) -> () {
    ()
}

struct Component;

impl WitNFS for Component {
    fn parse_url_and_mount(url: String) -> Result<WitMount, ()> {
        let ret = parse_url_and_mount(url.as_str());
        if ret.is_err() {
            return Err(());
        }

        let mnt = add_mount(ret.unwrap());

        // let mount = get_mount(mnt).read().unwrap();
        // let ret = mount.lookup("/");
        // if ret.is_err() {
        //     return Err(());
        // }

        // let root_fh = ret.unwrap();
        // Ok(root_fh)

        Ok(mnt)
    }

    fn null(mnt: WitMount) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.null()
            .map_err(into_wit_err)
    }

    fn access(mnt: WitMount, fh: Vec<u8>, mode: u32) -> Result<u32, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.access(&fh, mode)
            .map_err(into_wit_err)
    }

    fn access_path(mnt: WitMount, path: String, mode: u32) -> Result<u32, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.access_path(path.as_str(), mode)
            .map_err(into_wit_err)
    }

    fn close(mnt: WitMount, seqid: u32, stateid: u64) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.close(seqid, stateid)
            .map_err(into_wit_err)
    }

    fn commit(mnt: WitMount, fh: Vec<u8>, offset: u64, count: u32) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.commit(&fh, offset, count)
            .map_err(into_wit_err)
    }

    fn commit_path(mnt: WitMount, path: String, offset: u64, count: u32) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.commit_path(path.as_str(), offset, count)
            .map_err(into_wit_err)
    }

    fn create(mnt: WitMount, dir_fh: Vec<u8>, filename: String, mode: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.create(&dir_fh, filename.as_str(), mode)
            .map_err(into_wit_err)
    }

    fn create_path(mnt: WitMount, path: String, mode: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.create_path(path.as_str(), mode)
            .map_err(into_wit_err)
    }

    fn delegpurge(mnt: WitMount, clientid: u64) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.delegpurge(clientid)
            .map_err(into_wit_err)
    }

    fn delegreturn(mnt: WitMount, stateid: u64) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.delegreturn(stateid)
            .map_err(into_wit_err)
    }

    fn getattr(mnt: WitMount, fh: Vec<u8>) -> Result<WitAttr, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.getattr(&fh)
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn getattr_path(mnt: WitMount, path: String) -> Result<WitAttr, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.getattr_path(path.as_str())
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn setattr(
        mnt: WitMount,
        fh: Vec<u8>,
        guard_ctime: Option<WitTime>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<WitTime>,
        mtime: Option<WitTime>,
    ) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
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
        mnt: WitMount,
        path: String,
        specify_guard: bool,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<WitTime>,
        mtime: Option<WitTime>,
    ) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.setattr_path(
            path.as_str(),
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

    fn getfh(mnt: WitMount) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.getfh()
            .map_err(into_wit_err)
    }

    fn link(mnt: WitMount, src_fh: Vec<u8>, dst_dir_fh: Vec<u8>, dst_filename: String) -> Result<WitAttr, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.link(&src_fh, &dst_dir_fh, dst_filename.as_str())
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn link_path(mnt: WitMount, src_path: String, dst_path: String) -> Result<WitAttr, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.link_path(src_path.as_str(), dst_path.as_str())
            .map(into_wit_attr)
            .map_err(into_wit_err)
    }

    fn symlink(mnt: WitMount, src_path: String, dst_dir_fh: Vec<u8>, dst_filename: String) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.symlink(src_path.as_str(), &dst_dir_fh, dst_filename.as_str())
            .map_err(into_wit_err)
    }

    fn symlink_path(mnt: WitMount, src_path: String, dst_path: String) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.symlink_path(src_path.as_str(), dst_path.as_str())
            .map_err(into_wit_err)
    }

    fn readlink(mnt: WitMount, fh: Vec<u8>) -> Result<String, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readlink(&fh)
            .map_err(into_wit_err)
    }

    fn readlink_path(mnt: WitMount, path: String) -> Result<String, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readlink_path(path.as_str())
            .map_err(into_wit_err)
    }

    fn lookup(mnt: WitMount, path: String) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.lookup(path.as_str())
            .map_err(into_wit_err)
    }

    fn pathconf(mnt: WitMount, fh: Vec<u8>) -> Result<WitPathconf, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.pathconf(&fh)
            .map(into_wit_path_conf)
            .map_err(into_wit_err)
    }

    fn pathconf_path(mnt: WitMount, path: String) -> Result<WitPathconf, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.pathconf_path(path.as_str())
            .map(into_wit_path_conf)
            .map_err(into_wit_err)
    }

    fn read(mnt: WitMount, fh: Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.read(&fh, offset, count)
            .map_err(into_wit_err)
    }

    fn read_path(mnt: WitMount, path: String, offset: u64, count: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.read_path(path.as_str(), offset, count)
            .map_err(into_wit_err)
    }

    fn write(mnt: WitMount, fh: Vec<u8>, offset: u64, data: Vec<u8>) -> Result<u32, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.write(&fh, offset, &data)
            .map_err(into_wit_err)
    }

    fn write_path(mnt: WitMount, path: String, offset: u64, data: Vec<u8>) -> Result<u32, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.write_path(path.as_str(), offset, &data)
            .map_err(into_wit_err)
    }

    fn readdir(mnt: WitMount, dir_fh: Vec<u8>) -> Result<Vec<WitReaddirEntry>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readdir(&dir_fh)
            .map(into_wit_readdir_entries)
            .map_err(into_wit_err)
    }

    fn readdir_path(mnt: WitMount, dir_path: String) -> Result<Vec<WitReaddirEntry>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readdir_path(dir_path.as_str())
            .map(into_wit_readdir_entries)
            .map_err(into_wit_err)
    }

    fn readdirplus(mnt: WitMount, dir_fh: Vec<u8>) -> Result<Vec<WitReaddirplusEntry>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readdirplus(&dir_fh)
            .map(into_wit_readdirplus_entries)
            .map_err(into_wit_err)
    }

    fn readdirplus_path(mnt: WitMount, dir_path: String) -> Result<Vec<WitReaddirplusEntry>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.readdirplus_path(dir_path.as_str())
            .map(into_wit_readdirplus_entries)
            .map_err(into_wit_err)
    }

    fn mkdir(mnt: WitMount, dir_fh: Vec<u8>, dirname: String, mode: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.mkdir(&dir_fh, dirname.as_str(), mode)
            .map_err(into_wit_err)
    }

    fn mkdir_path(mnt: WitMount, path: String, mode: u32) -> Result<Vec<u8>, ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.mkdir_path(path.as_str(), mode)
            .map_err(into_wit_err)
    }

    fn remove(mnt: WitMount, dir_fh: Vec<u8>, filename: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.remove(&dir_fh, filename.as_str())
            .map_err(into_wit_err)
    }

    fn remove_path(mnt: WitMount, path: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.remove_path(path.as_str())
            .map_err(into_wit_err)
    }

    fn rmdir(mnt: WitMount, dir_fh: Vec<u8>, dirname: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.rmdir(&dir_fh, dirname.as_str())
            .map_err(into_wit_err)
    }

    fn rmdir_path(mnt: WitMount, path: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.rmdir_path(path.as_str())
            .map_err(into_wit_err)
    }

    fn rename(mnt: WitMount, from_dir_fh: Vec<u8>, from_filename: String, to_dir_fh: Vec<u8>, to_filename: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.rename(&from_dir_fh, from_filename.as_str(), &to_dir_fh, to_filename.as_str())
            .map_err(into_wit_err)
    }

    fn rename_path(mnt: WitMount, from_path: String, to_path: String) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        mount.rename_path(from_path.as_str(), to_path.as_str())
            .map_err(into_wit_err)
    }

    fn umount(mnt: WitMount) -> Result<(), ()> {
        let mount = get_mount(mnt).read().unwrap();
        let ret = mount.umount();
        if ret.is_ok() {
            remove_mount(mnt);
        }
        ret.map_err(into_wit_err)
    }
}

bindings::export!(Component);
