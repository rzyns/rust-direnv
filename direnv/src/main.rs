use std::process;
use std::env;

fn main() {
    process::Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
}
