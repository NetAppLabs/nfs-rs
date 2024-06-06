use super::{nfs_fh3, ACCESS3args, ACCESS3res, Error, ErrorKind, Mount, Result};

impl Mount {
    pub fn access_path(&self, path: &str, mode: u32) -> Result<u32> {
        self.access(&self.lookup_path(path)?.fh, mode)
    }

    pub fn access(&self, fh: &Vec<u8>, mode: u32) -> Result<u32> {
        let args = ACCESS3args {
            object: nfs_fh3 { data: fh.to_vec() },
            access: mode,
        };
        match self._access(args)? {
            ACCESS3res::NFS3_OK(ok) => Ok(ok.access),
            ACCESS3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
