use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, MKNOD3args, from_post_op_fh3, split_path, mknoddata3, devicedata3, sattr3, specdata3, set_size3};
use super::nfs3xdr::{MKNOD3res, diropargs3, nfs_fh3, filename3, set_mode3, set_uid3, set_gid3, set_atime, set_mtime};
use crate::nfs3;

impl Mount {
    #[allow(unused)]
    pub fn mknod_blk(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3BLK(devicedata3{
            dev_attributes: sattr3{
                mode: set_mode3::TRUE(0),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
            spec: specdata3{
                specdata1: 0,
                specdata2: 0,
            },
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_chr(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3CHR(devicedata3{
            dev_attributes: sattr3{
                mode: set_mode3::TRUE(0),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
            spec: specdata3{
                specdata1: 0,
                specdata2: 0,
            },
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_fifo(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3FIFO(sattr3{
            mode: set_mode3::TRUE(0),
            uid: set_uid3::default,
            gid: set_gid3::default,
            size: set_size3::default,
            atime: set_atime::default,
            mtime: set_mtime::default,
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_sock(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3SOCK(sattr3{
            mode: set_mode3::TRUE(0),
            uid: set_uid3::default,
            gid: set_gid3::default,
            size: set_size3::default,
            atime: set_atime::default,
            mtime: set_mtime::default,
        });
        self.mknod(path, what)
    }

    fn mknod(&self, path: &str, what: mknoddata3) -> Result<Vec<u8>> {
        let (dir, name) = split_path(path)?;
        let fh = self.lookup_path(dir.as_str())?;
        let args = MKNOD3args{
            where_: diropargs3{dir: nfs_fh3{data: fh}, name: filename3(name)},
            what,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Mknod, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = MKNOD3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse mknod response"));
        }

        match x.unwrap().0 {
            MKNOD3res::NFS3_OK(ok) => from_post_op_fh3(ok.obj),
            MKNOD3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
