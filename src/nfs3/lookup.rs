use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, ObjRes};
use super::nfs3xdr::{LOOKUP3args, LOOKUP3resok, LOOKUP3res, diropargs3, filename3, nfs_fh3};
use crate::nfs3;

impl Mount {
    pub fn lookup_path(&mut self, path: &str) -> Result<ObjRes> {
        let mut res = Ok(ObjRes{
            fh: self.fh.to_vec(),
            attr: None,
        });
        for n in &path_clean::clean(path) {
            if res.as_mut().is_ok() && n != "" && n != "/" && n != "." {
                res = self.lookup(&res.as_mut().ok().unwrap().fh, &n.to_string_lossy());
            }
        }

        res
    }

    pub fn lookup(&mut self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes> {
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
            LOOKUP3res::NFS3_OK(ok) => ok.into(),
            LOOKUP3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<LOOKUP3resok> for Result<ObjRes> {
    fn from(value: LOOKUP3resok) -> Self {
        Ok(ObjRes{
            fh: value.object.data,
            attr: value.obj_attributes.into(),
        })
    }
}
