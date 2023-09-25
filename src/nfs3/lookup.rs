use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{LOOKUP3args, LOOKUP3resok, LOOKUP3res, diropargs3, filename3, nfs_fh3, post_op_attr};
use crate::nfs3;

impl Mount {
    pub fn lookup_path(&self, path: &str) -> Result<Vec<u8>> {
        Ok(self.lookup_path_raw(path)?.object.data)
    }

    pub fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<Vec<u8>> {
        Ok(self.lookup_raw(dir_fh, filename)?.object.data)
    }

    pub(crate) fn lookup_path_raw(&self, path: &str) -> Result<LOOKUP3resok> {
        let mut res = Ok(LOOKUP3resok{
            object: nfs_fh3{data: self.fh.to_vec()},
            dir_attributes: post_op_attr::FALSE,
            obj_attributes: post_op_attr::FALSE,
        });
        for n in &path_clean::clean(path) {
            if res.as_mut().is_ok() && n != "" && n != "/" && n != "." {
                res = self.lookup_raw(&res.as_mut().ok().unwrap().object.data, &n.to_string_lossy());
            }
        }

        res
    }

    fn lookup_raw(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<LOOKUP3resok> {
        let args = LOOKUP3args{
            what: diropargs3{
                dir: nfs_fh3{data: dir_fh.to_vec()},
                name: filename3(filename.to_string()),
            },
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Lookup, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = LOOKUP3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse lookup response"));
        }

        match x.unwrap().0 {
            LOOKUP3res::NFS3_OK(ok) => Ok(ok),
            LOOKUP3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
