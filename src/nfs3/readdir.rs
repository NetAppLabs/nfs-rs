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
    cookieverf3, dirlist3, entry3, nfs_fh3, post_op_attr, Error, ErrorKind, Mount, READDIR3args,
    READDIR3res, READDIR3resok, Result,
};

#[derive(Debug)]
pub struct ReaddirEntry {
    pub fileid: u64,
    pub file_name: String,
}

impl From<ReaddirEntry> for crate::mount::ReaddirEntry {
    fn from(entry: ReaddirEntry) -> Self {
        Self {
            fileid: entry.fileid,
            file_name: entry.file_name,
        }
    }
}

impl Mount {
    pub fn readdir_path(&self, dir_path: &str) -> Result<Vec<ReaddirEntry>> {
        self.readdir(&self.lookup_path(dir_path)?.fh)
    }

    pub fn readdir(&self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirEntry>> {
        let mut entries = Vec::new();
        let mut cookie = 0u64;
        let mut res = READDIR3resok {
            cookieverf: cookieverf3([0u8; 8]),
            dir_attributes: post_op_attr::FALSE,
            reply: dirlist3 {
                entries: None,
                eof: false,
            },
        };
        while !res.reply.eof {
            res = self.readdir_at(dir_fh, cookie, res.cookieverf)?;
            if let Some(entry) = res.reply.entries {
                cookie = self.readdir_entries(&mut entries, entry);
            }
        }
        Ok(entries)
    }

    fn readdir_at(
        &self,
        dir_fh: &Vec<u8>,
        cookie: u64,
        cookieverf: cookieverf3,
    ) -> Result<READDIR3resok> {
        let args = READDIR3args {
            dir: nfs_fh3 {
                data: dir_fh.to_vec(),
            },
            cookie,
            cookieverf,
            count: 4096,
        };
        match self._readdir(args)? {
            READDIR3res::NFS3_OK(ok) => Ok(ok),
            READDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn readdir_entries(&self, entries: &mut Vec<ReaddirEntry>, entry: Box<entry3>) -> u64 {
        entries.push(ReaddirEntry {
            fileid: entry.fileid,
            file_name: entry.name.0,
        });
        if let Some(next) = entry.nextentry {
            return self.readdir_entries(entries, next);
        }
        entry.cookie
    }
}
