
use crate::ext::*;
use crate::game::*;

pub struct Engine {
	game: Game,
	on: bool,
}

impl Engine {
	pub fn new() -> Engine {
		Engine{game: Game::new(1, 2), on: true}
	}
	pub fn active(&self) -> bool {
		return self.on;
	}
	pub fn parse(&mut self, cmd: &String) {
		match cmd.as_ref() {
			"Exit" | "Q" | "q" => self.on = false,
			"W" | "w" => println!("Moving North;"),
			"A" | "a" => println!("Moving West;"),
			"S" | "s" => println!("Moving South;"),
			"D" | "d" => println!("Moving East;"),
			&_ => println!("Unknown command;"),
		}
	}
	pub fn output(&self) {
		println!("{}", self.game);
		return;
	}
}

pub fn cmdr(s: &String) {
	println!("Accepted command: {}", s);
}
