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

use std::io;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use nfs4::{
    FileAttributeId::{
        MaxLink as FileAttributeIdMaxLink,
        MaxName as FileAttributeIdMaxName,
        NoTrunc as FileAttributeIdNoTrunc,
        ChownRestricted as FileAttributeIdChownRestricted,
        CaseInsensitive as FileAttributeIdCaseInsensitive,
        CasePreserving as FileAttributeIdCasePreserving,
        Type as FileAttributeIdFileType,
        Mode as FileAttributeIdFileMode,
        NumLinks as FileAttributeIdNumLinks,
        Owner as FileAttributeIdOwner,
        OwnerGroup as FileAttributeIdOwnerGroup,
        Size as FileAttributeIdFileSize,
        SpaceUsed as FileAttributeIdSpaceUsed,
        RawDev as FileAttributeIdRawDev,
        FsId as FileAttributeIdFsId,
        FileId as FileAttributeIdFileId,
        FileHandle as FileAttributeIdFileHandle,
        TimeAccess as FileAttributeIdTimeAccess,
        TimeModify as FileAttributeIdTimeModify,
        TimeCreate as FileAttributeIdTimeCreate,
    },
    FileAttribute::{
        MaxLink as FileAttributeMaxLink,
        MaxName as FileAttributeMaxName,
        NoTrunc as FileAttributeNoTrunc,
        ChownRestricted as FileAttributeChownRestricted,
        CaseInsensitive as FileAttributeCaseInsensitive,
        CasePreserving as FileAttributeCasePreserving,
        Type as FileAttributeFileType,
        Mode as FileAttributeFileMode,
        NumLinks as FileAttributeNumLinks,
        Owner as FileAttributeOwner,
        OwnerGroup as FileAttributeOwnerGroup,
        Size as FileAttributeFileSize,
        SpaceUsed as FileAttributeSpaceUsed,
        RawDev as FileAttributeRawDev,
        FsId as FileAttributeFsId,
        FileId as FileAttributeFileId,
        FileHandle as FileAttributeFileHandle,
        TimeAccess as FileAttributeTimeAccess,
        TimeAccessSet as FileAttributeTimeAccessSet,
        TimeModify as FileAttributeTimeModify,
        TimeModifySet as FileAttributeTimeModifySet,
        TimeCreate as FileAttributeTimeCreate,
    },
    FileAttributes,
    Mode as FileMode,
    SetTime as FileSetTime,
};

use crate::{
    rpc, split_path, Error, NFSVersion, ObjRes, Result, SocketAddr, TcpStream, Time, ToSocketAddrs,
};

const ROOT_PATH: &str = "/";

macro_rules! get_value_from_file_attributes {
    ($file_attributes:ident, $attr_id:ident, $attr_type:ident, $from_value:ident) => {
        match $file_attributes.get($attr_id) {
            Some($attr_type(value)) => $from_value(&value),
            _ => Default::default(),
        }
    }
}

impl Into<nfs4_client::OpaqueAuth> for &crate::Auth {
    fn into(self) -> nfs4_client::OpaqueAuth {
        match self.flavor {
            crate::AuthFlavor::Null => nfs4_client::OpaqueAuth::none(),
            crate::AuthFlavor::Unix => nfs4_client::OpaqueAuth::auth_sys(nfs4_client::AuthSysParameters {
                stamp: 0,
                machine_name: "nfs-rs".into(),
                uid: nfs4_client::Uid(self.uid),
                gid: nfs4_client::Gid(self.gid),
                gids: Vec::new(),
            }),
        }
    }
}

impl Into<nfs4::Time> for crate::shared::Time {
    fn into(self) -> nfs4::Time {
        nfs4::Time{seconds: self.seconds as i64, nseconds: self.nseconds}
    }
}

impl Into<crate::mount::Pathconf> for nfs4::GetAttrRes {
    fn into(self) -> crate::mount::Pathconf {
        self.object_attributes.into()
    }
}

