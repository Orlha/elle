use std::fmt;

use crate::map::*;
use crate::ext::*;

pub struct Game {
	map: Map,
	x: i64,
	y: i64,
}

impl Game {
	pub fn new(x: i64, y: i64) -> Game {
		Game {map: Map::new(), x: x, y: y}
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}\n", self.x, self.y)?;
		write!(f, "map:\n{}", self.map)?;
		Ok(())
	}
}
