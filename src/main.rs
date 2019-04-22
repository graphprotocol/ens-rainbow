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
        let hash = k::keccak256(name.as_bytes());
        println!("'Name'|'{hash:x}'|'{{\"id\": {{\"data\": \"{hash:x}\", \"type\": \"String\"}}, \"name\": {{\"data\": \"{name}\", \"type\": \"String\"}} }}'|'none'",
                hash=hash.plain_hex(false), name=name);
    }
}
