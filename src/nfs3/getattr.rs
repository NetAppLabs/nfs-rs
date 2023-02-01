use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, Fattr};
use super::nfs3xdr::{GETATTR3args, GETATTR3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn getattr_path(&self, path: &str) -> Result<Fattr> {
        self.getattr(&self.lookup(path)?)
    }

    pub(crate) fn getattr(&self, fh: &Vec<u8>) -> Result<Fattr> {
        let args = GETATTR3args{
            object: nfs_fh3{data: fh.to_vec()},
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::GetAttr, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = GETATTR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse getattr response"));
        }

        match x.unwrap().0 {
            GETATTR3res::NFS3_OK(ok) => Ok(ok.obj_attributes.into()),
            _ => Err(Error::new(ErrorKind::Other, "getting attributes failed")),
        }
    }
}
