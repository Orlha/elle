use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::ext::*;

pub enum Action {
	MoveNorth,
	MoveEast,
	MoveSouth,
	MoveWest,
}

const TAPE_SIZE: usize = 32;
pub const CMD_SIZE: usize = 4;

pub struct Cell {
	id: u8,
	pos: Pos,
	//tape: Vec<Action>,
	//tape: [u8; TAPE_SIZE],
	tape: [u8; TAPE_SIZE],
}

impl Default for Cell {
	fn default() -> Cell {
		Cell{id: 0, pos: Pos::new(), tape: Default::default()}
	}
}

impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		writeln!(f, "Cell: {}", self.id);
		writeln!(f, "Pos: {} {}", self.pos.x, self.pos.y);
		write!(f, "Tape: {:02X?}", self.tape)
	}
}

impl Cell {
	pub fn new(id: u8, pos: Pos) -> Cell {
		let mut c = Cell{..Default::default()};
		c.id = id;
		c.pos = pos;
		let rnd = get_rand(TAPE_SIZE).unwrap();
		for i in 0..TAPE_SIZE {
			c.tape[i] = rnd[i] % CMD_SIZE as u8;
		}
		return c;
	}
	pub fn set_pos(&mut self, pos: Pos) -> Result<()> {
		self.pos = pos;
		Ok(())
	}
}
