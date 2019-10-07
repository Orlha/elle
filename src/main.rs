extern crate termion;

mod ext;
use ext::*;

use std::fmt;
use std::io;
//use std::result;
//use std::os;
use std::env;

const MAP_SIZE: usize = 4;

struct Map {
	data: [[i32; MAP_SIZE]; MAP_SIZE],
}

struct Game {
	map: Map,
	x: i32,
	y: i32,
}

impl Game {
	fn rdo(&self) {
		return;
	}
	fn new(x:i32, y:i32) -> Game {
		//Game{x: x, y: y, map: [[0; 1]; 1]}
		Game{x: x, y: y, map: Map::new()}
	}
}

impl Map {
	fn new() -> Map {
		Map{data: Default::default()}
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{} {}", self.x, self.y).ok();
		write!(f, "{}", self.map)
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				write!(f, "{} ", self.data[x][y]).ok();
			}
			if x < MAP_SIZE - 1 {
				writeln!(f, "").ok();
			}
		}
		Ok(())
	}
}

#[allow(dead_code)]
fn clear_screen() {
	//
	println!("{}", termion::clear::All);
}

fn sstr() {
	let x = "Qwe";
	match x {
		"Qwe" => {
			println!("Qwe yourself!");
			println!("Qwe yourself!");
			println!("Qwe yourself!");
		}
		"biteMe" => println!("Bite my _Rusty_ metal ass!"),
		"killYa" => println!("KILL ALL HUMANS!"),
		"letsGo" => println!("Let's GO ALREADYYY..."),
		_        => println!("Oh. Your. God.")
	}
}

fn get_string() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	//buffer.pop();
	trim_newline(&mut buffer);
	return Ok(buffer);
}

fn main() {
	let argv = &env::args();
	println!("{:?}", argv);
	let game = Game::new(1, 2);
	game.rdo();
	//clear_screen();
	println!("Started;");
	println!("{}", game);
	let s = get_string().unwrap();
	println!("{}", s);
	sstr();
	println!("Finished;");
}
