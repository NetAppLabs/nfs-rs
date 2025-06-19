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

use crate::split_path;
use super::{
    diropargs3, filename3, nfs_fh3, Error, ErrorKind, Mount, RMDIR3args, RMDIR3res, Result,
};

impl Mount {
    pub fn rmdir_path(&self, path: &str) -> Result<()> {
        let (dir, dirname) = split_path(path)?;
        let dir_fh = self.lookup_path(&dir)?.fh;
        self.rmdir(&dir_fh, &dirname)
    }

    pub fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        let args = RMDIR3args {
            object: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(dirname.to_string()),
            },
        };
        match self._rmdir(args)? {
            RMDIR3res::NFS3_OK(_) => Ok(()),
            RMDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
