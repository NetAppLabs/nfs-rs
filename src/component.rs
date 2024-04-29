use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use wit_bindgen::rt::{RustResource, WasmResource};

use crate::{
    Attr,
    Mount,
    ReaddirplusEntry,
    Time,
    nfs3::ErrorCode as NFS3ErrorCode,
    nfs3::MountErrorCode as MOUNT3ErrorCode,
    parse_url_and_mount,
};
use crate::bindings::exports::wasi::cli::environment::Guest as ExportGuestEnvironment;
use crate::bindings::exports::wasi::cli::exit::Guest as ExportGuestExit;
use crate::bindings::exports::wasi::cli::stdin::Guest as ExportGuestStdin;
use crate::bindings::exports::wasi::cli::stdout::Guest as ExportGuestStdout;
use crate::bindings::exports::wasi::cli::stderr::Guest as ExportGuestStderr;
use crate::bindings::exports::wasi::clocks::wall_clock::{
    Guest as ExportGuestWallClock,
    Datetime as ExportDatetime,
};
use crate::bindings::exports::wasi::io::error::{
    Error as ExportError,
    Guest as ExportGuestIOError,
    GuestError as ExportGuestError,
};
use crate::bindings::exports::wasi::filesystem::{
    preopens::Guest as ExportGuestPreopens,
    preopens::Descriptor as ExportDescriptor,
    types::Advice as ExportAdvice,
    types::DescriptorBorrow as ExportDescriptorBorrow,
    types::DescriptorFlags as ExportDescriptorFlags,
    types::DescriptorStat as ExportDescriptorStat,
    types::DescriptorType as ExportDescriptorType,
    types::DirectoryEntry as ExportDirectoryEntry,
    types::DirectoryEntryStream as ExportDirectoryEntryStream,
    types::ErrorBorrow as ExportErrorBorrow,
    types::ErrorCode as ExportErrorCode,
    types::Filesize as ExportFilesize,
    types::Guest as ExportGuestTypes,
    types::GuestDescriptor as ExportGuestDescriptor,
    types::GuestDirectoryEntryStream as ExportGuestDirectoryEntryStream,
    types::InputStream as ExportInputStream,
    types::InputStreamBorrow as ExportInputStreamBorrow,
    types::MetadataHashValue as ExportMetadataHashValue,
    types::NewTimestamp as ExportNewTimestamp,
    types::OpenFlags as ExportOpenFlags,
    types::OutputStream as ExportOutputStream,
    types::PathFlags as ExportPathFlags,
};
use crate::bindings::exports::wasi::io::poll::{
    Guest as ExportGuestPoll,
    GuestPollable as ExportGuestPollable,
};
use crate::bindings::exports::wasi::io::streams::{
    Guest as ExportGuestStreams,
    GuestInputStream as ExportGuestInputStream,
    GuestOutputStream as ExportGuestOutputStream,
    Pollable as ExportPollable,
    PollableBorrow as ExportPollableBorrow,
    StreamError as ExportStreamError,
};
use crate::bindings::wasi::cli::exit::exit as import_exit;
use crate::bindings::wasi::cli::stdin::get_stdin as import_get_stdin;
use crate::bindings::wasi::cli::stdout::get_stdout as import_get_stdout;
use crate::bindings::wasi::cli::stderr::get_stderr as import_get_stderr;
use crate::bindings::wasi::cli::environment::{
    get_arguments as import_get_arguments,
    get_environment as import_get_environment,
    initial_cwd as import_initial_cwd,
};
use crate::bindings::wasi::clocks::wall_clock::{
    now as import_now,
    resolution as import_resolution,
};
use crate::bindings::wasi::io::streams::{
    InputStream as ImportInputStream,
    OutputStream as ImportOutputStream,
    StreamError as ImportStreamError,
};

const STDIN_HANDLE: u32 = 0;
const STDOUT_HANDLE: u32 = 1;
const STDERR_HANDLE: u32 = 2;
const FIRST_USABLE_HANDLE: u32 = 3; // FIXME: verify

pub(super) struct Component;

#[derive(Debug)]
pub struct Descriptor {
    mount_id: u32,
    fh: Vec<u8>,
    flags: u8,
    #[allow(unused)]
    path: String,
}

#[derive(Debug)]
pub struct DirectoryEntryStream {
    _handle: u32,
    entries: Vec<ReaddirplusEntry>,
}

#[derive(Debug)]
pub struct InputStream {
    mount_id: u32,
    fh: Vec<u8>,
    _handle: u32,
}

#[derive(Debug)]
pub struct OutputStream {
    mount_id: u32,
    fh: Vec<u8>,
    _handle: u32,
}

#[derive(Debug)]
enum StreamType {
    Input,
    Output,
}

#[derive(Debug)]
pub struct Pollable {
    stream_type: StreamType,
    stream_handle: u32,
}

static mut ERROR_MSGS: Option<Arc<RwLock<HashMap<u32, String>>>> = None;

fn get_error_msgs() -> &'static Arc<RwLock<HashMap<u32, String>>> {
    unsafe {
        if ERROR_MSGS.is_none() {
            ERROR_MSGS = Some(Arc::new(RwLock::new(HashMap::new())));
        }
        ERROR_MSGS.as_mut().unwrap()
    }
}

fn get_error_msg(handle: u32) -> String {
    let error_msgs = get_error_msgs().read().unwrap();
    error_msgs.get(&handle).map_or(String::new(), |error_msg| error_msg.clone())
}

fn add_error_msg(handle: u32, error_msg: String) {
    let mut error_msgs = get_error_msgs().write().unwrap();
    error_msgs.insert(handle, error_msg);
}

// FIXME: error messages are only ever added, which will "leak" memory
// fn remove_error_msg(handle: u32) {
//     let mut error_msgs = get_error_msgs().write().unwrap();
//     error_msgs.remove(&handle);
// }

static mut ERROR_CODDES: Option<Arc<RwLock<HashMap<u32, ExportErrorCode>>>> = None;

fn get_error_codes() -> &'static Arc<RwLock<HashMap<u32, ExportErrorCode>>> {
    unsafe {
        if ERROR_CODDES.is_none() {
            ERROR_CODDES = Some(Arc::new(RwLock::new(HashMap::new())));
        }
        ERROR_CODDES.as_mut().unwrap()
    }
}

fn get_error_code(handle: u32) -> Option<ExportErrorCode> {
    let error_codes = get_error_codes().read().unwrap();
    error_codes.get(&handle).map(|c| c.clone())
}

