#[macro_export]
macro_rules! debug {
	($format:expr, $($arg:tt)+) => (
		if (unsafe { $crate::log::LEVEL.to_i8() < 0 }) {
			println!("【DEBUG】 {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}",
				Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)+))
		}
	);
}

#[macro_export]
macro_rules! info {
	($format:expr, $($arg:tt)+) => (
		if (unsafe {$crate::log::LEVEL.to_i8() <= 1 }) {
			if cfg!(target_family = "unix") || cfg!(target_family = "windows") {
				println!("{}[1;40;32m【INFO 】{}[0m {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}",
					0x1B as char, 0x1B as char, Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)+))
			} else {
				println!("【INFO 】 {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}",
					Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)+))
			}
		}
	);
}

#[macro_export]
macro_rules! warn {
	($format:expr, $($arg:tt)+) => (
		if (unsafe {$crate::log::LEVEL.to_i8() <= 2 }) {
			if cfg!(target_family = "unix") || cfg!(target_family = "windows") {
				println!("{}[1;40;33m【WARN 】{}[0m {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}", 
					0x1B as char, 0x1B as char, Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)+))
			} else {
				println!("【WARN 】 {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}",
					Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)+))
			}
		}
	);
}

#[macro_export]
macro_rules! error {
	($format:expr, $($arg:tt)*) => (
		if cfg!(target_family = "unix") || cfg!(target_family = "windows") {
			println!("{}[1;40;31m【ERROR】{}[0m {:?} MODULE:{} FILE:{} LINE: {}   DATA:{}",
				0x1B as char, 0x1B as char, Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)*))
		} else {
			println!("【ERROR】 {:?} MODULE:{} FILE:{} LINE:{}   DATA:{}",
				Local::now(), module_path!(), file!(), line!(), format_args!($format, $($arg)*))
		}
	);
}
