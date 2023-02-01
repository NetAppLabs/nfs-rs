use super::{Mount, Result, Error, ErrorKind, UMOUNT3args};
use super::mount3xdr::dirpath;
use crate::nfs3;

impl Mount {
    pub fn umount(&self) -> Result<()> {
        let args = UMOUNT3args{dirpath: dirpath(self.dir.to_owned())};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_mount3(nfs3::MountProc3::Umount, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let _ = self.rpc.call(buf)?;

        Ok(())
    }
}
