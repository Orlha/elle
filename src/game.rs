use std::fmt;

use crate::map::*;
use crate::ext::*;

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
	pub fn char_move(dir: Direction) {
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}\n", self.x, self.y)?;
		write!(f, "map:\n{}", self.map)?;
		Ok(())
	}
}
