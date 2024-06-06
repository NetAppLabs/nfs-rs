use super::{nfs_fh3, Error, ErrorKind, Mount, PATHCONF3args, PATHCONF3res, Pathconf, Result};

impl Mount {
    pub fn pathconf_path(&self, path: &str) -> Result<Pathconf> {
        self.pathconf(&self.lookup_path(path)?.fh)
    }

    pub fn pathconf(&self, fh: &Vec<u8>) -> Result<Pathconf> {
        let args = PATHCONF3args {
            object: nfs_fh3 { data: fh.to_vec() },
        };
        match self._pathconf(args)? {
            PATHCONF3res::NFS3_OK(ok) => Ok(ok.into()),
            PATHCONF3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
