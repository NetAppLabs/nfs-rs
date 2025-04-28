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
    cookieverf3, dirlistplus3, entryplus3, nfs_fh3, post_op_attr, post_op_fh3, Error, ErrorKind,
    Fattr, Mount, READDIRPLUS3args, READDIRPLUS3res, READDIRPLUS3resok, Result,
};

#[derive(Debug)]
pub struct ReaddirplusEntry {
    pub fileid: u64,
    pub file_name: String,
    pub attr: Option<Fattr>,
    pub handle: Vec<u8>,
}

impl From<ReaddirplusEntry> for crate::mount::ReaddirplusEntry {
    fn from(entry: ReaddirplusEntry) -> Self {
        Self {
            fileid: entry.fileid,
            file_name: entry.file_name,
            attr: entry.attr.map(Into::into),
            handle: entry.handle,
        }
    }
}

impl Mount {
    pub fn readdirplus_path(&self, dir_path: &str) -> Result<Vec<ReaddirplusEntry>> {
        self.readdirplus(&self.lookup_path(dir_path)?.fh)
    }

    pub fn readdirplus(&self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirplusEntry>> {
        let mut entries = Vec::new();
        let mut cookie = 0u64;
        let mut res = READDIRPLUS3resok {
            cookieverf: cookieverf3([0u8; 8]),
            dir_attributes: post_op_attr::FALSE,
            reply: dirlistplus3 {
                entries: None,
                eof: false,
            },
        };
        while !res.reply.eof {
            res = self.readdirplus_at(dir_fh, cookie, res.cookieverf)?;
            if let Some(entry) = res.reply.entries {
                cookie = self.readdirplus_entries(&mut entries, entry);
            }
        }
        Ok(entries)
    }

    fn readdirplus_at(
        &self,
        dir_fh: &Vec<u8>,
        cookie: u64,
        cookieverf: cookieverf3,
    ) -> Result<READDIRPLUS3resok> {
        let args = READDIRPLUS3args {
            dir: nfs_fh3 {
                data: dir_fh.to_vec(),
            },
            cookie,
            cookieverf,
            dircount: self.dircount,
            maxcount: self.maxcount,
        };
        match self._readdirplus(args)? {
            READDIRPLUS3res::NFS3_OK(ok) => Ok(ok),
            READDIRPLUS3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn readdirplus_entries(
        &self,
        entries: &mut Vec<ReaddirplusEntry>,
        entry: Box<entryplus3>,
    ) -> u64 {
        entries.push(ReaddirplusEntry {
            fileid: entry.fileid,
            file_name: entry.name.0,
            attr: entry.name_attributes.into(),
            handle: match entry.name_handle {
                post_op_fh3::TRUE(h) => h.data,
                _ => Vec::default(),
            },
        });
        if let Some(next) = entry.nextentry {
            return self.readdirplus_entries(entries, next);
        }
        entry.cookie
    }
}
