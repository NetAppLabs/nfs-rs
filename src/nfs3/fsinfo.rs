use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, FSInfo};
use super::nfs3xdr::{FSINFO3args, FSINFO3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    #[allow(unused)]
    pub fn fsinfo(&mut self) -> Result<FSInfo> {
        let args = FSINFO3args{fsroot: nfs_fh3{data: self.fh.to_vec()}};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::FSInfo, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = FSINFO3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse fsinfo response"));
        }

        match x.unwrap().0 {
            FSINFO3res::NFS3_OK(ok) => Ok(ok.into()),
            FSINFO3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
