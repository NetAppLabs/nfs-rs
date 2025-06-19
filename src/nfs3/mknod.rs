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
    devicedata3, diropargs3, filename3, from_post_op_fh3, mknoddata3, nfs_fh3, sattr3, set_atime,
    set_gid3, set_mode3, set_mtime, set_size3, set_uid3, specdata3, Error, ErrorKind, MKNOD3args,
    MKNOD3res, Mount, Result,
};

impl Mount {
    #[allow(unused)]
    pub fn mknod_blk(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3BLK(devicedata3 {
            dev_attributes: sattr3 {
                mode: set_mode3::TRUE(0),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
            spec: specdata3 {
                specdata1: 0,
                specdata2: 0,
            },
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_chr(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3CHR(devicedata3 {
            dev_attributes: sattr3 {
                mode: set_mode3::TRUE(0),
                uid: set_uid3::default,
                gid: set_gid3::default,
                size: set_size3::default,
                atime: set_atime::default,
                mtime: set_mtime::default,
            },
            spec: specdata3 {
                specdata1: 0,
                specdata2: 0,
            },
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_fifo(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3FIFO(sattr3 {
            mode: set_mode3::TRUE(0),
            uid: set_uid3::default,
            gid: set_gid3::default,
            size: set_size3::default,
            atime: set_atime::default,
            mtime: set_mtime::default,
        });
        self.mknod(path, what)
    }

    #[allow(unused)]
    pub fn mknod_sock(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: verify args
        let what = mknoddata3::NF3SOCK(sattr3 {
            mode: set_mode3::TRUE(0),
            uid: set_uid3::default,
            gid: set_gid3::default,
            size: set_size3::default,
            atime: set_atime::default,
            mtime: set_mtime::default,
        });
        self.mknod(path, what)
    }

    fn mknod(&self, path: &str, what: mknoddata3) -> Result<Vec<u8>> {
        let (dir, name) = split_path(path)?;
        let fh = self.lookup_path(&dir)?.fh;
        let args = MKNOD3args {
            where_: diropargs3 {
                dir: nfs_fh3 { data: fh },
                name: filename3(name),
            },
            what,
        };
        match self._mknod(args)? {
            MKNOD3res::NFS3_OK(ok) => from_post_op_fh3(ok.obj),
            MKNOD3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}
