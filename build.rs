extern crate protoc_rust_grpc;
//use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
fn main() {
	//let out_dir = env::var("OUT_DIR").unwrap();
	let out_dir = "src";
	let dest_path = Path::new(&out_dir).join("version.rs");
	let mut f = File::create(&dest_path).unwrap();
	let commit = Command::new("git")
	.arg("rev-parse")
	.arg("HEAD")
	.output()
	.expect("Failed to execute git command");
	let mut commit = String::from_utf8(commit.stdout).expect("Invalid utf8 string");
	commit.pop().unwrap();
	//commit.truncate();
	let output = format!(r#"pub const VERSION : &'static str = "{}";"#, commit);
	f.write_all(output.as_bytes()).unwrap();

    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .include("./proto/")
        .input("./proto/MediaTransfer.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}