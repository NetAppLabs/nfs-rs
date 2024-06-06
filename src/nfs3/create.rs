use super::{
    createhow3, diropargs3, filename3, from_post_op_fh3, nfs_fh3, sattr3, set_atime, set_gid3,
    set_mode3, set_mtime, set_size3, set_uid3, split_path, CREATE3args, CREATE3res, CREATE3resok,
    Error, ErrorKind, Mount, ObjRes, Result,
};

impl Mount {
    pub fn create_path(&self, path: &str, mode: u32) -> Result<ObjRes> {
        let (dir, filename) = split_path(path)?;
        self.create(&self.lookup_path(&dir)?.fh, &filename, mode)
    }

    pub fn create(&self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<ObjRes> {
        let args = CREATE3args {
            where_: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(filename.to_string()),
            },
            how: createhow3::UNCHECKED(sattr3 {
                mode: set_mode3::TRUE(mode),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            }),
        };
        match self._create(args)? {
            CREATE3res::NFS3_OK(ok) => ok.into(),
            CREATE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<CREATE3resok> for Result<ObjRes> {
    fn from(value: CREATE3resok) -> Self {
        Ok(ObjRes {
            fh: from_post_op_fh3(value.obj)?,
            attr: value.obj_attributes.into(),
        })
    }
}