fn add_error_code(handle: u32, error_code: ExportErrorCode) {
    let mut error_codes = get_error_codes().write().unwrap();
    error_codes.insert(handle, error_code);
}

// FIXME: error codes are only ever added, which will "leak" memory
// fn remove_error_code(handle: u32) {
//     let mut error_codes = get_error_codes().write().unwrap();
//     error_codes.remove(&handle);
// }

#[derive(Debug)]
pub struct Error {
    _handle: u32,
}

impl Error {
    pub(crate) fn new<E>(kind: std::io::ErrorKind, error: E) -> std::io::Error
    where E: Into<Box<dyn std::error::Error + Send + Sync>> {
        let mut res = ExportError::new(Self{_handle: 0});
        let handle = res.handle();
        let err: &mut Error = res.get_mut();
        err._handle = handle;
        let mut ret = std::io::Error::new(kind, error);
        if let Some(inner_err) = ret.get_mut() {
            if let Some(code) = inner_err.is::<NFS3ErrorCode>().then_some(inner_err.downcast_mut::<NFS3ErrorCode>().map(|e| *e as i32))
                .or(inner_err.is::<MOUNT3ErrorCode>().then_some(inner_err.downcast_mut::<MOUNT3ErrorCode>().map(|e| *e as i32)))
                .map(|c| c.unwrap()) {
                add_error_code(handle, code.into());
            }
        }
        add_error_msg(handle, ret.to_string());
        ret
    }
}

static mut MOUNTS: Option<HashMap<u32, Arc<RwLock<Box<dyn Mount>>>>> = None;

fn get_mounts() -> &'static mut HashMap<u32, Arc<RwLock<Box<dyn Mount>>>> {
    unsafe {
        if MOUNTS.is_none() {
            MOUNTS = Some(HashMap::new());
        }
        MOUNTS.as_mut().unwrap()
    }
}

fn get_mount(mnt: u32) -> Result<&'static Arc<RwLock<Box<dyn Mount>>>, std::io::Error> {
    let mounts = get_mounts();
    let mount = mounts.get(&mnt);
    if mount.is_none() {
        return Err(Error::new(std::io::ErrorKind::NotFound, "mount not found"));
    }
    Ok(mount.unwrap())
}

fn add_mount(mount: Box<dyn Mount>) -> u32 {
    let mounts = get_mounts();
    let mut mnt = rand::random::<u32>();
    while mnt == 0 || mounts.contains_key(&mnt) {
        mnt = rand::random::<u32>();
    }
    mounts.insert(mnt, Arc::new(RwLock::new(mount)));
    mnt
}

// fn remove_mount(mnt: u32) {
//     let mounts = get_mounts();
//     mounts.remove(&mnt);
// }

fn get_mount_for_stream(mnt: u32) -> Result<&'static Arc<RwLock<Box<dyn Mount>>>, ExportStreamError> {
    get_mount(mnt).map_err(|_| ExportStreamError::Closed) // FIXME: verify error
}

fn get_mount_for_filesystem(mnt: u32) -> Result<&'static Arc<RwLock<Box<dyn Mount>>>, ExportErrorCode> {
    get_mount(mnt).map_err(|_| ExportErrorCode::BadDescriptor)
}

impl From<ImportStreamError> for ExportStreamError {
    fn from(err: ImportStreamError) -> Self {
        match err {
            ImportStreamError::Closed => ExportStreamError::Closed,
            ImportStreamError::LastOperationFailed(e) => {
                let e = unsafe { ExportError::from_handle(e.handle()) };
                ExportStreamError::LastOperationFailed(e) // FIXME: verify
            },
        }
    }
}

impl Into<ExportStreamError> for std::io::Error {
    fn into(self) -> ExportStreamError {
        let mut res = ExportError::new(Error{_handle: 0});
        let handle = res.handle();
        let err: &mut Error = res.get_mut();
        err._handle = handle;
        add_error_msg(handle, self.to_string());
        ExportStreamError::LastOperationFailed(res) // FIXME: verify
    }
}

impl Into<ExportErrorCode> for i32 {
    fn into(self) -> ExportErrorCode {
        match self {
            // x if x == NFS3ErrorCode::NFS3_OK as i32 => ExportErrorCode::Unsupported,
            x if x == NFS3ErrorCode::NFS3ERR_PERM as i32 => ExportErrorCode::NotPermitted,
            x if x == NFS3ErrorCode::NFS3ERR_NOENT as i32 => ExportErrorCode::NoEntry,
            x if x == NFS3ErrorCode::NFS3ERR_IO as i32 => ExportErrorCode::Io,
            x if x == NFS3ErrorCode::NFS3ERR_NXIO as i32 => ExportErrorCode::NoSuchDevice,
            x if x == NFS3ErrorCode::NFS3ERR_ACCES as i32 => ExportErrorCode::Access,
            x if x == NFS3ErrorCode::NFS3ERR_EXIST as i32 => ExportErrorCode::Exist,
            x if x == NFS3ErrorCode::NFS3ERR_XDEV as i32 => ExportErrorCode::CrossDevice,
            x if x == NFS3ErrorCode::NFS3ERR_NODEV as i32 => ExportErrorCode::NoDevice,
            x if x == NFS3ErrorCode::NFS3ERR_NOTDIR as i32 => ExportErrorCode::NotDirectory,
            x if x == NFS3ErrorCode::NFS3ERR_ISDIR as i32 => ExportErrorCode::IsDirectory,
            x if x == NFS3ErrorCode::NFS3ERR_INVAL as i32 => ExportErrorCode::Invalid,
            x if x == NFS3ErrorCode::NFS3ERR_FBIG as i32 => ExportErrorCode::FileTooLarge,
            x if x == NFS3ErrorCode::NFS3ERR_NOSPC as i32 => ExportErrorCode::InsufficientSpace,
            x if x == NFS3ErrorCode::NFS3ERR_ROFS as i32 => ExportErrorCode::ReadOnly,
            x if x == NFS3ErrorCode::NFS3ERR_MLINK as i32 => ExportErrorCode::TooManyLinks,
            x if x == NFS3ErrorCode::NFS3ERR_NAMETOOLONG as i32 => ExportErrorCode::NameTooLong,
            x if x == NFS3ErrorCode::NFS3ERR_NOTEMPTY as i32 => ExportErrorCode::NotEmpty,
            x if x == NFS3ErrorCode::NFS3ERR_DQUOT as i32 => ExportErrorCode::Quota,
            x if x == NFS3ErrorCode::NFS3ERR_STALE as i32 => ExportErrorCode::BadDescriptor, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_REMOTE as i32 => ExportErrorCode::NameTooLong, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_BADHANDLE as i32 => ExportErrorCode::BadDescriptor,
            x if x == NFS3ErrorCode::NFS3ERR_NOT_SYNC as i32 => ExportErrorCode::Interrupted, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_BAD_COOKIE as i32 => ExportErrorCode::InvalidSeek, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_NOTSUPP as i32 => ExportErrorCode::Unsupported,
            x if x == NFS3ErrorCode::NFS3ERR_TOOSMALL as i32 => ExportErrorCode::MessageSize, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_SERVERFAULT as i32 => ExportErrorCode::NotRecoverable, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_BADTYPE as i32 => ExportErrorCode::Unsupported, // FIXME: verify
            x if x == NFS3ErrorCode::NFS3ERR_JUKEBOX as i32 => ExportErrorCode::WouldBlock, // FIXME: verify
            _ => ExportErrorCode::Unsupported, // FIXME: verify
        }
    }
}

