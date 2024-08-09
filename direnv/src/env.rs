use std::env;

#[derive(Debug)]
pub struct EnvVars(Vec<(String, String)>);

pub fn get_env() -> EnvVars {
    let mut vars = EnvVars(Vec::new());

    env::vars().for_each(|(key, value)| vars.0.push((key, value)));

    vars
}
