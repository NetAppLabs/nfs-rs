use super::{nfs_fh3, stable_how, Error, ErrorKind, Mount, Result, WRITE3args, WRITE3res};

impl Mount {
    pub fn write_path(&self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.write(&self.lookup_path(path)?.fh, offset, data)
    }

    pub fn write(&self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32> {
        if data.len() > u32::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "data length exceeds maximum",
            ));
        }
        let args = WRITE3args {
            file: nfs_fh3 { data: fh.to_vec() },
            stable: stable_how::FILE_SYNC,
            count: data.len() as u32,
            data: data.to_vec(),
            offset,
        };
        match self._write(args)? {
            WRITE3res::NFS3_OK(ok) => Ok(ok.count),
            WRITE3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))] // usize and u32 are the same for wasm32, so below test won't compile due to overflow in (u32::MAX as usize) + 1
mod tests {
    use super::*;

    #[test]
    fn mount_write_fh_data_exceeding_max_length() {
        let mount = Mount {
            rpc: crate::rpc::Client::new(0, None),
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
