// GENERATED CODE
//
// Generated from mount.x by xdrgen.
//
// DO NOT EDIT

pub const FHSIZE: i64 = 32i64;

pub const FHSIZE3: i64 = 64i64;

pub const MNTNAMLEN: i64 = 255i64;

pub const MNTPATHLEN: i64 = 1024i64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct dirpath(pub String);

pub struct exportnode {
    pub ex_dir: dirpath,
    pub ex_groups: Option<Box<groupnode>>,
    pub ex_next: Option<Box<exportnode>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct fhandle1(pub [u8; FHSIZE as usize]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct fhandle3(pub Vec<u8>);

pub struct groupnode {
    pub gr_name: name,
    pub gr_next: Option<Box<groupnode>>,
}

pub struct mountbody {
    pub ml_hostname: name,
    pub ml_directory: dirpath,
    pub ml_next: Option<Box<mountbody>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mountres1 {
    MNT1_OK(mountres1_ok),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct mountres1_ok {
    pub fhandle: fhandle1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum mountres3 {
    MNT3_OK(mountres3_ok),
    default,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct mountres3_ok {
    pub fhandle: fhandle3,
    pub auth_flavors: Vec<i32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mountstat1 {
    MNT1_OK = 0isize,
    MNT1ERR_PERM = 1isize,
    MNT1ERR_NOENT = 2isize,
    MNT1ERR_IO = 5isize,
    MNT1ERR_ACCES = 13isize,
    MNT1ERR_NOTDIR = 20isize,
    MNT1ERR_INVAL = 22isize,
    MNT1ERR_NAMETOOLONG = 63isize,
    MNT1ERR_NOTSUPP = 10004isize,
    MNT1ERR_SERVERFAULT = 10006isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mountstat3 {
    MNT3_OK = 0isize,
    MNT3ERR_PERM = 1isize,
    MNT3ERR_NOENT = 2isize,
    MNT3ERR_IO = 5isize,
    MNT3ERR_ACCES = 13isize,
    MNT3ERR_NOTDIR = 20isize,
    MNT3ERR_INVAL = 22isize,
    MNT3ERR_NAMETOOLONG = 63isize,
    MNT3ERR_NOTSUPP = 10004isize,
    MNT3ERR_SERVERFAULT = 10006isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct name(pub String);

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for dirpath {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(
            &self.0,
            Some(MNTPATHLEN as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for exportnode {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ex_dir.pack(out)? + self.ex_groups.pack(out)? + self.ex_next.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for fhandle1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for fhandle3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(FHSIZE3 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for groupnode {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.gr_name.pack(out)? + self.gr_next.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountbody {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(
            self.ml_hostname.pack(out)?
                + self.ml_directory.pack(out)?
                + self.ml_next.pack(out)?
                + 0,
        )
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountres1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &mountres1::MNT1_OK(ref val) => {
                (mountstat1::MNT1_OK as i32).pack(out)? + val.pack(out)?
            }
            &mountres1::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountres1_ok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fhandle.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountres3 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &mountres3::MNT3_OK(ref val) => {
                (mountstat3::MNT3_OK as i32).pack(out)? + val.pack(out)?
            }
            &mountres3::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountres3_ok {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.fhandle.pack(out)? + xdr_codec::pack_flex(&self.auth_flavors, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountstat1 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for mountstat3 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for name {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(
            &self.0,
            Some(MNTNAMLEN as usize),
            out,
        )?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for dirpath {
    fn unpack(input: &mut In) -> xdr_codec::Result<(dirpath, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(MNTPATHLEN as usize))?;
                sz = usz;
                dirpath(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for exportnode {
    fn unpack(input: &mut In) -> xdr_codec::Result<(exportnode, usize)> {
        let mut sz = 0;
        Ok((
            exportnode {
                ex_dir: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ex_groups: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ex_next: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for fhandle1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(fhandle1, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; FHSIZE as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], FHSIZE as usize)?;
                    (buf, sz)
                };
                sz = usz;
                fhandle1(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for fhandle3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(fhandle3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(FHSIZE3 as usize))?;
                sz = usz;
                fhandle3(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for groupnode {
    fn unpack(input: &mut In) -> xdr_codec::Result<(groupnode, usize)> {
        let mut sz = 0;
        Ok((
            groupnode {
                gr_name: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                gr_next: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountbody {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountbody, usize)> {
        let mut sz = 0;
        Ok((
            mountbody {
                ml_hostname: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ml_directory: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ml_next: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountres1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountres1, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => mountres1::MNT1_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => mountres1::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountres1_ok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountres1_ok, usize)> {
        let mut sz = 0;
        Ok((
            mountres1_ok {
                fhandle: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountres3 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountres3, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => mountres3::MNT3_OK({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => mountres3::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountres3_ok {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountres3_ok, usize)> {
        let mut sz = 0;
        Ok((
            mountres3_ok {
                fhandle: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                auth_flavors: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountstat1 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountstat1, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == mountstat1::MNT1_OK as i32 => mountstat1::MNT1_OK,
                    x if x == mountstat1::MNT1ERR_PERM as i32 => mountstat1::MNT1ERR_PERM,
                    x if x == mountstat1::MNT1ERR_NOENT as i32 => mountstat1::MNT1ERR_NOENT,
                    x if x == mountstat1::MNT1ERR_IO as i32 => mountstat1::MNT1ERR_IO,
                    x if x == mountstat1::MNT1ERR_ACCES as i32 => mountstat1::MNT1ERR_ACCES,
                    x if x == mountstat1::MNT1ERR_NOTDIR as i32 => mountstat1::MNT1ERR_NOTDIR,
                    x if x == mountstat1::MNT1ERR_INVAL as i32 => mountstat1::MNT1ERR_INVAL,
                    x if x == mountstat1::MNT1ERR_NAMETOOLONG as i32 => {
                        mountstat1::MNT1ERR_NAMETOOLONG
                    }
                    x if x == mountstat1::MNT1ERR_NOTSUPP as i32 => mountstat1::MNT1ERR_NOTSUPP,
                    x if x == mountstat1::MNT1ERR_SERVERFAULT as i32 => {
                        mountstat1::MNT1ERR_SERVERFAULT
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for mountstat3 {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(mountstat3, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == mountstat3::MNT3_OK as i32 => mountstat3::MNT3_OK,
                    x if x == mountstat3::MNT3ERR_PERM as i32 => mountstat3::MNT3ERR_PERM,
                    x if x == mountstat3::MNT3ERR_NOENT as i32 => mountstat3::MNT3ERR_NOENT,
                    x if x == mountstat3::MNT3ERR_IO as i32 => mountstat3::MNT3ERR_IO,
                    x if x == mountstat3::MNT3ERR_ACCES as i32 => mountstat3::MNT3ERR_ACCES,
                    x if x == mountstat3::MNT3ERR_NOTDIR as i32 => mountstat3::MNT3ERR_NOTDIR,
                    x if x == mountstat3::MNT3ERR_INVAL as i32 => mountstat3::MNT3ERR_INVAL,
                    x if x == mountstat3::MNT3ERR_NAMETOOLONG as i32 => {
                        mountstat3::MNT3ERR_NAMETOOLONG
                    }
                    x if x == mountstat3::MNT3ERR_NOTSUPP as i32 => mountstat3::MNT3ERR_NOTSUPP,
                    x if x == mountstat3::MNT3ERR_SERVERFAULT as i32 => {
                        mountstat3::MNT3ERR_SERVERFAULT
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for name {
    fn unpack(input: &mut In) -> xdr_codec::Result<(name, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(MNTNAMLEN as usize))?;
                sz = usz;
                name(v)
            },
            sz,
        ))
    }
}
