use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{READ3args, READ3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn read_path(&self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.read(&self.lookup(path)?, offset, count)
    }

    pub fn read(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>> {
        let args = READ3args{
            file: nfs_fh3{data: fh.to_vec()},
            offset,
            count,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Read, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = READ3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse read response"));
        }

        match x.unwrap().0 {
            READ3res::NFS3_OK(ok) => Ok(ok.data),
            READ3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
