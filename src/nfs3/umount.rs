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

use super::mount3xdr::dirpath;
use super::{Error, ErrorKind, Mount, MountProc3, Result, UMOUNT3args};

impl Mount {
    pub fn umount(&self) -> Result<()> {
        let args = UMOUNT3args {
            dirpath: dirpath(self.dir.to_owned()),
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_mount3(MountProc3::Umount, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let _ = self.rpc.call(buf)?;

        Ok(())
    }
}
