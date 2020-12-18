use std::env;
use chrono::Local;

const PORT: u16 = 8088;

pub fn get_port() -> u16 {
	match env::args().nth(1) {
		Some(p) => {
			match p.parse::<u16>() {
				Ok(port) => port,
				Err(e) => {
					error!("port parse err {:?} and use default port {}", e, PORT);
					8088
				}
			}
		},
		None => {
			error!("use default port {}", PORT);
			PORT
		},
	}
}