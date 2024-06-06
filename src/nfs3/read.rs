use super::{nfs_fh3, Error, ErrorKind, Mount, READ3args, READ3res, Result};

impl Mount {
    pub fn read_path(&self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.read(&self.lookup_path(path)?.fh, offset, count)
    }

    pub fn read(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>> {
        let args = READ3args {
            file: nfs_fh3 { data: fh.to_vec() },
            offset,
            count,
        };
        match self._read(args)? {
            READ3res::NFS3_OK(ok) => Ok(ok.data),
            READ3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
