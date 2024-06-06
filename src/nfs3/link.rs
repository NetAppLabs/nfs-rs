use super::{
    diropargs3, filename3, nfs_fh3, split_path, Error, ErrorKind, Fattr, LINK3args, LINK3res,
    Mount, Result,
};

impl Mount {
    pub fn link_path(&self, src_path: &str, dst_path: &str) -> Result<Fattr> {
        let (dst_dir, dst_filename) = split_path(dst_path)?;
        let src_fh = self.lookup_path(src_path)?.fh;
        let dst_dir_fh = self.lookup_path(&dst_dir)?.fh;
        self.link(&src_fh, &dst_dir_fh, &dst_filename)
    }

    pub fn link(
        &self,
        src_fh: &Vec<u8>,
        dst_dir_fh: &Vec<u8>,
        dst_filename: &str,
    ) -> Result<Fattr> {
        let args = LINK3args {
            file: nfs_fh3 {
                data: src_fh.to_vec(),
            },
            link: diropargs3 {
                dir: nfs_fh3 {
                    data: dst_dir_fh.to_vec(),
                },
                name: filename3(dst_filename.to_string()),
            },
        };
        match self._link(args)? {
            LINK3res::NFS3_OK(ok) => Into::<Option<Fattr>>::into(ok.file_attributes)
                .ok_or(Error::new(ErrorKind::Other, "linking failed")),
            LINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
