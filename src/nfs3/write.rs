// Copyright 2025 NetApp Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

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
            rsize: 8192,
            wsize: 16384,
        };
        let data = vec![0u8; (u32::MAX as usize) + 1];
        let res = mount.write(&Vec::new(), 0, &data).map_err(|e| e.kind());
        assert_eq!(res, Err(ErrorKind::InvalidData));
    }
}
