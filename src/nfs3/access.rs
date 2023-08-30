use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{ACCESS3args, ACCESS3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn access_path(&self, path: &str, mode: u32) -> Result<u32> {
        self.access(&self.lookup(path)?, mode)
    }

    pub fn access(&self, fh: &Vec<u8>, mode: u32) -> Result<u32> {
        let args = ACCESS3args{object: nfs_fh3{data: fh.to_vec()}, access: mode};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Access, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = ACCESS3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse access response"));
        }

        match x.unwrap().0 {
            ACCESS3res::NFS3_OK(ok) => Ok(ok.access),
            ACCESS3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
