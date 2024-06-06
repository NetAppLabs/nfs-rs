use super::{
    diropargs3, filename3, nfs_fh3, Error, ErrorKind, LOOKUP3args, LOOKUP3res, LOOKUP3resok, Mount,
    ObjRes, Result,
};

impl Mount {
    pub fn lookup_path(&self, path: &str) -> Result<ObjRes> {
        let mut res = Ok(ObjRes {
            fh: self.fh.to_vec(),
            attr: None,
        });
        for n in &path_clean::clean(path) {
            if res.as_mut().is_ok() && n != "" && n != "/" && n != "." {
                res = self.lookup(&res.as_mut().ok().unwrap().fh, &n.to_string_lossy());
            }
        }
        res
    }

    pub fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes> {
        let args = LOOKUP3args {
            what: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(filename.to_string()),
            },
        };
        match self._lookup(args)? {
            LOOKUP3res::NFS3_OK(ok) => ok.into(),
            LOOKUP3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<LOOKUP3resok> for Result<ObjRes> {
    fn from(value: LOOKUP3resok) -> Self {
        Ok(ObjRes {
            fh: value.object.data,
            attr: value.obj_attributes.into(),
        })
    }
}
