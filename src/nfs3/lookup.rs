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
    diropargs3, filename3, nfs_fh3, Error, ErrorKind, LOOKUP3args, LOOKUP3res, LOOKUP3resok, Mount,
    ObjRes, Result,
};

impl Mount {
    pub fn lookup_path(&self, path: &str) -> Result<ObjRes> {
        let mut res = Ok(ObjRes {
            fh: self.fh.to_vec(),
            attr: None,
        });
        for n in &path_clean::clean(path) {
            if res.as_mut().is_ok() && n != "" && n != "/" && n != "." {
                res = self.lookup(&res.as_mut().ok().unwrap().fh, &n.to_string_lossy());
            }
        }
        res
    }

    pub fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes> {
        let args = LOOKUP3args {
            what: diropargs3 {
                dir: nfs_fh3 {
                    data: dir_fh.to_vec(),
                },
                name: filename3(filename.to_string()),
            },
        };
        match self._lookup(args)? {
            LOOKUP3res::NFS3_OK(ok) => ok.into(),
            LOOKUP3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl From<LOOKUP3resok> for Result<ObjRes> {
    fn from(value: LOOKUP3resok) -> Self {
        Ok(ObjRes {
            fh: value.object.data,
            attr: value.obj_attributes.into(),
        })
    }
}
