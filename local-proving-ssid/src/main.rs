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

use chrono::{NaiveDate, Utc};
use risc0_zkvm::{default_prover, ExecutorEnv};
use ssid_core::SSID;
use ssid_methods::{SSID_ADULT_CHECK_ELF, SSID_ADULT_CHECK_ID};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Name or Identifier on ID
    #[arg(default_value = "Someone")]
    name: String,

    /// Birthday on ID
    #[arg(default_value = "2000-1-1")]
    birthday: String,

    /// Issuing Authority for ID
    #[arg(default_value = "The Man")]
    issuer: String,

    /// Issuing Authority Signature over ID
    #[arg(default_values_t =  [0; 32])]
    issuer_signature: Vec<u8>,

    #[arg(default_value = "2030-1-1")]
    expiration: String,
}
fn main() {
    let args = Cli::parse();

    let ssid = SSID {
        name: args.name,
        // Parsing details: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        birth: NaiveDate::parse_from_str(args.birthday.as_str(), "%F")
            .expect("Date must be 'YYYY-MM-DD'"),
        expiration: NaiveDate::parse_from_str(args.expiration.as_str(), "%F")
            .expect("Date must be 'YYYY-MM-DD'"),
        issuer: args.issuer,
        issuer_signature: args
            .issuer_signature
            .as_slice()
            .try_into()
            .expect("signature data is array"),
    };

    let date = Utc::now().date_naive();

    let env = ExecutorEnv::builder()
        .write(&ssid)
        .unwrap()
        .write(&date)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SSID_ADULT_CHECK_ELF).unwrap().receipt;

    // Verify receipt and parse it for committed data
    // receipt.verify(SSID_ADULT_CHECK_ID).unwrap();
    let committed_state: String = receipt.journal.decode().unwrap();

    println!("{committed_state:?}");
}
