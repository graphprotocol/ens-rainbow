extern crate tiny_keccak;
extern crate serde_json;
extern crate serde_derive;
extern crate hex_slice;

use tiny_keccak as k;
use serde::Deserialize;
use serde_json::{Deserializer};
use std::io;
use hex_slice::AsHex;

#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "t")]
    name: String
}

fn main() {
    println!("'entity'|'id'|'data'|'event_source'");
    for e in Deserializer::from_reader(io::stdin()).into_iter::<Entry>() {
        let name = e.unwrap().name;
        let keccak_hash = k::keccak256(name.as_bytes());
        let hex_keccak = keccak_hash.plain_hex(false);
        let formatted_keccak= format!("{:x}", hex_keccak);
        //        println!("formatted {:}", formatted);
        //        println!("formatted just in text: {:x}", hex_keccak);

        let resolver_hash = k::keccak256("resolver".as_bytes());
        let hex_resolver_hash = resolver_hash.plain_hex(false);
        if name == "resolver".to_string() {
            println!("RESOLVER FOUND");
            println!("resolver hashed: {:x}", hex_resolver_hash);
            println!("ours: {:x}", hex_keccak);
        }

        // trying using sha3_256
        let sha_hash = k::sha3_256(name.as_bytes());
        let hex_sha=sha_hash.plain_hex(false);

        // trying hashing zeros byte array and name and hashing together
        let sha_zeros = k::keccak256(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        let sha_name = k::keccak256(name.as_bytes());
        let mut zero = sha_zeros.into_iter().cloned().collect::<Vec<_>>();
        let name = sha_name.into_iter().cloned().collect::<Vec<_>>();
        zero.extend(name);
        let sha_zeros_hash = k::keccak256(zero.as_slice());
        let hex_sha_zeros=sha_zeros_hash.plain_hex(false);

        let sample_labelhash1 = "11e522faf24d279a5aa624f046e3003b3f27ef2b6497e532b96b4ce847f89f68";
        let sample_labelhash2 = "08caf3048c7c733ca26a5bc0dbaa2d433fd60113ffc77964e9e946af31d856e7";
        let sample_labelhash3 = "081f79b96eb458c2f4e37785351cf46a8515b30fbe4a388461cf682d7ee4fdd3";
        let sample_labelhash4 = "390ddd132c91589c103b7d227af5a35feede8b337a7afb703bc77b58604b8138";
        let sample_labelhash5 = "179cef2dd81a77353bc9d10a5a512afa1c3c68be65c694376a78ee4e28f27c6e";
        let sample_labelhash6 = "7d0cbd59385db0496d345aebd7c34ef360fc4429daebfb66612e8d0f8255fbbf";
        let sample_labelhash7 = "757cef2fc75b6b28b1feeedadc7600b38c28ba615bc04df6d111b3f5093bf0ed";
        let sample_labelhash7 = "54184928deaf5ff208a81a7df9801f8d2751062453c221aadfaf08f6dd1ab34d";
        let test_hashes = vec![sample_labelhash1,sample_labelhash2,sample_labelhash3,sample_labelhash4,sample_labelhash5,sample_labelhash6,sample_labelhash7];

        match test_hashes.into_iter().find(|&x| x == formatted_keccak) {
            Some(_) => println!("FOUND ONE"),
            None => ()
        }
    }
}
