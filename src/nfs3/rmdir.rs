use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{RMDIR3args, RMDIR3res, diropargs3, filename3, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn rmdir_path(&self, path: &str) -> Result<()> {
        let (dir, dirname) = nfs3::split_path(path)?;
        let dir_fh = self.lookup_path(dir.as_str())?;
        self.rmdir(&dir_fh, dirname.as_str())
    }

    pub fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        let args = RMDIR3args{
            object: diropargs3{dir: nfs_fh3{data: dir_fh.to_vec()}, name: filename3(dirname.to_string())},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Rmdir, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = RMDIR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse rmdir response"));
        }

        match x.unwrap().0 {
            RMDIR3res::NFS3_OK(_) => Ok(()),
            RMDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
