#![allow(unused_imports)]
use std::option::Option;

#[derive(Debug)]
pub enum LogLevel {
	Debug,
	Info,
	Warn,
	Error,
}

impl LogLevel {
	pub fn new(n: i8) -> Option<LogLevel> {
		match n {
			0 => Option::Some(LogLevel::Debug),
			1 => Option::Some(LogLevel::Info),
			2 => Option::Some(LogLevel::Warn),
			3 => Option::Some(LogLevel::Error),
			_ => None
		}
	}

	#[inline(always)]
	pub fn to_i8(&self) -> i8 {
		match &self {
			LogLevel::Debug => 0,
			LogLevel::Info => 1,
			LogLevel::Warn => 2,
			LogLevel::Error => 3
		}
	}
}

pub static mut LEVEL: LogLevel = LogLevel::Debug;

pub fn set_level(lv: LogLevel) {
	unsafe {
		LEVEL = lv;
	}
}

pub fn get_level() -> i8 {
	unsafe {
		LEVEL.to_i8()
	}
}