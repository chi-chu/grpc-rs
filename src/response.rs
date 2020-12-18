use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::env;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref PATH: Box<PathBuf> = {
		let args: Vec<String> = env::args().collect();
		let p = Path::new(&args[0]).parent().unwrap().parent().unwrap();
		let file_path = p.join("mocker").join("instance").join("server").join("sdp");
		println!("Response data path: {:?}", &file_path);
		Box::new(file_path)
	};
}

pub fn get_response<P: AsRef<Path>>(fp: P) -> Result<String, Box<dyn Error>> {
	let mut file = fs::File::open(&*PATH.join(fp))?;
	let mut resp = String::new();
	file.read_to_string(&mut resp)?;
	Ok(resp)
}