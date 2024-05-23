use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, ObjRes, MKDIR3args, sattr3, set_size3, from_post_op_fh3};
use super::nfs3xdr::{diropargs3, filename3, nfs_fh3, set_atime, set_gid3, set_mode3, set_mtime, set_uid3, MKDIR3res, MKDIR3resok};
use crate::nfs3;

impl Mount {
    pub fn mkdir_path(&self, path: &str, mode: u32) -> Result<ObjRes> {
        let (dir, dirname) = nfs3::split_path(path)?;
        let dir_fh = self.lookup_path(&dir)?.fh;
        self.mkdir(&dir_fh, &dirname, mode)
    }

    pub fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes> {
        let args = MKDIR3args{
            where_: diropargs3{dir: nfs_fh3{data: dir_fh.to_vec()}, name: filename3(dirname.to_string())},
            attrs: sattr3{
                mode: set_mode3::TRUE(mode),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Mkdir, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = MKDIR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse mkdir response"));
        }

        match x.unwrap().0 {
            MKDIR3res::NFS3_OK(ok) => ok.into(),
            MKDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<MKDIR3resok> for Result<ObjRes> {
    fn from(value: MKDIR3resok) -> Self {
        Ok(ObjRes{
            fh: from_post_op_fh3(value.obj)?,
            attr: value.obj_attributes.into(),
        })
    }
}
