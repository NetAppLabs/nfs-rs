use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, SYMLINK3args, sattr3, symlinkdata3, from_post_op_fh3, split_path};
use super::nfs3xdr::{SYMLINK3res, diropargs3, filename3, nfs_fh3, nfspath3};
use crate::nfs3;

impl Mount {
    pub fn symlink_path(&self, src_path: &str, dst_path: &str) -> Result<Vec<u8>> {
        let (dst_dir, dst_filename) = split_path(dst_path)?;
        let dst_dir_fh = self.lookup_path(&dst_dir)?;
        self.symlink(src_path, &dst_dir_fh, &dst_filename)
    }

    pub fn symlink(&self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<Vec<u8>> {
        let args = SYMLINK3args{
            where_: diropargs3{dir: nfs_fh3{data: dst_dir_fh.to_vec()}, name: filename3(dst_filename.to_string())},
            symlink: symlinkdata3{symlink_attributes: sattr3::default(), symlink_data: nfspath3(src_path.to_string())},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Symlink, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = SYMLINK3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse symlink response"));
        }

        match x.unwrap().0 {
            SYMLINK3res::NFS3_OK(ok) => from_post_op_fh3(ok.obj),
            SYMLINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