impl Into<ExportErrorCode> for std::io::Error {
    fn into(self) -> ExportErrorCode {
        let mut err = self;
        err.get_mut()
            .and_then(|inner_err| {
                if inner_err.is::<NFS3ErrorCode>() {
                    let nfs3code = inner_err.downcast_mut::<NFS3ErrorCode>().unwrap();
                    Some(*nfs3code as i32)
                } else if inner_err.is::<MOUNT3ErrorCode>() {
                    let mount3code = inner_err.downcast_mut::<MOUNT3ErrorCode>().unwrap();
                    Some(*mount3code as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
            .into()
    }
}

impl From<Attr> for ExportMetadataHashValue {
    fn from(attr: Attr) -> Self {
        let mut lower = DefaultHasher::new();
        attr.mtime.seconds.hash(&mut lower);
        attr.mtime.nseconds.hash(&mut lower);
        attr.fileid.hash(&mut lower);
        attr.gid.hash(&mut lower);
        attr.type_.hash(&mut lower);
        let mut upper = DefaultHasher::new();
        attr.ctime.seconds.hash(&mut upper);
        attr.ctime.nseconds.hash(&mut upper);
        attr.file_mode.hash(&mut upper);
        attr.filesize.hash(&mut upper);
        attr.uid.hash(&mut upper);
        Self{lower: lower.finish(), upper: upper.finish()}
    }
}

impl From<&ReaddirplusEntry> for ExportDirectoryEntry {
    fn from(entry: &ReaddirplusEntry) -> Self {
        Self{
            type_: entry.attr.as_ref().map_or(0, |a| a.type_).into(),
            name: entry.file_name.to_owned(),
        }
    }
}

impl From<Attr> for ExportDescriptorStat {
    fn from(attr: Attr) -> Self {
        Self{
            type_: attr.type_.into(),
            link_count: attr.nlink as u64,
            size: attr.filesize,
            data_access_timestamp: attr.atime.into(),
            data_modification_timestamp: attr.mtime.into(),
            status_change_timestamp: attr.ctime.into(),
        }
    }
}

impl Into<ExportDescriptorType> for u32 {
    fn into(self) -> ExportDescriptorType {
        match self {
            1 => ExportDescriptorType::RegularFile,
            2 => ExportDescriptorType::Directory,
            3 => ExportDescriptorType::BlockDevice,
            4 => ExportDescriptorType::CharacterDevice,
            5 => ExportDescriptorType::SymbolicLink,
            6 => ExportDescriptorType::Socket,
            7 => ExportDescriptorType::Fifo,
            _ => ExportDescriptorType::Unknown,
        }
    }
}

impl From<Time> for Option<ExportDatetime> {
    fn from(time: Time) -> Self {
        match (time.seconds as u64, time.nseconds) {
            (0, 0) => None,
            (seconds, nanoseconds) => Some(ExportDatetime{seconds, nanoseconds}),
        }
    }
}

impl From<ExportNewTimestamp> for Option<Time> {
    fn from(value: ExportNewTimestamp) -> Self {
        match value {
            ExportNewTimestamp::NoChange => None,
            ExportNewTimestamp::Timestamp(t) => Some(Time{seconds: (t.seconds as u32).wrapping_mul(1000), nseconds: t.nanoseconds}),
            ExportNewTimestamp::Now => {
                let now = std::time::SystemTime::now();
                let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                Some(Time{seconds: (since_epoch.as_secs() as u32).wrapping_mul(1000), nseconds: since_epoch.subsec_nanos()})
            },
        }
    }
}

impl Component {
    fn get_mount_id() -> u32 {
        import_get_environment()
            .iter()
            .find_map(|(key, value)| {
                match key.as_str() {
                    "NFS_URL_FOR_WASI_FILESYSTEM" => parse_url_and_mount(value)
                        .map(|mount| add_mount(mount))
                        .map_err(|e| panic!("failed to parse NFS URL and mount\nNFS URL: \"{}\"\nerror: {}", value, e.to_string()))
                        .ok(),
                    _ => None,
                }
            })
            .unwrap_or_else(|| panic!("no preopened directories have been set up"))
    }
}

impl ExportGuestPreopens for Component {
    fn get_directories() -> Vec<(ExportDescriptor, String)> {
        let mount_id = Self::get_mount_id();
        let res = get_mount(mount_id);
        if res.is_err() {
            panic!("preopened directory has become invalid");
        }
        let mount = res.unwrap().read().unwrap();
        let flags = (ExportDescriptorFlags::READ | ExportDescriptorFlags::WRITE | ExportDescriptorFlags::MUTATE_DIRECTORY).bits();
        let path = "/".to_string();
        let fh = mount.lookup_path(&path).unwrap();
        vec!((ExportDescriptor::new(Descriptor::new(mount_id, fh, flags, path, true)), "/".to_string()))
    }
}

impl ExportGuestTypes for Component {
    type Descriptor = Descriptor;

    type DirectoryEntryStream = DirectoryEntryStream;

    fn filesystem_error_code(err: ExportErrorBorrow<'_>) -> Option<ExportErrorCode> {
        let e = err.get::<Error>();
        get_error_code(e._handle).or_else(|| match e.to_debug_string().as_str() {
            // XXX: documentation for FilesystemError::to_debug_string() says it should only be used for
            //      debugging and the returned string should not be consumed mechanically
            //      however, since FilesystemError has no other functions or members, what other way is
            //      there to extract an ExportErrorCode, if get_error_code returns None?
            // FIXME: verify that FilesystemError::to_debug_string() actually returns error messages below
            "" => None, // FIXME: verify
            "Permission denied, similar to `EACCES` in POSIX." => Some(ExportErrorCode::Access),
            "Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX." => Some(ExportErrorCode::WouldBlock),
            "Connection already in progress, similar to `EALREADY` in POSIX." => Some(ExportErrorCode::Already),
            "Bad descriptor, similar to `EBADF` in POSIX." => Some(ExportErrorCode::BadDescriptor),
            "Device or resource busy, similar to `EBUSY` in POSIX." => Some(ExportErrorCode::Busy),
            "Resource deadlock would occur, similar to `EDEADLK` in POSIX." => Some(ExportErrorCode::Deadlock),
            "Storage quota exceeded, similar to `EDQUOT` in POSIX." => Some(ExportErrorCode::Quota),
            "File exists, similar to `EEXIST` in POSIX." => Some(ExportErrorCode::Exist),
            "File too large, similar to `EFBIG` in POSIX." => Some(ExportErrorCode::FileTooLarge),
            "Illegal byte sequence, similar to `EILSEQ` in POSIX." => Some(ExportErrorCode::IllegalByteSequence),
            "Operation in progress, similar to `EINPROGRESS` in POSIX." => Some(ExportErrorCode::InProgress),
            "Interrupted function, similar to `EINTR` in POSIX." => Some(ExportErrorCode::Interrupted),
            "Invalid argument, similar to `EINVAL` in POSIX." => Some(ExportErrorCode::Invalid),
            "I/O error, similar to `EIO` in POSIX." => Some(ExportErrorCode::Io),
            "Is a directory, similar to `EISDIR` in POSIX." => Some(ExportErrorCode::IsDirectory),
            "Too many levels of symbolic links, similar to `ELOOP` in POSIX." => Some(ExportErrorCode::Loop),
            "Too many links, similar to `EMLINK` in POSIX." => Some(ExportErrorCode::TooManyLinks),
            "Message too large, similar to `EMSGSIZE` in POSIX." => Some(ExportErrorCode::MessageSize),
            "Filename too long, similar to `ENAMETOOLONG` in POSIX." => Some(ExportErrorCode::NameTooLong),
            "No such device, similar to `ENODEV` in POSIX." => Some(ExportErrorCode::NoDevice),
            "No such file or directory, similar to `ENOENT` in POSIX." => Some(ExportErrorCode::NoEntry),
            "No locks available, similar to `ENOLCK` in POSIX." => Some(ExportErrorCode::NoLock),
            "Not enough space, similar to `ENOMEM` in POSIX." => Some(ExportErrorCode::InsufficientMemory),
            "No space left on device, similar to `ENOSPC` in POSIX." => Some(ExportErrorCode::InsufficientSpace),
            "Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX." => Some(ExportErrorCode::NotDirectory),
            "Directory not empty, similar to `ENOTEMPTY` in POSIX." => Some(ExportErrorCode::NotEmpty),
            "State not recoverable, similar to `ENOTRECOVERABLE` in POSIX." => Some(ExportErrorCode::NotRecoverable),
            "Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX." => Some(ExportErrorCode::Unsupported),
            "Inappropriate I/O control operation, similar to `ENOTTY` in POSIX." => Some(ExportErrorCode::NoTty),
            "No such device or address, similar to `ENXIO` in POSIX." => Some(ExportErrorCode::NoSuchDevice),
            "Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX." => Some(ExportErrorCode::Overflow),
            "Operation not permitted, similar to `EPERM` in POSIX." => Some(ExportErrorCode::NotPermitted),
            "Broken pipe, similar to `EPIPE` in POSIX." => Some(ExportErrorCode::Pipe),
            "Read-only file system, similar to `EROFS` in POSIX." => Some(ExportErrorCode::ReadOnly),
            "Invalid seek, similar to `ESPIPE` in POSIX." => Some(ExportErrorCode::InvalidSeek),
            "Text file busy, similar to `ETXTBSY` in POSIX." => Some(ExportErrorCode::TextFileBusy),
            "Cross-device link, similar to `EXDEV` in POSIX." => Some(ExportErrorCode::CrossDevice),
            _ => Some(ExportErrorCode::Unsupported), // FIXME: verify -- should this be None? (removing need for "" case above)
        })
    }
}

impl Descriptor {
    pub fn new(mount_id: u32, fh: Vec<u8>, flags: u8, path: String, is_dir: bool) -> Self {
        let path = match is_dir {
            true => format!("{}/", path.trim_end_matches('/')),
            false => path.trim_end_matches('/').to_string(),
        };
        Self{mount_id, fh, flags, path}
    }

    // fn expand_path(&self, path: &String) -> String {
    //     assert!(self.path.ends_with('/'));
    //     format!("{}{}", self.path, path)
    // }
}

unsafe impl RustResource for Descriptor {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for Descriptor {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:filesystem/types@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]descriptor"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle);
    }
}

impl ExportGuestDescriptor for Descriptor {
    // FIXME: documentation for `read_via_stream` says that multiple read, write, and append streams
    //        may be active on the same open file and they do not interfere with each other
    //        is that true for our implementation?
    fn read_via_stream(&self, offset: ExportFilesize) -> Result<ExportInputStream, ExportErrorCode> {
        // FIXME: error out if self is directory?
        // if self.get_type()? == DescriptorType::Directory {
        //     return Err(ExportErrorCode::IsDirectory);
        // }
        let mut res = ExportInputStream::new(InputStream{mount_id: self.mount_id, fh: self.fh.clone(), _handle: 0});
        let handle = res.handle();
        let ins: &mut InputStream = res.get_mut();
        ins._handle = handle;
        set_stream_offset(handle, offset);
        Ok(res)
    }

    fn write_via_stream(&self, offset: ExportFilesize) -> Result<ExportOutputStream, ExportErrorCode> {
        // FIXME: error out if self is directory?
        // if self.get_type()? == DescriptorType::Directory {
        //     return Err(ExportErrorCode::IsDirectory);
        // }
        let mut res = ExportOutputStream::new(OutputStream{mount_id: self.mount_id, fh: self.fh.clone(), _handle: 0});
        let handle = res.handle();
        let outs: &mut OutputStream = res.get_mut();
        outs._handle = handle;
        set_stream_offset(handle, offset);
        Ok(res)
    }

    fn append_via_stream(&self) -> Result<ExportOutputStream, ExportErrorCode> {
        // FIXME: error out if self is directory?
        // let stat = self.stat()?;
        // if stat.type_ == DescriptorType::Directory {
        //     return Err(ExportErrorCode::IsDirectory);
        // }
        // let offset = stat.size;
        let offset = self.stat().map(|s| s.size)?;
        let mut res = ExportOutputStream::new(OutputStream{mount_id: self.mount_id, fh: self.fh.clone(), _handle: 0});
        let handle = res.handle();
        let outs: &mut OutputStream = res.get_mut();
        outs._handle = handle;
        set_stream_offset(handle, offset);
        Ok(res)
     }

    fn advise(&self, _offset: ExportFilesize, _length: ExportFilesize, _advice: ExportAdvice) -> Result<(), ExportErrorCode> {
        Ok(()) // FIXME: verify
    }

    fn sync_data(&self) -> Result<(), ExportErrorCode> {
        Ok(())
    }

    fn get_flags(&self) -> Result<ExportDescriptorFlags, ExportErrorCode> {
        Ok(ExportDescriptorFlags::from_bits(self.flags).unwrap())
    }

    fn get_type(&self) -> Result<ExportDescriptorType, ExportErrorCode> {
        self.stat()
            .map(|s| s.type_)
    }

    fn set_size(&self, size: ExportFilesize) -> Result<(), ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.setattr(&self.fh, None, None, None, None, Some(size), None, None)
            .map_err(Into::into)
    }

    fn set_times(&self, data_access_timestamp: ExportNewTimestamp, data_modification_timestamp: ExportNewTimestamp) -> Result<(), ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let atime = data_access_timestamp.into();
        let mtime = data_modification_timestamp.into();
        mount.setattr(&self.fh, None, None, None, None, None, atime, mtime)
            .map_err(Into::into)
    }

    fn read(&self, length: ExportFilesize, offset: ExportFilesize) -> Result<(Vec<u8>, bool), ExportErrorCode> {
        if length > u32::MAX as ExportFilesize {
            return Err(ExportErrorCode::InsufficientMemory); // FIXME: verify
        }
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.read(&self.fh, offset, length as u32)
            .map(|buffer| {
                let len = buffer.len() as ExportFilesize;
                (buffer, len < length) // FIXME: verify that this really equates to EOF
            })
            .map_err(Into::into)
    }

    fn write(&self, buffer: Vec<u8>, offset: ExportFilesize) -> Result<ExportFilesize, ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.write(&self.fh, offset, &buffer)
            .map(|r| r as ExportFilesize)
            .map_err(Into::into)
    }

