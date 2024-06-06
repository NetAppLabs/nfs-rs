use super::{nfs_fh3, Error, ErrorKind, Fattr, GETATTR3args, GETATTR3res, Mount, Result};

impl Mount {
    pub fn getattr_path(&self, path: &str) -> Result<Fattr> {
        self.getattr(&self.lookup_path(path)?.fh)
    }

    pub fn getattr(&self, fh: &Vec<u8>) -> Result<Fattr> {
        let args = GETATTR3args {
            object: nfs_fh3 { data: fh.to_vec() },
        };
        match self._getattr(args)? {
            GETATTR3res::NFS3_OK(ok) => Ok(ok.obj_attributes.into()),
            GETATTR3res::default(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
