use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, FSStat};
use super::nfs3xdr::{FSSTAT3args, FSSTAT3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    #[allow(unused)]
    pub fn fsstat(&self) -> Result<FSStat> {
        let args = FSSTAT3args{fsroot: nfs_fh3{data: self.fh.to_vec()}};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::FSStat, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = FSSTAT3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse fsstat response"));
        }

        match x.unwrap().0 {
            FSSTAT3res::NFS3_OK(ok) => Ok(ok.into()),
            _ => Err(Error::new(ErrorKind::Other, "getting file system stats failed")),
        }
    }
}
