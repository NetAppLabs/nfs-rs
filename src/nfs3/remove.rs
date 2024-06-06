use super::{
    diropargs3, filename3, nfs_fh3, split_path, Error, ErrorKind, Mount, REMOVE3args, REMOVE3res,
    Result,
};

impl Mount {
    pub fn remove_path(&self, path: &str) -> Result<()> {
        let (dir, filename) = split_path(path)?;
        let dir_fh = self.lookup_path(&dir)?.fh;
        self.remove(&dir_fh, &filename)
    }

    pub fn remove(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<()> {
        let args = REMOVE3args {
            object: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(filename.to_string()),
            },
        };
        match self._remove(args)? {
            REMOVE3res::NFS3_OK(_) => Ok(()),
            REMOVE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
