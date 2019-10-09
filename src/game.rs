use std::fmt;

use crate::map::*;
use crate::ext::*;
use crate::char::*;

pub struct Game {
	map: Map,
	char: Char,
	x: i64,
	y: i64,
}

pub enum Direction {
	North,
	East,
	South,
	West,
}

impl Game {
	pub fn new(x: i64, y: i64) -> Game {
		Game {map: Map::new(), char: Char::new(), x: x, y: y}
	}
	pub fn char_move(&mut self, dir: Direction) {
		let mut pos = self.char.pos();
		match dir {
			Direction::North => pos.y -= 1,
			Direction::East  => pos.x += 1,
			Direction::South => pos.y += 1,
			Direction::West  => pos.x -= 1,
		}
		let r = self.map.set_pos(pos.x, pos.y);
		match r {
			Ok(()) => {
				self.char.set_pos(&pos);
			}
			Err(_t) => (),
		}
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "pos: {}", self.char.pos())?;
		write!(f, "map:\n{}", self.map)?;
		Ok(())
	}
}