impl Into<crate::mount::Pathconf> for nfs4::FileAttributes {
    fn into(self) -> crate::mount::Pathconf {
        crate::mount::Pathconf {
            linkmax: get_value_from_file_attributes!(self, FileAttributeIdMaxLink, FileAttributeMaxLink, from_ref),
            name_max: get_value_from_file_attributes!(self, FileAttributeIdMaxName, FileAttributeMaxName, from_ref),
            no_trunc: get_value_from_file_attributes!(self, FileAttributeIdNoTrunc, FileAttributeNoTrunc, from_ref),
            chown_restricted: get_value_from_file_attributes!(self, FileAttributeIdChownRestricted, FileAttributeChownRestricted, from_ref),
            case_insensitive: get_value_from_file_attributes!(self, FileAttributeIdCaseInsensitive, FileAttributeCaseInsensitive, from_ref),
            case_preserving: get_value_from_file_attributes!(self, FileAttributeIdCasePreserving, FileAttributeCasePreserving, from_ref),
            attr: Some(self.into()),
        }
    }
}

impl Into<crate::mount::Attr> for nfs4::GetAttrRes {
    fn into(self) -> crate::mount::Attr {
        self.object_attributes.into()
    }
}

impl Into<crate::mount::Attr> for nfs4::FileAttributes {
    fn into(self) -> crate::mount::Attr {
        crate::mount::Attr{
            type_: get_value_from_file_attributes!(self, FileAttributeIdFileType, FileAttributeFileType, from_file_type),
            file_mode: get_value_from_file_attributes!(self, FileAttributeIdFileMode, FileAttributeFileMode, from_file_mode),
            nlink: get_value_from_file_attributes!(self, FileAttributeIdNumLinks, FileAttributeNumLinks, from_ref),
            uid: get_value_from_file_attributes!(self, FileAttributeIdOwner, FileAttributeOwner, from_uid_or_gid_string),
            gid: get_value_from_file_attributes!(self, FileAttributeIdOwnerGroup, FileAttributeOwnerGroup, from_uid_or_gid_string),
            filesize: get_value_from_file_attributes!(self, FileAttributeIdFileSize, FileAttributeFileSize, from_ref),
            used: get_value_from_file_attributes!(self, FileAttributeIdSpaceUsed, FileAttributeSpaceUsed, from_ref),
            spec_data: get_value_from_file_attributes!(self, FileAttributeIdRawDev, FileAttributeRawDev, from_device_data),
            fsid: get_value_from_file_attributes!(self, FileAttributeIdFsId, FileAttributeFsId, from_fs_id),
            fileid: get_value_from_file_attributes!(self, FileAttributeIdFileId, FileAttributeFileId, from_file_id),
            atime: get_value_from_file_attributes!(self, FileAttributeIdTimeAccess, FileAttributeTimeAccess, from_time),
            mtime: get_value_from_file_attributes!(self, FileAttributeIdTimeModify, FileAttributeTimeModify, from_time),
            ctime: get_value_from_file_attributes!(self, FileAttributeIdTimeCreate, FileAttributeTimeCreate, from_time),
        }
    }
}

impl Into<crate::mount::ReaddirEntry> for nfs4::DirectoryEntry {
    fn into(self) -> crate::mount::ReaddirEntry {
        let fileid = match self.attrs.get(FileAttributeIdFileId) {
            Some(FileAttributeFileId(value)) => from_file_id(value),
            _ => Default::default(),
        };
        crate::mount::ReaddirEntry{
            file_name: self.name.to_string(),
            fileid,
        }
    }
}

impl Into<crate::mount::ReaddirplusEntry> for nfs4::DirectoryEntry {
    fn into(self) -> crate::mount::ReaddirplusEntry {
        let attrs = self.attrs;
        let handle = get_value_from_file_attributes!(attrs, FileAttributeIdFileHandle, FileAttributeFileHandle, from_file_handle);
        let attr: crate::mount::Attr = attrs.into();
        let fileid = attr.fileid;
        crate::mount::ReaddirplusEntry{
            fileid,
            file_name: self.name.to_string(),
            attr: Some(attr),
            handle,
        }
    }
}

