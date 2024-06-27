// Copyright 2024 RISC Zero, Inc.
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

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Simple Self-Sovereign ID
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SSID {
    /// User's legal name
    pub name: String,
    /// Birth Date
    pub birth: NaiveDate,
    /// Expiration of this ID
    pub expiration: NaiveDate,
    /// Name of credential issuer
    pub issuer: String,
    /// Signature of issuer on this SSID
    pub issuer_signature: [u8; 32],
}

impl SSID {
    /// WARN: this is only a mock!
    /// Checks if the SSID is correctly signed and is not expired as of a specific [DateTime]
    pub fn is_valid(&self, date: &NaiveDate) -> bool {
        eprint!("WARN: this is only a mock function without signature checking!");

        // TODO verify self.signature here

        assert!(self.expiration > *date);
        true
    }
}
