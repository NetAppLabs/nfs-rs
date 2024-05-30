use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, Fattr, split_path};
use super::nfs3xdr::{LINK3args, LINK3res, diropargs3, filename3, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn link_path(&mut self, src_path: &str, dst_path: &str) -> Result<Fattr> {
        let (dst_dir, dst_filename) = split_path(dst_path)?;
        let src_fh = self.lookup_path(src_path)?.fh;
        let dst_dir_fh = self.lookup_path(&dst_dir)?.fh;
        self.link(&src_fh, &dst_dir_fh, &dst_filename)
    }

    pub fn link(&mut self, src_fh: &Vec<u8>, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<Fattr> {
        let args = LINK3args{
            file: nfs_fh3{data: src_fh.to_vec()},
            link: diropargs3{dir: nfs_fh3{data: dst_dir_fh.to_vec()}, name: filename3(dst_filename.to_string())},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Link, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = LINK3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse link response"));
        }

        match x.unwrap().0 {
            LINK3res::NFS3_OK(ok) => match ok.file_attributes {
                nfs3::nfs3xdr::post_op_attr::TRUE(a) => Ok(a.into()),
                _ => Err(Error::new(ErrorKind::Other, "linking failed")),
            },
            LINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
