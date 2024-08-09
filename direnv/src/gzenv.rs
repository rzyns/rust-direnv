use bincode;
use std::env;
use std::fmt::Display;
use std::vec::Vec;

#[derive(Debug, Hash)]
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

    env::vars().for_each(|(key, value)| vars.0.push((key, value)));

    let a = bincode::serialize(&vars.0).unwrap();

    println!("{:#?}", a)
}