    fn read_directory(&self) -> Result<ExportDirectoryEntryStream, ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.readdirplus(&self.fh)
            .map(|entries| {
                let mut res = ExportDirectoryEntryStream::new(DirectoryEntryStream::new(entries));
                let handle = res.handle();
                let des: &mut DirectoryEntryStream = res.get_mut();
                des._handle = handle;
                set_directory_entry_stream_next_index(handle, &des.entries);
                res
            })
            .map_err(Into::into)
    }

    fn sync(&self) -> Result<(), ExportErrorCode> {
        Ok(())
    }

    fn create_directory_at(&self, path: String) -> Result<(), ExportErrorCode> {
        const MODE: u32 = 0o775;
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.mkdir(&self.fh, &path, MODE)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn stat(&self) -> Result<ExportDescriptorStat, ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.getattr(&self.fh)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn stat_at(&self, _path_flags: ExportPathFlags, path: String) -> Result<ExportDescriptorStat, ExportErrorCode> {
        // FIXME: ignoring path flags
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let fh = mount.lookup(&self.fh, &path)
            .map_err(Into::into)?;
        mount.getattr(&fh)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn set_times_at(&self, _path_flags: ExportPathFlags, path: String, data_access_timestamp: ExportNewTimestamp, data_modification_timestamp: ExportNewTimestamp) -> Result<(), ExportErrorCode> {
        // FIXME: ignoring path flags
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let atime = data_access_timestamp.into();
        let mtime = data_modification_timestamp.into();
        let fh = mount.lookup(&self.fh, &path)
            .map_err(Into::into)?;
        let attr = mount.getattr(&fh)
            .map_err(Into::into)?;
        mount.setattr(&fh, Some(attr.ctime), None, None, None, None, atime, mtime)
            .map_err(Into::into)
    }

    fn link_at(&self, _old_path_flags: ExportPathFlags, old_path: String, new_descriptor: ExportDescriptorBorrow<'_>, new_path: String) -> Result<(), ExportErrorCode> {
        // FIXME: ignoring path flags
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let fh = mount.lookup(&self.fh, &old_path)
            .map_err(Into::into)?;
        mount.link(&fh, &new_descriptor.get::<Descriptor>().fh, &new_path)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn open_at(&self, _path_flags: ExportPathFlags, path: String, open_flags: ExportOpenFlags, flags: ExportDescriptorFlags) -> Result<ExportDescriptor, ExportErrorCode> {
        // FIXME: ignoring path flags
        if let Some(own_flags) = ExportDescriptorFlags::from_bits(self.flags) {
            if !own_flags.contains(ExportDescriptorFlags::MUTATE_DIRECTORY) &&
                (open_flags.contains(ExportOpenFlags::CREATE) || open_flags.contains(ExportOpenFlags::TRUNCATE) || flags.contains(ExportDescriptorFlags::MUTATE_DIRECTORY)) {
                return Err(ExportErrorCode::ReadOnly);
            }
        }
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let mut created = false;
        let mut res = mount.lookup(&self.fh, &path);
        if res.is_err() {
            let err = res.unwrap_err();
            let err_code = err.into();
            if err_code != ExportErrorCode::NoEntry || !open_flags.contains(ExportOpenFlags::CREATE) {
                return Err(err_code);
            }

            if open_flags.contains(ExportOpenFlags::DIRECTORY) {
                // FIXME: verify that `open_at` should really create directory when open_flags.contains(OpenFlags::CREATE | OpenFlags::DIRECTORY)
                const MODE: u32 = 0o775;
                res = mount.mkdir(&self.fh, &path, MODE);
            } else {
                const MODE: u32 = 0o664;
                res = mount.create(&self.fh, &path, MODE);
            }
            if res.is_err() {
                let err = res.unwrap_err();
                return Err(err.into());
            }
            created = true;
        } else if open_flags.contains(ExportOpenFlags::EXCLUSIVE) {
            return Err(ExportErrorCode::Exist);
        }
        let fh = res.unwrap();
        let attr = mount.getattr(&fh)
            .map_err(Into::into)?;
        let type_: ExportDescriptorType = attr.type_.into();
        if !created && open_flags.contains(ExportOpenFlags::DIRECTORY) && type_ != ExportDescriptorType::Directory {
            return Err(ExportErrorCode::NotDirectory);
        }
        if !created && !open_flags.contains(ExportOpenFlags::DIRECTORY) && type_ == ExportDescriptorType::Directory {
            return Err(ExportErrorCode::IsDirectory);
        }
        if !created && open_flags.contains(ExportOpenFlags::TRUNCATE) {
            mount.setattr(&fh, None, None, None, None, Some(0), None, None)
                .map_err(Into::into)?;
        }
        Ok(ExportDescriptor::new(Descriptor::new(self.mount_id, fh, flags.bits(), path, type_ == ExportDescriptorType::Directory)))
    }

    fn readlink_at(&self, path: String) -> Result<String, ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let fh = mount.lookup(&self.fh, &path)
            .map_err(Into::into)?;
        mount.readlink(&fh)
            .map_err(Into::into)
    }

    fn remove_directory_at(&self, path: String) -> Result<(), ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.rmdir(&self.fh, &path)
            .map_err(Into::into)
    }

    fn rename_at(&self, old_path: String, new_descriptor: ExportDescriptorBorrow<'_>, new_path: String) -> Result<(), ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.rename(&self.fh, &old_path, &new_descriptor.get::<Descriptor>().fh, &new_path)
            .map_err(Into::into)
    }

    fn symlink_at(&self, old_path: String, new_path: String) -> Result<(), ExportErrorCode> {
        // XXX: documentation for `symlink_at` says to return ExportErrorCode::NotPermitted if old_path starts with /
        if old_path.starts_with("/") {
            return Err(ExportErrorCode::NotPermitted);
        }
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.symlink(&old_path, &self.fh, &new_path)
            .map(|_| ())
            .map_err(Into::into)
    }

    fn unlink_file_at(&self, path: String) -> Result<(), ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.remove(&self.fh, &path)
            .map_err(Into::into)
    }

    fn is_same_object(&self, other: ExportDescriptorBorrow<'_>) -> bool {
        let other_descriptor: &Descriptor = other.get();
        self.mount_id == other_descriptor.mount_id && self.fh == other_descriptor.fh
    }

    fn metadata_hash(&self) -> Result<ExportMetadataHashValue, ExportErrorCode> {
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        mount.getattr(&self.fh)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn metadata_hash_at(&self, _path_flags: ExportPathFlags, path: String) -> Result<ExportMetadataHashValue, ExportErrorCode> {
        // FIXME: ignoring path flags
        let mount = get_mount_for_filesystem(self.mount_id)?.read().unwrap();
        let fh = mount.lookup(&self.fh, &path)
            .map_err(Into::into)?;
        mount.getattr(&fh)
            .map(Into::into)
            .map_err(Into::into)
    }
}

