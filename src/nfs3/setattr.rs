use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, Time, SETATTR3args, sattr3, set_size3};
use super::nfs3xdr::{SETATTR3res, nfstime3, nfs_fh3, post_op_attr, sattrguard3, set_mode3, set_uid3, set_gid3, set_atime, set_mtime};
use crate::nfs3;

impl Mount {
    pub fn setattr_path(&self, path: &str, specify_guard: bool, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        let res = self.lookup_raw(path)?;
        let guard_ctime = match (specify_guard, res.obj_attributes) {
            (true, post_op_attr::TRUE(ok)) => Some(Time{seconds: ok.ctime.seconds, nseconds: ok.ctime.nseconds}),
            _ => None,
        };
        self.setattr(&res.object.data, guard_ctime, mode, uid, gid, size, atime, mtime)
    }

    pub fn setattr(&self, fh: &Vec<u8>, guard_ctime: Option<Time>, mode: Option<u32>, uid: Option<u32>, gid: Option<u32>, size: Option<u64>, atime: Option<Time>, mtime: Option<Time>) -> Result<()> {
        let args = SETATTR3args{
            object: nfs_fh3{data: fh.to_vec()},
            new_attributes: sattr3{
                mode: mode.map_or(set_mode3::default, |m| set_mode3::TRUE(m)),
                uid: uid.map_or(set_uid3::default, |u| set_uid3::TRUE(u)),
                gid: gid.map_or(set_gid3::default, |g| set_gid3::TRUE(g)),
                size: size.map_or(set_size3::default, |s| set_size3::TRUE(s)),
                atime: atime.map_or(set_atime::default, |a| set_atime::SET_TO_CLIENT_TIME(nfstime3{seconds: a.seconds, nseconds: a.nseconds})),
                mtime: mtime.map_or(set_mtime::default, |m| set_mtime::SET_TO_CLIENT_TIME(nfstime3{seconds: m.seconds, nseconds: m.nseconds})),
            },
            guard: match guard_ctime {
                Some(ctime) => sattrguard3::TRUE(nfstime3{seconds: ctime.seconds, nseconds: ctime.nseconds}),
                None => sattrguard3::FALSE,
            },
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::SetAttr, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = SETATTR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse setattr response"));
        }

        match x.unwrap().0 {
            SETATTR3res::NFS3_OK(_) => Ok(()),
            SETATTR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
