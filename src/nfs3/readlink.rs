use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{READLINK3args, READLINK3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn readlink_path(&mut self, path: &str) -> Result<String> {
        let res = self.lookup_path(path)?;
        self.readlink(&res.fh)
    }

    pub fn readlink(&mut self, fh: &Vec<u8>) -> Result<String> {
        let args = READLINK3args{symlink: nfs_fh3{data: fh.to_vec()}};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Readlink, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = READLINK3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse readlink response"));
        }

        match x.unwrap().0 {
            READLINK3res::NFS3_OK(ok) => Ok(ok.data.0),
            READLINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
