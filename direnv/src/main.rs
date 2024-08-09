use std::process;

use env::get_env;
mod env;

fn main() {
    process::Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");

    println!("{:#?}", get_env())
}
