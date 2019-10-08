use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io;
use std::error;

const MAP_SIZE: usize = 4;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Map {
	//
	data: [[u8; MAP_SIZE]; MAP_SIZE],
}

pub struct Game {
	map: Map,
	x: i32,
	y: i32,
}

impl Map {
	pub fn new() -> Map {
		Map{data: Default::default()}
	}
}

impl Game {
	pub fn new(x: i32, y: i32) -> Game {
		Game {map: Map::new(), x: x, y: y}
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if y == 0 {
					write!(f, "\t")?;
				}
				write!(f, "{} ", self.data[x][y])?;
			}
			if x < MAP_SIZE - 1 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}\n", self.x, self.y)?;
		write!(f, "map:\n{}", self.map)?;
		Ok(())
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

fn clear_screen() {
	println!("{}", termion::clear::All);
	//
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
