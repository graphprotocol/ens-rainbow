extern crate tiny_keccak;
extern crate serde_json;
extern crate serde_derive;
extern crate hex;

use tiny_keccak as k;
use serde::Deserialize;
use serde_json::{Deserializer};
use std::io;

#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "t")]
    name: String
}

fn main() {
    for e in Deserializer::from_reader(io::stdin()).into_iter::<Entry>() {
        let name = e.unwrap().name;
        let keccak_hash = k::keccak256(name.as_bytes());
        let formatted_keccak = format!("0x{}", hex::encode(keccak_hash));

        println!("Name,{hash},\"{{\"\"id\"\": {{\"\"data\"\": \"\"{hash}\"\", \"\"type\"\": \"\"String\"\"}}, \"\"name\"\": {{\"\"data\"\": \"\"{name}\"\", \"\"type\"\": \"\"String\"\"}}}}\"",
                hash=formatted_keccak, name=name);
    }
}
