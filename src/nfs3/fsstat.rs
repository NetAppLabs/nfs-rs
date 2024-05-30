use super::{nfs_fh3, Error, ErrorKind, FSSTAT3args, FSSTAT3res, FSStat, Mount, Result};

impl Mount {
    #[allow(unused)]
    pub fn fsstat(&self) -> Result<FSStat> {
        let args = FSSTAT3args {
            fsroot: nfs_fh3 {
                data: self.fh.to_vec(),
            },
        };
        match self._fsstat(args)? {
            FSSTAT3res::NFS3_OK(ok) => Ok(ok.into()),
            FSSTAT3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
