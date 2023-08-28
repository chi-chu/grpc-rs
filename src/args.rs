use std::env;
use chrono::Local;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::string::String;
use std::sync::RwLock;

const PORT: u16 = 8080;
static CMD: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

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
			error!("no port param find use default port {}", PORT);
			PORT
		},
	}
}

pub fn init_cmd_line() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut map = CMD.write().unwrap();
    let mut i = 0;
    'a: loop {
        if i + 3 >= args.len() {
            break 'a;
        }
        map.insert(args[i].replace("-", "").to_owned(), args[i + 1].to_owned());
        i += 2;
    }
}

pub fn is_grpc() -> bool {
    let g = CMD.read().unwrap();
    return match g.get("protocol") {
        None => false,
        Some(protocol) => {
            if protocol.as_str() == "grpc" {
                return true;
            }
            false
        }
    };
}
