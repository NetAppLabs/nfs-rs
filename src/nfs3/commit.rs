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

use super::{nfs_fh3, COMMIT3args, COMMIT3res, Error, ErrorKind, Mount, Result};

impl Mount {
    pub fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.commit(&self.lookup_path(path)?.fh, offset, count)
    }

    pub fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        let args = COMMIT3args {
            file: nfs_fh3 { data: fh.to_vec() },
            offset,
            count,
        };
        match self._commit(args)? {
            COMMIT3res::NFS3_OK(_) => Ok(()),
            COMMIT3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
