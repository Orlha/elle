/*
 * Module CMDR (Command/er) | Engine
 */

use std::io::{stdin, stdout, Write};

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
	pub fn req_cmd(&self) -> String {
		print!("command: ");
		stdout().flush().unwrap();
		let s = read_string().unwrap();
		return s;
	}
	pub fn parse(&mut self, cmd: &String) -> Result<()> {
		match cmd.as_ref() {
			"Exit" | "Q" | "q" => self.on = false,
			"W" | "w" => {
				println!("Moving North;");
				self.game.char_move(Direction::North);
			}
			"D" | "d" => {
				println!("Moving East;");
				self.game.char_move(Direction::East);
			}
			"S" | "s" => {
				println!("Moving South;");
				self.game.char_move(Direction::South);
			}
			"A" | "a" => {
				println!("Moving West;");
				self.game.char_move(Direction::West);
			}
			&_ => return Err("Unknown command".into()),
		}
		Ok(())
	}
	pub fn output(&self) {
		println!("{}", self.game);
		return;
	}
	pub fn clear_screen(&self) {
		println!("{}", termion::clear::All);
		//
	}
}