static mut DIRECTORY_ENTRY_STREAM_INDICES: Option<Arc<RwLock<HashMap<u32, usize>>>> = None;

fn get_directory_entry_stream_indices() -> &'static Arc<RwLock<HashMap<u32, usize>>> {
    unsafe {
        if DIRECTORY_ENTRY_STREAM_INDICES.is_none() {
            DIRECTORY_ENTRY_STREAM_INDICES = Some(Arc::new(RwLock::new(HashMap::new())));
        }
        DIRECTORY_ENTRY_STREAM_INDICES.as_mut().unwrap()
    }
}

fn get_directory_entry_stream_index(handle: u32) -> Option<usize> {
    let indices = get_directory_entry_stream_indices().read().unwrap();
    indices.get(&handle).copied()
}

fn set_directory_entry_stream_next_index(handle: u32, entries: &Vec<ReaddirplusEntry>) {
    let mut indices = get_directory_entry_stream_indices().write().unwrap();
    let current_index = indices.remove(&handle);
    let mut next_index = current_index.map_or(0, |idx| idx + 1);
    while next_index < entries.len() && (entries[next_index].file_name == "." || entries[next_index].file_name == "..") {
        next_index += 1;
    }
    if next_index < entries.len() {
        indices.insert(handle, next_index);
    }
}