fn into_error(err: nfs4_client::Error) -> Error {
    match err { // FIXME: move error codes from crate::nfs3 to crate::shared?
        nfs4_client::Error::Protocol(nfs4::StatusError::Exist) => Error::new(io::ErrorKind::Other, crate::nfs3::ErrorCode::NFS3ERR_EXIST),
        nfs4_client::Error::Protocol(nfs4::StatusError::NoEnt) => Error::new(io::ErrorKind::Other, crate::nfs3::ErrorCode::NFS3ERR_NOENT),
        nfs4_client::Error::Protocol(nfs4::StatusError::NotEmpty) => Error::new(io::ErrorKind::Other, crate::nfs3::ErrorCode::NFS3ERR_NOTEMPTY),
        nfs4_client::Error::Protocol(nfs4::StatusError::Stale) => Error::new(io::ErrorKind::Other, crate::nfs3::ErrorCode::NFS3ERR_STALE),
        _ => Error::new(io::ErrorKind::Other, format!("received error: {:?}", err))
    } // TODO: cover more cases in order to consistently return error codes wherever possible?
}

fn from_ref<T: Clone>(value: &T) -> T {
    value.clone()
}

fn from_file_handle(file_handle: &nfs4::FileHandle) -> Vec<u8> {
    file_handle.0.to_vec()
}

fn from_file_type(file_type: &nfs4::FileType) -> u32 {
    match file_type {
        nfs4::FileType::Regular => 1,
        nfs4::FileType::Directory => 2,
        nfs4::FileType::Block => 3,
        nfs4::FileType::Character => 4,
        nfs4::FileType::Link => 5,
        nfs4::FileType::Socket => 6,
        nfs4::FileType::Fifo => 7,
        nfs4::FileType::AttrDir => 8,
    }
}

fn from_file_mode(file_mode: &nfs4::Mode) -> u32 {
    file_mode.0
}

fn from_uid_or_gid_string(uid_or_gid: &String) -> u32 {
    uid_or_gid.parse().unwrap_or_default()
}

fn from_device_data(dev_data: &nfs4::DeviceData) -> [u32; 2] {
    [dev_data.major, dev_data.minor]
}

fn from_fs_id(fsid: &nfs4::FsId) -> u64 {
    // FIXME: change type of crate::mount::Attr::fsid to [u64; 2]? (and just set to [fsid, 0] for NFSv3?)
    //        as is, below is incorrect and may cause two different file systems to be recognised as the
    //        same file system (though unlikely? - so perhaps this is the best we can do for now?)
    fsid.major | fsid.minor
}

fn from_file_id(fileid: &nfs4::FileId) -> u64 {
    fileid.0
}

fn from_time(time: &nfs4::Time) -> crate::shared::Time {
    crate::shared::Time{
        seconds: time.seconds as u32,
        nseconds: time.nseconds,
    }
}

fn from_read_link_res(read_link_res: nfs4::ReadLinkRes) -> String {
    read_link_res.link
}

fn from_read_res(read_res: nfs4::ReadRes) -> Vec<u8> {
    read_res.data
}

fn from_write_res(write_res: nfs4::WriteRes) -> u32 {
    write_res.count
}

pub(super) struct Mount4p1 {
    client: Arc<RwLock<nfs4_client::Client<TcpStream>>>,
    root_path: PathBuf,
    root_fh: Vec<u8>,
    rsize: u32,
    wsize: u32,
}

impl Mount4p1 {
    fn get_dir_fh_and_entry_name(&self, path: &str) -> Result<(Vec<u8>, String)> {
        let (dir_path, name) = split_path(path)?;
        self._lookup_path(&dir_path)
            .map(|fh| (fh, name))
    }

    fn get_obj_res_with_attrs(client: &mut nfs4_client::Client<TcpStream>, fh: &Vec<u8>) -> Result<ObjRes> {
        let attr = Self::_getattr(client, fh)?;
        Ok(ObjRes{
            attr: Some(attr),
            fh: fh.to_vec(),
        })
    }

    fn _getattr(client: &mut nfs4_client::Client<TcpStream>, fh: &Vec<u8>) -> Result<crate::mount::Attr> {
        let handle = nfs4::FileHandle(fh.to_vec());
        client.get_attr(handle)
            .map(Into::into)
            .map_err(into_error)
    }

    fn _lookup(client: &mut nfs4_client::Client<TcpStream>, dir_fh: &Vec<u8>, entry_name: &str) -> Result<Vec<u8>> {
        let cleaned = path_clean::clean(entry_name);
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        client.look_up_in(dir_handle, cleaned)
            .map(|handle| handle.0)
            .map_err(into_error)
    }

