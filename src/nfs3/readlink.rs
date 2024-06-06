use super::{nfs_fh3, Error, ErrorKind, Mount, READLINK3args, READLINK3res, Result};

impl Mount {
    pub fn readlink_path(&self, path: &str) -> Result<String> {
        self.readlink(&self.lookup_path(path)?.fh)
    }

    pub fn readlink(&self, fh: &Vec<u8>) -> Result<String> {
        let args = READLINK3args {
            symlink: nfs_fh3 { data: fh.to_vec() },
        };
        match self._readlink(args)? {
            READLINK3res::NFS3_OK(ok) => Ok(ok.data.0),
            READLINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
