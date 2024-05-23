use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, Pathconf};
use super::nfs3xdr::{PATHCONF3args, PATHCONF3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn pathconf_path(&self, path: &str) -> Result<Pathconf> {
        self.pathconf(&self.lookup_path(path)?.fh)
    }

    pub fn pathconf(&self, fh: &Vec<u8>) -> Result<Pathconf> {
        let args = PATHCONF3args{object: nfs_fh3{data: fh.to_vec()}};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Pathconf, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = PATHCONF3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse pathconf response"));
        }

        match x.unwrap().0 {
            PATHCONF3res::NFS3_OK(ok) => Ok(ok.into()),
            PATHCONF3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
