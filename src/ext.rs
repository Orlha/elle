use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io;
use std::error;

use crate::game::*;


pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Copy, Clone)]
pub struct Pos {
	pub x: i64,
	pub y: i64,
}

impl Pos {
	pub fn new() -> Pos {
		Pos{x: 0, y: 0}
	}
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

pub fn get_rand(x: usize) -> Result<Vec<u8>> {
	let mut f = File::open("/dev/urandom")?;
	let mut buf = vec![0u8; x];
	f.read_exact(&mut buf)?;
	return Ok(buf);
}

pub trait Ext: std::fmt::Display + std::fmt::Debug
{
	fn out(&self) -> std::result::Result<(), std::fmt::Error>;
	fn tt(&self) {
		println!("Self: {}", self);
		println!("Self: {:?}", self);
		println!("Self: {:#?}", self);
		return;
	}
	/*
	fn t1(&self) -> Option<i32> {
		let x = get_rand();
		if(x % 2 == 0) {
			return Some(1);
		}
		return None;
	}
	*/
}

pub fn trim_newline(s: &mut String) {
	if s.ends_with('\n') {
		s.pop();
		if s.ends_with('\r') {
			s.pop();
		}
	}
}

pub fn read_string() -> Result<String> {
	let mut str = String::new();
	io::stdin().read_line(&mut str)?;
	trim_newline(&mut str);
	Ok(str)
}
