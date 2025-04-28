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
    diropargs3, filename3, from_post_op_fh3, nfs_fh3, sattr3, set_atime, set_gid3, set_mode3,
    set_mtime, set_size3, set_uid3, split_path, Error, ErrorKind, MKDIR3args, MKDIR3res,
    MKDIR3resok, Mount, ObjRes, Result,
};

impl Mount {
    pub fn mkdir_path(&self, path: &str, mode: u32) -> Result<ObjRes> {
        let (dir, dirname) = split_path(path)?;
        let dir_fh = self.lookup_path(&dir)?.fh;
        self.mkdir(&dir_fh, &dirname, mode)
    }

    pub fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes> {
        let args = MKDIR3args {
            where_: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(dirname.to_string()),
            },
            attrs: sattr3 {
                mode: set_mode3::TRUE(mode),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
        };
        match self._mkdir(args)? {
            MKDIR3res::NFS3_OK(ok) => ok.into(),
            MKDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<MKDIR3resok> for Result<ObjRes> {
    fn from(value: MKDIR3resok) -> Self {
        Ok(ObjRes {
            fh: from_post_op_fh3(value.obj)?,
            attr: value.obj_attributes.into(),
        })
    }
}
