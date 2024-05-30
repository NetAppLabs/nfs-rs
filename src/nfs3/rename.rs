use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, split_path};
use super::nfs3xdr::{RENAME3args, RENAME3res, diropargs3, filename3, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn rename_path(&mut self, from: &str, to: &str) -> Result<()> {
        let (from_dir, from_filename) = split_path(from)?;
        let (to_dir, to_filename) = split_path(to)?;
        let is_same_dir = from_dir == to_dir;
        let from_dir_fh = self.lookup_path(&from_dir)?.fh;
        let to_dir_fh = if is_same_dir {
            from_dir_fh.to_vec()
        } else {
            self.lookup_path(&to_dir)?.fh
        };
        self.rename(&from_dir_fh, &from_filename, &to_dir_fh, &to_filename)
    }

    pub fn rename(&mut self, from_dir_fh: &Vec<u8>, from_filename: &str, to_dir_fh: &Vec<u8>, to_filename: &str) -> Result<()> {
        let args = RENAME3args{
            from: diropargs3{dir: nfs_fh3{data: from_dir_fh.to_vec()}, name: filename3(from_filename.to_string())},
            to: diropargs3{dir: nfs_fh3{data: to_dir_fh.to_vec()}, name: filename3(to_filename.to_string())},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Rename, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = RENAME3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse rename response"));
        }

        match x.unwrap().0 {
            RENAME3res::NFS3_OK(_) => Ok(()),
            RENAME3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
