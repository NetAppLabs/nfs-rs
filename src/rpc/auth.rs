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

use xdr_codec::Pack;

const AUTH_BODY_MAX_SIZE: usize = 400;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AuthFlavor {
    Null = 0,
    Unix = 1,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Auth {
    pub(crate) flavor: AuthFlavor,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) body: Vec<u8>,
}

impl Auth {
    pub(crate) fn new_null() -> Self {
        Self {
            flavor: AuthFlavor::Null,
            uid: 0,
            gid: 0,
            body: vec![],
        }
    }

    pub(crate) fn new_unix(machinename: &str, uid: u32, gid: u32) -> Self {
        let stamp = super::get_current_time();
        let unix = AuthUnix {
            stamp,
            machinename: machinename.to_string(),
            uid,
            gid,
            gids: Vec::new(),
        };

        let mut body = Vec::<u8>::new();
        let _ = unix.pack(&mut body).unwrap();
        Self {
            flavor: AuthFlavor::Unix,
            uid,
            gid,
            body,
        }
    }
}

impl<Out: xdr_codec::Write> Pack<Out> for Auth {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((self.flavor.clone() as u32).pack(out)?
            + xdr_codec::pack_opaque_flex(&self.body, Some(AUTH_BODY_MAX_SIZE), out)?)
    }
}

struct AuthUnix {
    stamp: u32,
    machinename: String,
    uid: u32,
    gid: u32,
    gids: Vec<u32>,
}

impl<Out: xdr_codec::Write> Pack<Out> for AuthUnix {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.stamp.pack(out)?
            + self.machinename.pack(out)?
            + self.uid.pack(out)?
            + self.gid.pack(out)?
            + self.gids.pack(out)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_new_null() {
        assert_eq!(
            Auth::new_null(),
            Auth {
                flavor: AuthFlavor::Null,
                uid: 0,
                gid: 0,
                body: vec![]
            }
        );
    }

    #[test]
    fn auth_null_pack() {
        let auth = Auth::new_null();
        let mut buf = Vec::<u8>::new();
        let res = auth.pack(&mut buf);
        assert!(res.is_ok());
        let expected: Vec<u8> = vec![
            0, 0, 0, 0, // auth flavor (AuthFlavor::Null)
            0, 0, 0, 0, // body length in bytes
        ];
        assert_eq!(buf, expected);
        assert_eq!(buf.len(), res.unwrap());
    }

    #[test]
    fn auth_new_unix_with_padding_of_0() {
        let actual = Auth::new_unix("machined", 1, 2);
        let mut expected = Auth {
            flavor: AuthFlavor::Unix,
            uid: 1,
            gid: 2,
            body: vec![
                0, 0, 0, 0, // stamp (random - hopefully never matches actual)
                0, 0, 0, 8,   // machinename length in bytes
                109, // 'm'
                97,  // 'a'
                99,  // 'c'
                104, // 'h'
                105, // 'i'
                110, // 'n'
                101, // 'e'
                100, // 'd'
                0, 0, 0, 1, // uid (1)
                0, 0, 0, 2, // gid (2)
                0, 0, 0, 0, // gidlen
            ],
        };
        assert_ne!(actual, expected);
        expected.body.splice(0..4, actual.body[0..4].to_vec());
        assert_eq!(actual, expected);
    }

    #[test]
    fn auth_new_unix_with_padding_of_1() {
        let actual = Auth::new_unix("machine", 350, 400);
        let mut expected = Auth {
            flavor: AuthFlavor::Unix,
            uid: 350,
            gid: 400,
            body: vec![
                0, 0, 0, 0, // stamp (random - hopefully never matches actual)
                0, 0, 0, 7,   // machinename length in bytes
                109, // 'm'
                97,  // 'a'
                99,  // 'c'
                104, // 'h'
                105, // 'i'
                110, // 'n'
                101, // 'e'
                0,   // padding
                0, 0, 1, 94, // uid (350)
                0, 0, 1, 144, // gid (400)
                0, 0, 0, 0, // gidlen
            ],
        };
        assert_ne!(actual, expected);
        expected.body.splice(0..4, actual.body[0..4].to_vec());
        assert_eq!(actual, expected);
    }

    #[test]
    fn auth_new_unix_with_padding_of_2() {
        let actual = Auth::new_unix("machin", 666, 616);
        let mut expected = Auth {
            flavor: AuthFlavor::Unix,
            uid: 666,
            gid: 616,
            body: vec![
                0, 0, 0, 0, // stamp (random - hopefully never matches actual)
                0, 0, 0, 6,   // machinename length in bytes
                109, // 'm'
                97,  // 'a'
                99,  // 'c'
                104, // 'h'
                105, // 'i'
                110, // 'n'
                0, 0, // padding
                0, 0, 2, 154, // uid (666)
                0, 0, 2, 104, // gid (616)
                0, 0, 0, 0, // gidlen
            ],
        };
        assert_ne!(actual, expected);
        expected.body.splice(0..4, actual.body[0..4].to_vec());
        assert_eq!(actual, expected);
    }

    #[test]
    fn auth_new_unix_with_padding_of_3() {
        let actual = Auth::new_unix("machi", 1024, 2048);
        let mut expected = Auth {
            flavor: AuthFlavor::Unix,
            uid: 1024,
            gid: 2048,
            body: vec![
                0, 0, 0, 0, // stamp (random - hopefully never matches actual)
                0, 0, 0, 5,   // machinename length in bytes
                109, // 'm'
                97,  // 'a'
                99,  // 'c'
                104, // 'h'
                105, // 'i'
                0, 0, 0, // padding
                0, 0, 4, 0, // uid (1024)
                0, 0, 8, 0, // gid (2048)
                0, 0, 0, 0, // gidlen
            ],
        };
        assert_ne!(actual, expected);
        expected.body.splice(0..4, actual.body[0..4].to_vec());
        assert_eq!(actual, expected);
    }

    #[test]
    fn auth_unix_pack() {
        let auth = Auth::new_unix("machine", 350, 400);
        let mut buf = Vec::<u8>::new();
        let res = auth.pack(&mut buf);
        assert!(res.is_ok());
        let mut expected: Vec<u8> = vec![
            0, 0, 0, 1, // auth flavor (AuthFlavor::Unix)
            0, 0, 0, 28, // body length in bytes
            0, 0, 0, 0, // stamp (random - hopefully never matches actual)
            0, 0, 0, 7,   // machinename length in bytes
            109, // 'm'
            97,  // 'a'
            99,  // 'c'
            104, // 'h'
            105, // 'i'
            110, // 'n'
            101, // 'e'
            0,   // padding
            0, 0, 1, 94, // uid (350)
            0, 0, 1, 144, // gid (400)
            0, 0, 0, 0, // gidlen
        ];
        assert_ne!(buf, expected);
        expected.splice(8..12, auth.body[0..4].to_vec());
        assert_eq!(buf, expected);
        assert_eq!(buf.len(), res.unwrap());
    }
}
