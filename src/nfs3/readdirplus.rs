use xdr_codec::Unpack;
use super::{Mount, Result, Error, ErrorKind, Fattr};
use super::nfs3xdr::{READDIRPLUS3args, READDIRPLUS3res, READDIRPLUS3resok, cookieverf3, dirlistplus3, entryplus3, nfs_fh3, post_op_attr, post_op_fh3};
use crate::nfs3;

#[derive(Debug)]
pub struct ReaddirplusEntry {
    pub fileid: u64,
    pub file_name: String,
    pub cookie: u64,
    pub attr: Option<Fattr>,
    pub handle: Vec<u8>,
}

impl From<&ReaddirplusEntry> for crate::mount::ReaddirplusEntry {
    fn from(entry: &ReaddirplusEntry) -> Self {
        Self{
            fileid: entry.fileid,
            file_name: entry.file_name.clone(),
            cookie: entry.cookie,
            attr: entry.attr.as_ref().map(|a| a.into()),
            handle: entry.handle.clone(),
        }
    }
}

impl Mount {
    pub fn readdirplus_path(&self, dir_path: &str) -> Result<Vec<ReaddirplusEntry>> {
        self.readdirplus(&self.lookup(dir_path)?)
    }

    pub fn readdirplus(&self, dir_fh: &Vec<u8>) -> Result<Vec<ReaddirplusEntry>> {
        let mut entries = Vec::new();
        let mut cookie = 0u64;
        let mut res = READDIRPLUS3resok{
            cookieverf: cookieverf3([0u8; 8]),
            dir_attributes: post_op_attr::FALSE,
            reply: dirlistplus3{entries: None, eof: false},
        };
        while !res.reply.eof {
            res = self.readdirplus_at(dir_fh, cookie, res.cookieverf)?;
            if let Some(entry) = res.reply.entries {
                cookie = self.readdirplus_entries(&mut entries, entry);
            }
        }
        Ok(entries)
    }

    fn readdirplus_at(&self, dir_fh: &Vec<u8>, cookie: u64, cookieverf: cookieverf3) -> Result<READDIRPLUS3resok> {
        let args = READDIRPLUS3args{
            dir: nfs_fh3{data: dir_fh.to_vec()},
            cookie,
            cookieverf,
            dircount: self.dircount,
            maxcount: self.maxcount,
        };
        let mut buf = Vec::<u8>::new();
        let res = self.pack_nfs3(nfs3::NFSProc3::Readdirplus, &args, &mut buf);
        if res.is_err() {
            return Err(Error::new(ErrorKind::Other, res.unwrap_err()));
        }

        let res = self.rpc.call(buf)?;
        let mut r = res.as_slice();
        let x = READDIRPLUS3res::unpack(&mut r);
        if x.is_err() {
            return Err(Error::new(ErrorKind::Other, "could not parse readdirplus response"));
        }

        match x.unwrap().0 {
            READDIRPLUS3res::NFS3_OK(y) => Ok(y),
            _ => Err(Error::new(ErrorKind::Other, "reading directory failed")),
        }
    }

    fn readdirplus_entries(&self, entries: &mut Vec<ReaddirplusEntry>, entry: Box<entryplus3>) -> u64 {
        entries.push(ReaddirplusEntry{
            fileid: entry.fileid,
            file_name: entry.name.0,
            cookie: entry.cookie,
            attr: match entry.name_attributes {
                post_op_attr::TRUE(a) => Some(a.into()),
                _ => None,
            },
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
