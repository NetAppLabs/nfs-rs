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
    nfs_fh3, nfstime3, sattr3, sattrguard3, set_atime, set_gid3, set_mode3, set_mtime, set_size3,
    set_uid3, Error, ErrorKind, Mount, Result, SETATTR3args, SETATTR3res, Time,
};

impl Mount {
    pub fn setattr_path(
        &self,
        path: &str,
        specify_guard: bool,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Time>,
        mtime: Option<Time>,
    ) -> Result<()> {
        let res = self.lookup_path(path)?;
        let guard_ctime = match (specify_guard, res.attr) {
            (true, Some(attr)) => Some(Time {
                seconds: attr.ctime.seconds,
                nseconds: attr.ctime.nseconds,
            }),
            _ => None,
        };
        self.setattr(&res.fh, guard_ctime, mode, uid, gid, size, atime, mtime)
    }

    pub fn setattr(
        &self,
        fh: &Vec<u8>,
        guard_ctime: Option<Time>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Time>,
        mtime: Option<Time>,
    ) -> Result<()> {
        let args = SETATTR3args {
            object: nfs_fh3 { data: fh.to_vec() },
            new_attributes: sattr3 {
                mode: mode.map_or(set_mode3::default, |m| set_mode3::TRUE(m)),
                uid: uid.map_or(set_uid3::default, |u| set_uid3::TRUE(u)),
                gid: gid.map_or(set_gid3::default, |g| set_gid3::TRUE(g)),
                size: size.map_or(set_size3::default, |s| set_size3::TRUE(s)),
                atime: atime.map_or(set_atime::default, |a| {
                    set_atime::SET_TO_CLIENT_TIME(nfstime3 {
                        seconds: a.seconds,
                        nseconds: a.nseconds,
                    })
                }),
                mtime: mtime.map_or(set_mtime::default, |m| {
                    set_mtime::SET_TO_CLIENT_TIME(nfstime3 {
                        seconds: m.seconds,
                        nseconds: m.nseconds,
                    })
                }),
            },
            guard: match guard_ctime {
                Some(ctime) => sattrguard3::TRUE(nfstime3 {
                    seconds: ctime.seconds,
                    nseconds: ctime.nseconds,
                }),
                None => sattrguard3::FALSE,
            },
        };
        match self._setattr(args)? {
            SETATTR3res::NFS3_OK(_) => Ok(()),
            SETATTR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
