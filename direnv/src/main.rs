pub mod gzenv;
pub mod dotenv;
pub mod env;

use std::process;
use env::get_env;

fn main() {
    process::Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");

    println!("{:#?}", get_env())
}
