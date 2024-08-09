use std::fmt::Display;
use std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq)]
pub struct EnvVars(Vec<(String, String)>);

impl Display for EnvVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{}={}", key, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    pub fn test_stuff() {
        let mut vars = EnvVars(Vec::new());

        env::vars().for_each(|(key, value)| vars.0.push((key, value)));

        let b: EnvVars = serde_json::from_str(&serde_json::to_string(&vars).expect("error")).expect("invalid");

        assert_eq!(vars, b);
    }
}
