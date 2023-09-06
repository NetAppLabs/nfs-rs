use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{COMMIT3args, COMMIT3res, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.commit(&self.lookup_path(path)?, offset, count)
    }

    pub fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        let args = COMMIT3args{
            file: nfs_fh3{data: fh.to_vec()},
            offset,
            count,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Commit, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = COMMIT3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse commit response"));
        }

        match x.unwrap().0 {
            COMMIT3res::NFS3_OK(_) => Ok(()),
            COMMIT3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
