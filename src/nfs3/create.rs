use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, CREATE3args, createhow3, sattr3, set_size3, from_post_op_fh3};
use super::nfs3xdr::{CREATE3res, diropargs3, filename3, nfs_fh3, set_mode3, set_uid3, set_gid3, set_atime, set_mtime};
use crate::nfs3;

impl Mount {
    pub fn create_path(&self, path: &str, mode: u32) -> Result<Vec<u8>> {
        let (dir, filename) = nfs3::split_path(path)?;
        self.create(&self.lookup_path(&dir)?, &filename, mode)
    }

    pub fn create(&self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<Vec<u8>> {
        let args = CREATE3args{
            where_: diropargs3{dir: nfs_fh3{data: dir_fh.to_vec()}, name: filename3(filename.to_string())},
            how: createhow3::UNCHECKED(sattr3{
                mode: set_mode3::TRUE(mode),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            }),
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Create, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = CREATE3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse create response"));
        }

        match x.unwrap().0 {
            CREATE3res::NFS3_OK(ok) => from_post_op_fh3(ok.obj),
            CREATE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
