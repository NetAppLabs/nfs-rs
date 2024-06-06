use super::{
    diropargs3, filename3, from_post_op_fh3, nfs_fh3, nfspath3, sattr3, split_path, symlinkdata3,
    Error, ErrorKind, Mount, ObjRes, Result, SYMLINK3args, SYMLINK3res, SYMLINK3resok,
};

impl Mount {
    pub fn symlink_path(&self, src_path: &str, dst_path: &str) -> Result<ObjRes> {
        let (dst_dir, dst_filename) = split_path(dst_path)?;
        let dst_dir_fh = self.lookup_path(&dst_dir)?.fh;
        self.symlink(src_path, &dst_dir_fh, &dst_filename)
    }

    pub fn symlink(
        &self,
        src_path: &str,
        dst_dir_fh: &Vec<u8>,
        dst_filename: &str,
    ) -> Result<ObjRes> {
        let args = SYMLINK3args {
            where_: diropargs3 {
                dir: nfs_fh3 {
                    data: dst_dir_fh.to_vec(),
                },
                name: filename3(dst_filename.to_string()),
            },
            symlink: symlinkdata3 {
                symlink_attributes: sattr3::default(),
                symlink_data: nfspath3(src_path.to_string()),
            },
        };
        match self._symlink(args)? {
            SYMLINK3res::NFS3_OK(ok) => ok.into(),
            SYMLINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<SYMLINK3resok> for Result<ObjRes> {
    fn from(value: SYMLINK3resok) -> Self {
        Ok(ObjRes {
            fh: from_post_op_fh3(value.obj)?,
            attr: value.obj_attributes.into(),
        })
    }
}
