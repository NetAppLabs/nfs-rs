use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind};
use super::nfs3xdr::{READDIR3args, READDIR3res, READDIR3resok, cookieverf3, dirlist3, entry3, nfs_fh3, post_op_attr};
use crate::nfs3;

#[derive(Debug)]
pub struct ReaddirEntry {
    pub fileid: u64,
    pub file_name: String,
    pub cookie: u64,
}

impl From<&ReaddirEntry> for crate::mount::ReaddirEntry {
    fn from(entry: &ReaddirEntry) -> Self {
        Self{
            fileid: entry.fileid,
            file_name: entry.file_name.clone(),
            cookie: entry.cookie,
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
        let mut res = READDIR3resok{
            cookieverf: cookieverf3([0u8; 8]),
            dir_attributes: post_op_attr::FALSE,
            reply: dirlist3{entries: None, eof: false},
        };
        while !res.reply.eof {
            res = self.readdir_at(dir_fh, cookie, res.cookieverf)?;
            if let Some(entry) = res.reply.entries {
                cookie = self.readdir_entries(&mut entries, entry);
            }
        }
        Ok(entries)
    }

    fn readdir_at(&self, dir_fh: &Vec<u8>, cookie: u64, cookieverf: cookieverf3) -> Result<READDIR3resok> {
        let args = READDIR3args{
            dir: nfs_fh3{data: dir_fh.to_vec()},
            cookie,
            cookieverf,
            count: 4096,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Readdir, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = READDIR3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse readdir response"));
        }

        match x.unwrap().0 {
            READDIR3res::NFS3_OK(ok) => Ok(ok),
            READDIR3res::default((e, _)) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn readdir_entries(&self, entries: &mut Vec<ReaddirEntry>, entry: Box<entry3>) -> u64 {
        entries.push(ReaddirEntry{
            fileid: entry.fileid,
            file_name: entry.name.0,
            cookie: entry.cookie,
        });
        if let Some(next) = entry.nextentry {
            return self.readdir_entries(entries, next);
        }
        entry.cookie
    }
}
