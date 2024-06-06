use super::{
    diropargs3, filename3, nfs_fh3, split_path, Error, ErrorKind, Mount, RMDIR3args, RMDIR3res,
    Result,
};

impl Mount {
    pub fn rmdir_path(&self, path: &str) -> Result<()> {
        let (dir, dirname) = split_path(path)?;
        let dir_fh = self.lookup_path(&dir)?.fh;
        self.rmdir(&dir_fh, &dirname)
    }

    pub fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        let args = RMDIR3args {
            object: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(dirname.to_string()),
            },
        };
        match self._rmdir(args)? {
            RMDIR3res::NFS3_OK(_) => Ok(()),
            RMDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
