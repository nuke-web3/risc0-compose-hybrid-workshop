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

use ssid_core::SSID;
use risc0_zkvm::guest::env;
use chrono::NaiveDate;

fn main() {
    let ssid: SSID = env::read();
    // let date: NaiveDate = env::read();

    // // Note: as we check the ID is signed (in a real application, not this demo)
    // // and that it's not expired explicitly by the given date,
    // // the worst a dishonest prover could do is publicly commit to a time 
    // // that isn't "now" but is forced to reveal the time they used.
    // assert!(ssid.is_valid(&date));

    // // Constraints publicly committed to:
    // // - The ID owner's identifier (name could be UUID or on-chain address, etc.)
    // // - The issuer's identifier
    // // - The time this proof was generated (ASSUMES an honest prover)
    // env::commit(&ssid.name);
    // env::commit(&ssid.issuer);
    // env::commit(&date);
    env::commit(&1234);
}