impl DirectoryEntryStream {
    pub fn new(entries: Vec<ReaddirplusEntry>) -> Self {
        Self{_handle: 0, entries}
    }
}

unsafe impl RustResource for DirectoryEntryStream {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for DirectoryEntryStream {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:filesystem/types@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]directory-entry-stream"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle);
    }
}

impl ExportGuestDirectoryEntryStream for DirectoryEntryStream {
    fn read_directory_entry(&self) -> Result<Option<ExportDirectoryEntry>, ExportErrorCode> {
        let handle: u32 = self._handle;
        let index = get_directory_entry_stream_index(handle);
        match index {
            None => Ok(None),
            Some(idx) => {
                set_directory_entry_stream_next_index(handle, &self.entries);
                Ok(Some((&self.entries[idx]).into()))
            },
        }
    }
}

static mut RESOURCES: Option<Arc<RwLock<HashMap<u32, usize>>>> = None;

fn get_resources() -> &'static Arc<RwLock<HashMap<u32, usize>>> {
    unsafe {
        if RESOURCES.is_none() {
            RESOURCES = Some(Arc::new(RwLock::new(HashMap::new())));
        }
        RESOURCES.as_mut().unwrap()
    }
}

fn get_resource(handle: u32) -> usize {
    let resources = get_resources().read().unwrap();
    resources.get(&handle).copied().unwrap_or_default()
}

fn add_resource(res: usize) -> u32 {
    let mut resources = get_resources().write().unwrap();
    let mut handle = rand::random::<u32>();
    while handle < FIRST_USABLE_HANDLE || resources.contains_key(&handle) {
        handle = rand::random::<u32>();
    }
    resources.insert(handle, res);
    handle
}

fn remove_resource(handle: u32) {
    let mut resources = get_resources().write().unwrap();
    resources.remove(&handle);
}

