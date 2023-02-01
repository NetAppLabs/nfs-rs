use super::{Mount, Result, Error, ErrorKind, NULL3args};
use crate::nfs3;

impl Mount {
    pub fn null(&self) -> Result<()> {
        let args = NULL3args{};
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Null, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let _ = self.rpc.call(buf)?;
        Ok(())
    }
}
