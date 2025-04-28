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

use super::{
    diropargs3, filename3, nfs_fh3, split_path, Error, ErrorKind, Mount, RENAME3args, RENAME3res,
    Result,
};

impl Mount {
    pub fn rename_path(&self, from: &str, to: &str) -> Result<()> {
        let (from_dir, from_filename) = split_path(from)?;
        let (to_dir, to_filename) = split_path(to)?;
        let is_same_dir = from_dir == to_dir;
        let from_dir_fh = self.lookup_path(&from_dir)?.fh;
        let to_dir_fh = if is_same_dir {
            from_dir_fh.to_vec()
        } else {
            self.lookup_path(&to_dir)?.fh
        };
        self.rename(&from_dir_fh, &from_filename, &to_dir_fh, &to_filename)
    }

    pub fn rename(
        &self,
        from_dir_fh: &Vec<u8>,
        from_filename: &str,
        to_dir_fh: &Vec<u8>,
        to_filename: &str,
    ) -> Result<()> {
        let args = RENAME3args {
            from: diropargs3 {
                dir: nfs_fh3 {
                    data: from_dir_fh.to_vec(),
                },
                name: filename3(from_filename.to_string()),
            },
            to: diropargs3 {
                dir: nfs_fh3 {
                    data: to_dir_fh.to_vec(),
                },
                name: filename3(to_filename.to_string()),
            },
        };
        match self._rename(args)? {
            RENAME3res::NFS3_OK(_) => Ok(()),
            RENAME3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
