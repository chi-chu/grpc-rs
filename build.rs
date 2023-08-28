#![allow(unused)]

use std::{fs, thread};
use std::process::{Command, ExitStatus};

fn main() {
    println!("now exec dir is {:?}", env::current_dir().unwrap().display());
    gen_and_rm();
}

#[cfg(not(target_os = "windows"))]
fn gen_and_rm() {
    tonic_build::configure().build_server(false).out_dir("src").compile(&["src/credit.proto"], &["src"]).unwrap();
}

#[cfg(target_os = "windows")]
fn gen_and_rm() {
    Command::new("rmdir").
        arg("/s").
        arg("/q").
        arg("./backend-partner-credit").
        status();
}
