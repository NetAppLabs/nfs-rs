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

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Struct describing an NFS timestamp.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Time {
    pub seconds: u32,
    pub nseconds: u32,
}

impl Time {
    // Convert to std::time::SystemTime
    pub fn to_system_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::new(self.seconds as u64, self.nseconds)
    }

    // Create Time from SystemTime
    pub fn from_system_time(system_time: SystemTime) -> Self {
        match system_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => Time {
                seconds: duration.as_secs() as u32,
                nseconds: duration.subsec_nanos(),
            },
            Err(e) => {
                // Handle time earlier than UNIX_EPOCH
                let duration = e.duration();
                Time {
                    seconds: duration.as_secs() as u32,
                    nseconds: duration.subsec_nanos(),
                }
            }
        }
    }
}