static mut STREAM_OFFSETS: Option<Arc<RwLock<HashMap<u32, u64>>>> = None;

fn get_stream_offsets() -> &'static Arc<RwLock<HashMap<u32, u64>>> {
    unsafe {
        if STREAM_OFFSETS.is_none() {
            STREAM_OFFSETS = Some(Arc::new(RwLock::new(HashMap::new())));
        }
        STREAM_OFFSETS.as_mut().unwrap()
    }
}

fn get_stream_offset(handle: u32) -> u64 {
    // FIXME: do offsets make sense for sources of type DescriptorType::Socket and DescriptorType::Fifo?
    //        if not, should this function always return zero for such sources?
    let offsets = get_stream_offsets().read().unwrap();
    offsets.get(&handle).copied().unwrap_or_default()
}

fn set_stream_offset(handle: u32, offset: u64) {
    let mut offsets = get_stream_offsets().write().unwrap();
    offsets.insert(handle, offset);
}

// FIXME: offsets are only ever set, which will "leak" memory
// fn remove_stream_offset(handle: u32) {
//     let mut offsets = get_stream_offsets().write().unwrap();
//     offsets.remove(&handle);
// }

impl InputStream {
    fn builtin(&self) -> Option<ImportInputStream> {
        self.fh.is_empty().then_some(import_get_stdin())
    }

    fn ready(&self) -> bool {
        if let Some(_stream) = self.builtin() {
            return true;
        }
        let res = self.peek(1);
        if res.is_err() {
            match res.unwrap_err() {
                ExportStreamError::Closed => true,
                _ => false,
            }
        } else {
            res.unwrap().len() > 0
        }
    }

    fn peek(&self, len: u32) -> Result<Vec<u8>, ExportStreamError> {
        // do pretty much same as `read(&self, len: u64)` but don't call `set_stream_offset`
        // FIXME: is this safe to do? what if source type is DescriptorType::Socket or DescriptorType::Fifo?
        //        does it even make sense to specify offset when reading from those, at all?
        let offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        mount.read(&self.fh, offset, len)
            .map_err(Into::into)
    }
}

unsafe impl RustResource for InputStream {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for InputStream {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:io/streams@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]input-stream"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle)
    }
}

impl ExportGuestInputStream for InputStream {
    fn read(&self, len: u64) -> Result<Vec<u8>, ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.read(len)
                .map_err(Into::into);
        }
        if len > u32::MAX as u64 {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "specified length exceeds maximum").into());
        }
        let offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        mount.read(&self.fh, offset, len as u32)
            .map(|buffer| {
                set_stream_offset(self._handle, offset + buffer.len() as u64);
                buffer
            })
            .map_err(Into::into)
    }

    fn blocking_read(&self, len: u64) -> Result<Vec<u8>, ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.blocking_read(len)
                .map_err(Into::into);
        }
        if len > u32::MAX as u64 {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "specified length exceeds maximum").into());
        }
        let offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        loop {
            let buffer = mount.read(&self.fh, offset, len as u32)
                .map_err(Into::<ExportStreamError>::into)?;
            if buffer.len() > 0 {
                set_stream_offset(self._handle, offset + buffer.len() as u64);
                return Ok(buffer);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    fn skip(&self, len: u64) -> Result<u64, ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.skip(len)
                .map_err(Into::into);
        }
        self.read(len)
            .map(|buffer| buffer.len() as u64)
    }

    fn blocking_skip(&self, len: u64) -> Result<u64, ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.blocking_skip(len)
                .map_err(Into::into);
        }
        self.blocking_read(len)
            .map(|buffer| buffer.len() as u64)
    }

    fn subscribe(&self) -> ExportPollable {
        ExportPollable::new(Pollable{stream_type: StreamType::Input, stream_handle: self._handle})
    }
}

impl OutputStream {
    fn builtin(&self) -> Option<ImportOutputStream> {
        match (self.fh.is_empty(), self._handle) {
            (true, STDOUT_HANDLE) => Some(import_get_stdout()),
            (true, STDERR_HANDLE) => Some(import_get_stderr()),
            _ => None,
        }
    }

    fn ready(&self) -> bool {
        match self.check_write() {
            Ok(c) => c > 0,
            Err(e) => match e {
                ExportStreamError::Closed => true,
                _ => false,
            },
        }
    }
}

unsafe impl RustResource for OutputStream {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for OutputStream {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:io/streams@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]output-stream"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle)
    }
}

