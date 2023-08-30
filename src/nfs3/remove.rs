use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{REMOVE3args, REMOVE3res, diropargs3, filename3, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn remove_path(&self, path: &str) -> Result<()> {
        let (dir, filename) = nfs3::split_path(path)?;
        let dir_fh = self.lookup(dir.as_str())?;
        self.remove(&dir_fh, filename.as_str())
    }

    pub fn remove(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<()> {
        let args = REMOVE3args{
            object: diropargs3{dir: nfs_fh3{data: dir_fh.to_vec()}, name: filename3(filename.to_string())},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Remove, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = REMOVE3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse remove response"));
        }

        match x.unwrap().0 {
            REMOVE3res::NFS3_OK(_) => Ok(()),
            REMOVE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
