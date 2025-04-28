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

use super::{nfs_fh3, Error, ErrorKind, FSINFO3args, FSINFO3res, FSInfo, Mount, Result};

impl Mount {
    #[allow(unused)]
    pub fn fsinfo(&self) -> Result<FSInfo> {
        let args = FSINFO3args {
            fsroot: nfs_fh3 {
                data: self.fh.to_vec(),
            },
        };
        match self._fsinfo(args)? {
            FSINFO3res::NFS3_OK(ok) => Ok(ok.into()),
            FSINFO3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