impl ExportGuestOutputStream for OutputStream {
    fn check_write(&self) -> Result<u64, ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.check_write()
            .map_err(Into::into);
        }
        Ok(4096) // FIXME: verify
    }

    fn write(&self, contents: Vec<u8>) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.write(&contents)
                .map_err(Into::into);
        }
        let offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        mount.write(&self.fh, offset, &contents)
            .map(|count| { // FIXME: what if count < contents.len()?  should there be an error?  or loop?
                set_stream_offset(self._handle, offset + count as u64);
                ()
            })
            .map_err(Into::into)
    }

    fn blocking_write_and_flush(&self, mut contents: Vec<u8>) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.blocking_write_and_flush(&contents)
                .map_err(Into::into);
        }
        // XXX: implementation for blocking_write_and_flush taken from pseudo-code in documentation for trait function
        let mut offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        let pollable = self.subscribe();
        while !contents.is_empty() {
            pollable.get::<Pollable>().block();
            let n = self.check_write()?; // FIXME: replace call to self.check_write() with equivalent if `check_write(&self)` ever does anything (i.e. calls get_mount_for_stream)
            let len = contents.len().min(n as usize);
            let (chunk, rest) = contents.split_at(len);
            let _ = mount.write(&self.fh, offset, &chunk.to_vec())
                .map(|count| { // FIXME: what if count < chunk.len()?  should there be an error?  or stuff chunk[count..] and rest together?
                    offset += count as u64;
                    set_stream_offset(self._handle, offset);
                })
                .map_err(Into::<ExportStreamError>::into)?;
            contents = rest.to_vec();
        }
        let _ = self.flush()?; // FIXME: replace call to self.flush() with equivalent if `flush(&self)` ever does anything (i.e. calls get_mount_for_stream)
        pollable.get::<Pollable>().block();
        self.check_write() // FIXME: replace call to self.check_write() with equivalent if `check_write(&self)` ever does anything (i.e. calls get_mount_for_stream)
            .map(|_| ())
    }

    fn flush(&self) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.flush()
                .map_err(Into::into);
        }
        Ok(()) // FIXME: verify
    }

    fn blocking_flush(&self) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.blocking_flush()
                .map_err(Into::into);
        }
        Ok(()) // FIXME: verify
    }

    fn subscribe(&self) -> ExportPollable {
        ExportPollable::new(Pollable{stream_type: StreamType::Output, stream_handle: self._handle})
    }

    fn write_zeroes(&self, len: u64) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.write_zeroes(len)
                .map_err(Into::into);
        }
        self.write(vec![0u8; len as usize])
    }

    fn blocking_write_zeroes_and_flush(&self, len: u64) -> Result<(), ExportStreamError> {
        if let Some(stream) = self.builtin() {
            return stream.blocking_write_zeroes_and_flush(len)
                .map_err(Into::into);
        }
        self.blocking_write_and_flush(vec![0u8; len as usize])
    }

    fn splice(&self, src: ExportInputStreamBorrow<'_>, len: u64) -> Result<u64, ExportStreamError> {
        // if let Some(stream) = self.builtin() {
        //     return stream.splice(src, len)
        //         .map_err(Into::into);
        // }
        let src_stream = src.get::<InputStream>();
        let n = self.check_write()?;
        let count = len.min(n);
        let contents = src_stream.read(count)?;
        let offset = get_stream_offset(self._handle);
        let mount = get_mount_for_stream(self.mount_id)?.read().unwrap();
        mount.write(&self.fh, offset, &contents)
            .map(|count| {
                let count = count as u64;
                set_stream_offset(self._handle, offset + count);
                count
            })
            .map_err(Into::into)
    }

    fn blocking_splice(&self, src: ExportInputStreamBorrow<'_>, len: u64) -> Result<u64, ExportStreamError> {
        // if let Some(stream) = self.builtin() {
        //     return stream.blocking_splice(src, len)
        //         .map_err(Into::into);
        // }
        let write_pollable = self.subscribe();
        let wpoll = write_pollable.get::<Pollable>();
        let read_pollable = src.get::<InputStream>().subscribe();
        let rpoll = read_pollable.get::<Pollable>();
        while !wpoll.ready() || !rpoll.ready() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        self.splice(src, len)
    }
}

impl ExportGuestPoll for Component {
    type Pollable = Pollable;

    fn poll(in_: Vec<ExportPollableBorrow<'_>>) -> Vec<u32> {
        in_
            .iter()
            .enumerate()
            .filter(|(_, &ref p)| p.get::<Pollable>().ready())
            .map(|(index, _)| index as u32)
            .collect()
    }
}

unsafe impl RustResource for Pollable {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for Pollable {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:io/streams@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]pollable"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle)
    }
}

impl ExportGuestPollable for Pollable {
    fn ready(&self) -> bool {
        match self.stream_type {
            StreamType::Input => unsafe { ExportInputStream::from_handle(self.stream_handle).get::<InputStream>().ready() },
            StreamType::Output => unsafe { ExportOutputStream::from_handle(self.stream_handle).get::<OutputStream>().ready() },
        }
    }

    fn block(&self) {
        while !self.ready() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

unsafe impl RustResource for Error {
    unsafe fn new(rep: usize) -> u32 {
        add_resource(rep)
    }

    unsafe fn rep(handle: u32) -> usize {
        get_resource(handle)
    }
}

unsafe impl WasmResource for Error {
    unsafe fn drop(handle: u32) {
        #[link(wasm_import_module = "[export]wasi:io/error@0.2.0")]
        extern "C" {
            #[link_name = "[resource-drop]error"]
            fn drop(_: u32);
        }

        drop(handle);
        remove_resource(handle)
    }
}

impl ExportGuestError for Error {
    fn to_debug_string(&self) -> String {
        get_error_msg(self._handle)
    }
}

impl ExportGuestEnvironment for Component {
    fn get_environment() -> Vec<(String, String)> {
        import_get_environment()
    }

    fn get_arguments() -> Vec<String> {
        import_get_arguments()
    }

    fn initial_cwd() -> Option<String> {
        import_initial_cwd()
    }
}

impl ExportGuestExit for Component {
    fn exit(status: Result<(), ()>) {
        import_exit(status)
    }
}

impl ExportGuestIOError for Component {
    type Error = Error;
}

impl ExportGuestStreams for Component {
    type InputStream = InputStream;

    type OutputStream = OutputStream;
}

impl ExportGuestStdin for Component {
    fn get_stdin() -> ExportInputStream {
        ExportInputStream::new(InputStream{mount_id: 0, fh: Vec::new(), _handle: STDIN_HANDLE})
    }
}

impl ExportGuestStdout for Component {
    fn get_stdout() -> ExportOutputStream {
        ExportOutputStream::new(OutputStream{mount_id: 0, fh: Vec::new(), _handle: STDOUT_HANDLE})
    }
}

impl ExportGuestStderr for Component {
    fn get_stderr() -> ExportOutputStream {
        ExportOutputStream::new(OutputStream{mount_id: 0, fh: Vec::new(), _handle: STDERR_HANDLE})
    }
}

impl ExportGuestWallClock for Component {
    fn now() -> ExportDatetime {
        let res = import_now();
        ExportDatetime{seconds: res.seconds, nanoseconds: res.nanoseconds}
    }

    fn resolution() -> ExportDatetime {
        let res = import_resolution();
        ExportDatetime{seconds: res.seconds, nanoseconds: res.nanoseconds}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_new_path_handling() {
        struct TestData {
            path: &'static str,
            is_dir: bool,
            expected_path: &'static str,
        }
        let test_data = [
            TestData{path: "", is_dir: true, expected_path: "/"},
            TestData{path: "/subdir", is_dir: true, expected_path: "/subdir/"},
            TestData{path: "/some/deeper/subdir", is_dir: true, expected_path: "/some/deeper/subdir/"},
            TestData{path: "/file", is_dir: false, expected_path: "/file"},
            TestData{path: "/some/deeper/file", is_dir: false, expected_path: "/some/deeper/file"},
        ];
        for td in test_data {
            for suffix in ["", "/", "//", "///"] { // for verifying correct trimming of '/' characters from end of path
                let descriptor = Descriptor::new(0, Vec::new(), 0, format!("{}{}", td.path, suffix), td.is_dir);
                assert_eq!(descriptor.path, td.expected_path);
            }
        }
    }
}
