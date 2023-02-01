use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, MKDIR3args, sattr3, set_size3, from_post_op_fh3};
use super::nfs3xdr::{MKDIR3res, diropargs3, filename3, nfs_fh3, set_mode3, set_uid3, set_gid3, set_atime, set_mtime};
use crate::nfs3;

impl Mount {
    pub fn mkdir_path(&self, path: &str, mode: u32) -> Result<Vec<u8>> {
        let (dir, dirname) = nfs3::split_path(path)?;
        let dir_fh = self.lookup(dir.as_str())?;
        self.mkdir(&dir_fh, dirname.as_str(), mode)
    }

    pub fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<Vec<u8>> {
        let args = MKDIR3args{
            where_: diropargs3{dir: nfs_fh3{data: dir_fh.to_vec()}, name: filename3(dirname.to_string())},
            attrs: sattr3{
                mode: set_mode3::TRUE(mode),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Mkdir, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = MKDIR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse mkdir response"));
        }

        match x.unwrap().0 {
            MKDIR3res::NFS3_OK(y) => from_post_op_fh3(y.obj),
            _ => Err(Error::new(ErrorKind::Other, "creating directory failed")),
        }
    }
}
