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
    println!("
-- SQL script to recreate the ens_names table from a list of names

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

ALTER TABLE ONLY public.ens_names DROP CONSTRAINT ens_names_pkey;
DROP TABLE public.ens_names;
SET default_with_oids = false;

CREATE TABLE public.ens_names (
    hash character varying NOT NULL,
    name character varying NOT NULL
);


--
-- Data for ens_names
--

COPY public.ens_names (hash, name) FROM stdin;
    ");
    for e in Deserializer::from_reader(io::stdin()).into_iter::<Entry>() {
        let name = e.unwrap().name;
        let keccak_hash = k::keccak256(name.as_bytes());
        let formatted_keccak = format!("0x{}", hex::encode(keccak_hash));

        println!("{hash}\t{name}",
                hash=formatted_keccak, name=name);
    }
    println!("\\.");
    println!("
--
-- Primary key and index for hash
--

ALTER TABLE ONLY public.ens_names
    ADD CONSTRAINT ens_names_pkey PRIMARY KEY (hash);
    ");
}
