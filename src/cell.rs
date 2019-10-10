use crate::ext::*;

pub struct Cell {
	pos: Pos,
}

impl Cell {
	pub fn new() -> Cell {
		Cell{pos: Pos::new()}
	}
}
