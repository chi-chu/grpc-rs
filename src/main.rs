use grpc_rs::log;
use grpc_rs::server;

#[macro_use]
mod macros;

include!("version.rs");

fn main() {
	println!("Server GIT Commit Version: {}", VERSION);
	log::set_level(log::LogLevel::Debug);
    server::start();
}
