// use sonic_rs::{Deserialize, Serialize};
use std::env;
use std::vec::Vec;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use flate2::{write::GzEncoder, Compression};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Hash)]
struct EnvVars(Vec<(String, String)>);

impl Display for EnvVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{}={}", key, value)?;
        }
        Ok(())
    }
}

fn main() {
    let mut vars = EnvVars(Vec::new());

    env::vars().for_each(|(key, value)| {
        vars.0.push((key, value))
    });

    // let stdout = io::stdout();
    let mut serializer = Serializer::new(Vec::new());

    let serialized = match vars.serialize(&mut serializer) {
        Ok(_) => serializer.into_inner(),
        Err(e) => panic!("Failed to serialize: {}", e),
    };

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    let gzipped = match e.write_all(&serialized[..]) {
        Ok(_) => match e.finish() {
            Ok(v) => v,
            Err(e) => panic!("Failed to finish compression: {}", e),
        },
        Err(e) => panic!("Failed to compress: {}", e),
    };

    let mut ungzipped = Vec::new();

    match flate2::read::GzDecoder::new(&gzipped[..]).read_to_end(&mut ungzipped) {
        Ok(v) => v,
        Err(e) => panic!("Failed to decompress: {}", e),
    };

    let deserialized = match EnvVars::deserialize(&mut Deserializer::new(&serialized[..])) {
        Ok(v) => v,
        Err(e) => panic!("Failed to deserialize: {}", e),
    };

    println!("{:#?}", deserialized);
    println!("Serialized size: {} bytes", serialized.len());
    println!("Gzipped size: {} bytes", gzipped.len());
}