    fn _lookup_path(&self, path: &str) -> Result<Vec<u8>> {
        let cleaned = path_clean::clean(path);
        if let Some(cleaned_str) = cleaned.to_str() {
            if cleaned_str.is_empty() || cleaned_str == ROOT_PATH {
                return Ok(self.root_fh.to_vec())
            }
        }
        let file_path = if cleaned.is_absolute() {
            cleaned.strip_prefix("/").unwrap()
        } else {
            cleaned.as_path()
        };
        let mut client = self.client.write().unwrap();
        client.look_up(self.root_path.join(file_path))
            .map(|handle| handle.0)
            .map_err(into_error)
    }
}

impl core::fmt::Debug for Mount4p1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mount4p1")
            .field("root_path", &self.root_path)
            .field("root_fh", &self.root_fh)
            .field("rsize", &self.rsize)
            .field("wsize", &self.wsize)
            .finish()
    }
}

impl crate::Mount for Mount4p1 {
    fn get_max_read_size(&self) -> u32 {
        self.rsize
    }

    fn get_max_write_size(&self) -> u32 {
        self.wsize
    }

    fn null(&self) -> Result<()> {
        let mut client = self.client.write().unwrap();
        client.null()
            .map_err(into_error)
    }

    fn access(&self, fh: &Vec<u8>, mode: u32) -> Result<u32> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        client.access(handle, mode)
            .map(|res| res.access.bits())
            .map_err(into_error)
    }

    fn access_path(&self, path: &str, mode: u32) -> Result<u32> {
        self.access(&self._lookup_path(path)?, mode)
    }

    fn close(&self, _seqid: u32, _stateid: u64) -> Result<()> {
        unimplemented!("close")
    }

    fn commit(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<()> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        client.commit(handle, offset, count)
            .map(|_commit_res| ()) // FIXME: change trait to return verifier (u64?) from commit function?
            .map_err(into_error)
    }

    fn commit_path(&self, path: &str, offset: u64, count: u32) -> Result<()> {
        self.commit(&self._lookup_path(path)?, offset, count)
    }

    fn create(&self, dir_fh: &Vec<u8>, filename: &str, mode: u32) -> Result<ObjRes> {
        let mut client = self.client.write().unwrap();
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        let now = std::time::SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let mtime = nfs4::Time{seconds: since_epoch.as_secs() as i64, nseconds: since_epoch.subsec_nanos()};
        let mut attrs = FileAttributes::default();
        attrs.insert(FileAttributeFileMode(FileMode(mode)));
        attrs.insert(FileAttributeTimeModifySet(FileSetTime::SetToClientTime(mtime)));
        let handle = client.create_file(dir_handle, filename, attrs)
            .map_err(into_error)?;
        Self::get_obj_res_with_attrs(&mut client, &handle.0)
    }

    fn create_path(&self, path: &str, mode: u32) -> Result<ObjRes> {
        let (dir_fh, filename) = self.get_dir_fh_and_entry_name(path)?;
        self.create(&dir_fh, &filename, mode)
    }

    fn delegpurge(&self, _clientid: u64) -> Result<()> {
        unimplemented!("delegpurge")
    }

    fn delegreturn(&self, _stateid: u64) -> Result<()> {
        unimplemented!("delegreturn")
    }

    fn getattr(&self, fh: &Vec<u8>) -> Result<crate::mount::Attr> {
        let mut client = self.client.write().unwrap();
        Self::_getattr(&mut client, fh)
    }

    fn getattr_path(&self, path: &str) -> Result<crate::mount::Attr> {
        self.getattr(&self._lookup_path(path)?)
    }

    fn setattr(
        &self,
        fh: &Vec<u8>,
        guard_mtime: Option<Time>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Time>,
        mtime: Option<Time>,
    ) -> Result<()> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        let opts: Vec<nfs4::FileAttribute> = vec![
            mode.map(|m| FileAttributeFileMode(FileMode(m))),
            uid.map(|u| FileAttributeOwner(u.to_string())),
            gid.map(|g| FileAttributeOwnerGroup(g.to_string())),
            size.map(|s| FileAttributeFileSize(s)),
            atime.map(|a| FileAttributeTimeAccessSet(FileSetTime::SetToClientTime(a.into()))),
            mtime.map(|m| FileAttributeTimeModifySet(FileSetTime::SetToClientTime(m.into()))),
        ].into_iter().flatten().collect();
        let attrs = FileAttributes::from_iter(opts.into_iter());
        if let Some(guard_mtime) = guard_mtime {
            let verif_attrs = vec![FileAttributeTimeModify(guard_mtime.into())];
            client.set_attr_verified(handle, attrs, FileAttributes::from_iter(verif_attrs.into_iter()))
                .map_err(into_error)
        } else {
            client.set_attr(handle, attrs)
                .map_err(into_error)
        }
    }

    fn setattr_path(
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
        let obj_res = self.lookup_path(path)?;
        let guard_mtime = match (specify_guard, obj_res.attr) {
            (true, Some(attr)) => Some(attr.mtime.into()),
            (true, None) => self.getattr_path(path).map(|attr| Some(attr.mtime.into()))?,
            _ => None,
        };
        self.setattr(&obj_res.fh, guard_mtime, mode, uid, gid, size, atime, mtime)
    }

    fn getfh(&self) -> Result<()> {
        unimplemented!("getfh")
    }

    fn link(
        &self,
        src_fh: &Vec<u8>,
        dst_dir_fh: &Vec<u8>,
        dst_filename: &str,
    ) -> Result<crate::mount::Attr> {
        let mut client = self.client.write().unwrap();
        let src_handle = nfs4::FileHandle(src_fh.to_vec());
        let dst_dir_handle = nfs4::FileHandle(dst_dir_fh.to_vec());
        let _ = client.link(src_handle, dst_dir_handle, dst_filename)
            .map_err(into_error)?;
        let fh = Self::_lookup(&mut client, dst_dir_fh, dst_filename)?;
        Self::_getattr(&mut client, &fh)
    }

    fn link_path(&self, src_path: &str, dst_path: &str) -> Result<crate::mount::Attr> {
        let (dst_dir_fh, dst_filename) = self.get_dir_fh_and_entry_name(dst_path)?;
        self.link(&self._lookup_path(src_path)?, &dst_dir_fh, &dst_filename)
    }

    fn symlink(&self, src_path: &str, dst_dir_fh: &Vec<u8>, dst_filename: &str) -> Result<ObjRes> {
        let mut client = self.client.write().unwrap();
        let dst_dir_handle = nfs4::FileHandle(dst_dir_fh.to_vec());
        let mut attrs = FileAttributes::default();
        attrs.insert(FileAttributeFileMode(FileMode(0o777)));
        // XXX: unlike `client.create_file`, `client.create_link` does not need to have set modify time attr
        let handle = client.create_link(src_path, dst_dir_handle, dst_filename, attrs)
            .map_err(into_error)?;
        Self::get_obj_res_with_attrs(&mut client, &handle.0)
    }

    fn symlink_path(&self, src_path: &str, dst_path: &str) -> Result<ObjRes> {
        let (dst_dir_fh, dst_filename) = self.get_dir_fh_and_entry_name(dst_path)?;
        self.symlink(src_path, &dst_dir_fh, &dst_filename)
    }

    fn readlink(&self, fh: &Vec<u8>) -> Result<String> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        client.read_link(handle)
            .map(from_read_link_res)
            .map_err(into_error)
    }

    fn readlink_path(&self, path: &str) -> Result<String> {
        self.readlink(&self._lookup_path(path)?)
    }

    fn lookup(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<ObjRes> {
        let mut client = self.client.write().unwrap();
        let fh = Self::_lookup(&mut client, dir_fh, filename)?;
        Self::get_obj_res_with_attrs(&mut client, &fh)
    }

    fn lookup_path(&self, path: &str) -> Result<ObjRes> {
        let fh = self._lookup_path(path)?;
        let mut client = self.client.write().unwrap();
        Self::get_obj_res_with_attrs(&mut client, &fh)
    }

    fn pathconf(&self, fh: &Vec<u8>) -> Result<crate::mount::Pathconf> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        client.get_attr(handle)
            .map(Into::into)
            .map_err(into_error)
    }

    fn pathconf_path(&self, path: &str) -> Result<crate::mount::Pathconf> {
        self.pathconf(&self._lookup_path(path)?)
    }

    fn read(&self, fh: &Vec<u8>, offset: u64, count: u32) -> Result<Vec<u8>> {
        let mut client = self.client.write().unwrap();
        let handle = nfs4::FileHandle(fh.to_vec());
        client.read(handle, offset, count)
            .map(from_read_res)
            .map_err(into_error)
    }

    fn read_path(&self, path: &str, offset: u64, count: u32) -> Result<Vec<u8>> {
        self.read(&self._lookup_path(path)?, offset, count)
    }

    fn write(&self, fh: &Vec<u8>, offset: u64, data: &Vec<u8>) -> Result<u32> {
        let mut client = self.client.write().unwrap();
        let max_write_size = self.get_max_write_size();
        let data_len = data.len();
        let mut index = 0usize;
        let mut offset = offset;
        let mut remaining = data_len as u32;
        while remaining > 0 {
            let chunk_size = remaining.min(max_write_size) as usize;
            let end_index = index + chunk_size;
            let chunk = &data[index..end_index];
            let handle = nfs4::FileHandle(fh.to_vec());
            let write_size = client.write(handle, offset, chunk.to_vec())
                .map(from_write_res)
                .map_err(into_error)?;
            remaining -= write_size;
            offset += write_size as u64;
            index += write_size as usize;
        }
        Ok(data_len as u32)
    }

    fn write_path(&self, path: &str, offset: u64, data: &Vec<u8>) -> Result<u32> {
        self.write(&self._lookup_path(path)?, offset, data)
    }

    fn readdir(&self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirEntry>> {
        let mut client = self.client.write().unwrap();
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        let attr_ids = vec![FileAttributeIdFileId];
        let attrs = nfs4::EnumSet::<nfs4::FileAttributeId>::from_iter(attr_ids.into_iter());
        client.read_dir(dir_handle, attrs)
            .map(|entries| entries.into_iter().map(Into::into).collect())
            .map_err(into_error)
    }

    fn readdir_path(&self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirEntry>> {
        self.readdir(&self._lookup_path(dir_path)?)
    }

    fn readdirplus(&self, dir_fh: &Vec<u8>) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        let mut client = self.client.write().unwrap();
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        let attr_ids = vec![
            FileAttributeIdFileType,
            FileAttributeIdFileMode,
            FileAttributeIdNumLinks,
            FileAttributeIdOwner,
            FileAttributeIdOwnerGroup,
            FileAttributeIdFileSize,
            FileAttributeIdSpaceUsed,
            FileAttributeIdRawDev,
            FileAttributeIdFsId,
            FileAttributeIdFileId,
            FileAttributeIdFileHandle,
            FileAttributeIdTimeAccess,
            FileAttributeIdTimeModify,
            FileAttributeIdTimeCreate,
        ];
        let attrs = nfs4::EnumSet::<nfs4::FileAttributeId>::from_iter(attr_ids.into_iter());
        client.read_dir(dir_handle, attrs)
            .map(|entries| entries.into_iter().map(Into::into).collect())
            .map_err(into_error)
    }

    fn readdirplus_path(&self, dir_path: &str) -> Result<Vec<crate::mount::ReaddirplusEntry>> {
        self.readdirplus(&self._lookup_path(dir_path)?)
    }

    fn mkdir(&self, dir_fh: &Vec<u8>, dirname: &str, mode: u32) -> Result<ObjRes> {
        let mut client = self.client.write().unwrap();
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        let mut attrs = FileAttributes::default();
        attrs.insert(FileAttributeFileMode(FileMode(mode)));
        // XXX: unlike `client.create_file`, `client.create_directory` does not need to have set modify time attr
        let handle = client.create_directory(dir_handle, dirname, attrs)
            .map_err(into_error)?;
        Self::get_obj_res_with_attrs(&mut client, &handle.0)
    }

    fn mkdir_path(&self, path: &str, mode: u32) -> Result<ObjRes> {
        let (dir_fh, dirname) = self.get_dir_fh_and_entry_name(path)?;
        self.mkdir(&dir_fh, &dirname, mode)
    }

    fn remove(&self, dir_fh: &Vec<u8>, filename: &str) -> Result<()> {
        let mut client = self.client.write().unwrap();
        let dir_handle = nfs4::FileHandle(dir_fh.to_vec());
        client.remove(dir_handle, filename)
            .map(|_change_info| ()) // FIXME: change trait to return change info from remove function? (what would we do for NFSv3?)
            .map_err(into_error)
    }

    fn remove_path(&self, path: &str) -> Result<()> {
        let (dir_fh, filename) = self.get_dir_fh_and_entry_name(path)?;
        self.remove(&dir_fh, &filename)
    }

    fn rmdir(&self, dir_fh: &Vec<u8>, dirname: &str) -> Result<()> {
        self.remove(dir_fh, dirname) // XXX: NFSv4.1 has no rmdir operation, just remove (which works for both files and directories)
    }

    fn rmdir_path(&self, path: &str) -> Result<()> {
        let (dir_fh, dirname) = self.get_dir_fh_and_entry_name(path)?;
        self.rmdir(&dir_fh, &dirname)
    }

    fn rename(
        &self,
        from_dir_fh: &Vec<u8>,
        from_filename: &str,
        to_dir_fh: &Vec<u8>,
        to_filename: &str,
    ) -> Result<()> {
        let mut client = self.client.write().unwrap();
        let from_dir_handle = nfs4::FileHandle(from_dir_fh.to_vec());
        let to_dir_handle = nfs4::FileHandle(to_dir_fh.to_vec());
        client.rename(from_dir_handle, to_dir_handle, from_filename, to_filename)
            .map(|_rename_res| ()) // FIXME: change trait to return rename res (two change infos) from rename function? (what would we do for NFSv3?)
            .map_err(into_error)
    }

    fn rename_path(&self, from_path: &str, to_path: &str) -> Result<()> {
        let (from_dir_fh, from_filename) = self.get_dir_fh_and_entry_name(from_path)?;
        let (to_dir_fh, to_filename) = self.get_dir_fh_and_entry_name(to_path)?;
        self.rename(&from_dir_fh, &from_filename, &to_dir_fh, &to_filename)
    }

    fn umount(&self) -> Result<()> {
        unimplemented!("umount")
    }

    fn version(&self) -> NFSVersion {
        NFSVersion::NFSv4p1
    }
}

