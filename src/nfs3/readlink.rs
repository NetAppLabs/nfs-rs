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

use super::{nfs_fh3, Error, ErrorKind, Mount, READLINK3args, READLINK3res, Result};

impl Mount {
    pub fn readlink_path(&self, path: &str) -> Result<String> {
        self.readlink(&self.lookup_path(path)?.fh)
    }

    pub fn readlink(&self, fh: &Vec<u8>) -> Result<String> {
        let args = READLINK3args {
            symlink: nfs_fh3 { data: fh.to_vec() },
        };
        match self._readlink(args)? {
            READLINK3res::NFS3_OK(ok) => Ok(ok.data.0),
            READLINK3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
