use super::{nfs_fh3, Error, ErrorKind, FSINFO3args, FSINFO3res, FSInfo, Mount, Result};

impl Mount {
    #[allow(unused)]
    pub fn fsinfo(&self) -> Result<FSInfo> {
        let args = FSINFO3args {
            fsroot: nfs_fh3 {
                data: self.fh.to_vec(),
            },
        };
        match self._fsinfo(args)? {
            FSINFO3res::NFS3_OK(ok) => Ok(ok.into()),
            FSINFO3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