fn ensure_port(
    addrs: &Vec<SocketAddr>,
    port: u16,
    prog: u32,
    vers: u32,
    auth: &crate::Auth,
) -> Result<u16> {
    if port != 0 {
        return Ok(port);
    }
    rpc::portmap(addrs, prog, vers, auth)
}

pub(crate) fn mount(args: &crate::MountArgs) -> Result<Box<dyn crate::Mount>> {
    // start by resolving host address and assigning portmapper port to each resolved address
    let addrs = (args.host.as_str(), rpc::PORTMAP_PORT)
        .to_socket_addrs()?
        .collect();
    let auth = crate::Auth::new_unix("nfs-rs", args.uid, args.gid);
    let nfsport = ensure_port(
        &addrs,
        args.nfsport,
        rpc::NFS_PROG,
        rpc::NFS4_VERSION,
        &auth,
    )?;
    for mut addr in addrs {
        addr.set_port(nfsport); // replace portmapper port with NFS port obtained above
        let res = mount_on_addr(&addr, &args, &auth);
        if res.is_ok() {
            return Ok(res.unwrap());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "no valid socket address",
    ))
}

fn mount_on_addr(
    addr: &SocketAddr,
    args: &crate::MountArgs,
    auth: &crate::Auth,
) -> Result<Box<dyn crate::Mount>> {
    let tcp_stream = TcpStream::connect(addr)?;
    let mut client = nfs4_client::Client::new(tcp_stream, Some(auth.into()))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("nfs4_client::Client::new returned error: {:?}", e)))?;
    let root_path = path_clean::clean(&args.dirpath);
    let root_fh = client.look_up(&root_path)
        .map_err(into_error)?;
    let rsize = args.rsize.min(client.get_max_read_size() as u32);
    let wsize = args.wsize.min(client.get_max_write_size() as u32);
    Ok(Box::new(Mount4p1{client: Arc::new(RwLock::new(client)), root_path, root_fh: root_fh.0, rsize, wsize}))
}
