use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{WRITE3args, WRITE3res, nfs_fh3, stable_how};
use crate::nfs3;

impl Mount {
    pub fn write_path(&self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.write(&self.lookup(path)?, offset, data)
    }

    pub fn write(&self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32> {
        if data.len() > u32::MAX as usize {
            return Err(Error::new(ErrorKind::InvalidData, "data length exceeds maximum"));
        }
        let args = WRITE3args{
            file: nfs_fh3{data: fh.to_vec()},
            stable: stable_how::FILE_SYNC,
            count: data.len() as u32,
            data: data.to_vec(),
            offset,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Write, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = WRITE3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse write response"));
        }

        match x.unwrap().0 {
            WRITE3res::NFS3_OK(ok) => Ok(ok.count),
            WRITE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
#[cfg(not(target_os = "wasi"))]
mod tests {
    use super::*;

    #[test]
    fn mount_write_fh_data_exceeding_max_length() {
        let mount = Mount{
            rpc: crate::rpc::Client::new(None, None),
            auth: crate::rpc::auth::Auth::new_null(),
            dir: "/".to_string(),
            fh: Vec::new(),
            dircount: 512,
            maxcount: 4096,
        };
        let data = vec![0u8; (u32::MAX as usize) + 1];
        let res = mount.write(&Vec::new(), 0, &data).map_err(|e| e.kind());
        assert_eq!(res, Err(ErrorKind::InvalidData));
    }
}
