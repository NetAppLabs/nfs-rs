pub type Errno = u16;
/// No error occurred. System call completed successfully.
pub const ERRNO_SUCCESS: Errno = 0;
/// Argument list too long.
pub const ERRNO_2BIG: Errno = 1;
/// Permission denied.
pub const ERRNO_ACCES: Errno = 2;
/// Address in use.
pub const ERRNO_ADDRINUSE: Errno = 3;
/// Address not available.
pub const ERRNO_ADDRNOTAVAIL: Errno = 4;
/// Address family not supported.
pub const ERRNO_AFNOSUPPORT: Errno = 5;
/// Resource unavailable, or operation would block.
pub const ERRNO_AGAIN: Errno = 6;
/// Connection already in progress.
pub const ERRNO_ALREADY: Errno = 7;
/// Bad file descriptor.
pub const ERRNO_BADF: Errno = 8;
/// Bad message.
pub const ERRNO_BADMSG: Errno = 9;
/// Device or resource busy.
pub const ERRNO_BUSY: Errno = 10;
/// Operation canceled.
pub const ERRNO_CANCELED: Errno = 11;
/// No child processes.
pub const ERRNO_CHILD: Errno = 12;
/// Connection aborted.
pub const ERRNO_CONNABORTED: Errno = 13;
/// Connection refused.
pub const ERRNO_CONNREFUSED: Errno = 14;
/// Connection reset.
pub const ERRNO_CONNRESET: Errno = 15;
/// Resource deadlock would occur.
pub const ERRNO_DEADLK: Errno = 16;
/// Destination address required.
pub const ERRNO_DESTADDRREQ: Errno = 17;
/// Mathematics argument out of domain of function.
pub const ERRNO_DOM: Errno = 18;
/// Reserved.
pub const ERRNO_DQUOT: Errno = 19;
/// File exists.
pub const ERRNO_EXIST: Errno = 20;
/// Bad address.
pub const ERRNO_FAULT: Errno = 21;
/// File too large.
pub const ERRNO_FBIG: Errno = 22;
/// Host is unreachable.
pub const ERRNO_HOSTUNREACH: Errno = 23;
/// Identifier removed.
pub const ERRNO_IDRM: Errno = 24;
/// Illegal byte sequence.
pub const ERRNO_ILSEQ: Errno = 25;
/// Operation in progress.
pub const ERRNO_INPROGRESS: Errno = 26;
/// Interrupted function.
pub const ERRNO_INTR: Errno = 27;
/// Invalid argument.
pub const ERRNO_INVAL: Errno = 28;
/// I/O error.
pub const ERRNO_IO: Errno = 29;
/// Socket is connected.
pub const ERRNO_ISCONN: Errno = 30;
/// Is a directory.
pub const ERRNO_ISDIR: Errno = 31;
/// Too many levels of symbolic links.
pub const ERRNO_LOOP: Errno = 32;
/// File descriptor value too large.
pub const ERRNO_MFILE: Errno = 33;
/// Too many links.
pub const ERRNO_MLINK: Errno = 34;
/// Message too large.
pub const ERRNO_MSGSIZE: Errno = 35;
/// Reserved.
pub const ERRNO_MULTIHOP: Errno = 36;
/// Filename too long.
pub const ERRNO_NAMETOOLONG: Errno = 37;
/// Network is down.
pub const ERRNO_NETDOWN: Errno = 38;
/// Connection aborted by network.
pub const ERRNO_NETRESET: Errno = 39;
/// Network unreachable.
pub const ERRNO_NETUNREACH: Errno = 40;
/// Too many files open in system.
pub const ERRNO_NFILE: Errno = 41;
/// No buffer space available.
pub const ERRNO_NOBUFS: Errno = 42;
/// No such device.
pub const ERRNO_NODEV: Errno = 43;
/// No such file or directory.
pub const ERRNO_NOENT: Errno = 44;
/// Executable file format error.
pub const ERRNO_NOEXEC: Errno = 45;
/// No locks available.
pub const ERRNO_NOLCK: Errno = 46;
/// Reserved.
pub const ERRNO_NOLINK: Errno = 47;
/// Not enough space.
pub const ERRNO_NOMEM: Errno = 48;
/// No message of the desired type.
pub const ERRNO_NOMSG: Errno = 49;
/// Protocol not available.
pub const ERRNO_NOPROTOOPT: Errno = 50;
/// No space left on device.
pub const ERRNO_NOSPC: Errno = 51;
/// Function not supported.
pub const ERRNO_NOSYS: Errno = 52;
/// The socket is not connected.
pub const ERRNO_NOTCONN: Errno = 53;
/// Not a directory or a symbolic link to a directory.
pub const ERRNO_NOTDIR: Errno = 54;
/// Directory not empty.
pub const ERRNO_NOTEMPTY: Errno = 55;
/// State not recoverable.
pub const ERRNO_NOTRECOVERABLE: Errno = 56;
/// Not a socket.
pub const ERRNO_NOTSOCK: Errno = 57;
/// Not supported, or operation not supported on socket.
pub const ERRNO_NOTSUP: Errno = 58;
/// Inappropriate I/O control operation.
pub const ERRNO_NOTTY: Errno = 59;
/// No such device or address.
pub const ERRNO_NXIO: Errno = 60;
/// Value too large to be stored in data type.
pub const ERRNO_OVERFLOW: Errno = 61;
/// Previous owner died.
pub const ERRNO_OWNERDEAD: Errno = 62;
/// Operation not permitted.
pub const ERRNO_PERM: Errno = 63;
/// Broken pipe.
pub const ERRNO_PIPE: Errno = 64;
/// Protocol error.
pub const ERRNO_PROTO: Errno = 65;
/// Protocol not supported.
pub const ERRNO_PROTONOSUPPORT: Errno = 66;
/// Protocol wrong type for socket.
pub const ERRNO_PROTOTYPE: Errno = 67;
/// Result too large.
pub const ERRNO_RANGE: Errno = 68;
/// Read-only file system.
pub const ERRNO_ROFS: Errno = 69;
/// Invalid seek.
pub const ERRNO_SPIPE: Errno = 70;
/// No such process.
pub const ERRNO_SRCH: Errno = 71;
/// Reserved.
pub const ERRNO_STALE: Errno = 72;
/// Connection timed out.
pub const ERRNO_TIMEDOUT: Errno = 73;
/// Text file busy.
pub const ERRNO_TXTBSY: Errno = 74;
/// Cross-device link.
pub const ERRNO_XDEV: Errno = 75;
/// Extension: Capabilities insufficient.
pub const ERRNO_NOTCAPABLE: Errno = 76;
pub(crate) fn strerror(code: u16) -> &'static str {
    match code {
        ERRNO_SUCCESS => "No error occurred. System call completed successfully.",
        ERRNO_2BIG => "Argument list too long.",
        ERRNO_ACCES => "Permission denied.",
        ERRNO_ADDRINUSE => "Address in use.",
        ERRNO_ADDRNOTAVAIL => "Address not available.",
        ERRNO_AFNOSUPPORT => "Address family not supported.",
        ERRNO_AGAIN => "Resource unavailable, or operation would block.",
        ERRNO_ALREADY => "Connection already in progress.",
        ERRNO_BADF => "Bad file descriptor.",
        ERRNO_BADMSG => "Bad message.",
        ERRNO_BUSY => "Device or resource busy.",
        ERRNO_CANCELED => "Operation canceled.",
        ERRNO_CHILD => "No child processes.",
        ERRNO_CONNABORTED => "Connection aborted.",
        ERRNO_CONNREFUSED => "Connection refused.",
        ERRNO_CONNRESET => "Connection reset.",
        ERRNO_DEADLK => "Resource deadlock would occur.",
        ERRNO_DESTADDRREQ => "Destination address required.",
        ERRNO_DOM => "Mathematics argument out of domain of function.",
        ERRNO_DQUOT => "Reserved.",
        ERRNO_EXIST => "File exists.",
        ERRNO_FAULT => "Bad address.",
        ERRNO_FBIG => "File too large.",
        ERRNO_HOSTUNREACH => "Host is unreachable.",
        ERRNO_IDRM => "Identifier removed.",
        ERRNO_ILSEQ => "Illegal byte sequence.",
        ERRNO_INPROGRESS => "Operation in progress.",
        ERRNO_INTR => "Interrupted function.",
        ERRNO_INVAL => "Invalid argument.",
        ERRNO_IO => "I/O error.",
        ERRNO_ISCONN => "Socket is connected.",
        ERRNO_ISDIR => "Is a directory.",
        ERRNO_LOOP => "Too many levels of symbolic links.",
        ERRNO_MFILE => "File descriptor value too large.",
        ERRNO_MLINK => "Too many links.",
        ERRNO_MSGSIZE => "Message too large.",
        ERRNO_MULTIHOP => "Reserved.",
        ERRNO_NAMETOOLONG => "Filename too long.",
        ERRNO_NETDOWN => "Network is down.",
        ERRNO_NETRESET => "Connection aborted by network.",
        ERRNO_NETUNREACH => "Network unreachable.",
        ERRNO_NFILE => "Too many files open in system.",
        ERRNO_NOBUFS => "No buffer space available.",
        ERRNO_NODEV => "No such device.",
        ERRNO_NOENT => "No such file or directory.",
        ERRNO_NOEXEC => "Executable file format error.",
        ERRNO_NOLCK => "No locks available.",
        ERRNO_NOLINK => "Reserved.",
        ERRNO_NOMEM => "Not enough space.",
        ERRNO_NOMSG => "No message of the desired type.",
        ERRNO_NOPROTOOPT => "Protocol not available.",
        ERRNO_NOSPC => "No space left on device.",
        ERRNO_NOSYS => "Function not supported.",
        ERRNO_NOTCONN => "The socket is not connected.",
        ERRNO_NOTDIR => "Not a directory or a symbolic link to a directory.",
        ERRNO_NOTEMPTY => "Directory not empty.",
        ERRNO_NOTRECOVERABLE => "State not recoverable.",
        ERRNO_NOTSOCK => "Not a socket.",
        ERRNO_NOTSUP => "Not supported, or operation not supported on socket.",
        ERRNO_NOTTY => "Inappropriate I/O control operation.",
        ERRNO_NXIO => "No such device or address.",
        ERRNO_OVERFLOW => "Value too large to be stored in data type.",
        ERRNO_OWNERDEAD => "Previous owner died.",
        ERRNO_PERM => "Operation not permitted.",
        ERRNO_PIPE => "Broken pipe.",
        ERRNO_PROTO => "Protocol error.",
        ERRNO_PROTONOSUPPORT => "Protocol not supported.",
        ERRNO_PROTOTYPE => "Protocol wrong type for socket.",
        ERRNO_RANGE => "Result too large.",
        ERRNO_ROFS => "Read-only file system.",
        ERRNO_SPIPE => "Invalid seek.",
        ERRNO_SRCH => "No such process.",
        ERRNO_STALE => "Reserved.",
        ERRNO_TIMEDOUT => "Connection timed out.",
        ERRNO_TXTBSY => "Text file busy.",
        ERRNO_XDEV => "Cross-device link.",
        ERRNO_NOTCAPABLE => "Extension: Capabilities insufficient.",
        _ => "Unknown error.",
    }
}
