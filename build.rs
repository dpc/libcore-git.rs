#![feature(old_io, old_path)]

// Generate the git version string.

use std::old_io::process::{Command, ProcessOutput};
use std::old_io::{File};

pub fn main() {
    match Command::new("git").arg("clone").arg("--depth").arg("1")
        .arg("https://github.com/rust-lang/rust.git").output() {
            Ok(ProcessOutput { status: exit, output: out, error: err }) => {
                if exit.success() {
                } else {
                    println!("Error getting rustc version: {}", String::from_utf8(err).unwrap());
                }
            }
            Err(err) => {
                println!("Error cloning rustc git repo: {}", err);
            }
        }
}
