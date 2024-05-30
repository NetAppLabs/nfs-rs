use super::{nfs_fh3, COMMIT3args, COMMIT3res, Error, ErrorKind, Mount, Result};

impl Mount {
    pub fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.commit(&self.lookup_path(path)?.fh, offset, count)
    }

    pub fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        let args = COMMIT3args {
            file: nfs_fh3 { data: fh.to_vec() },
            offset,
            count,
        };
        match self._commit(args)? {
            COMMIT3res::NFS3_OK(_) => Ok(()),
            COMMIT3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
