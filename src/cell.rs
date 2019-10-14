use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::ext::*;

const TAPE_SIZE: usize = 32;
pub const CMD_SIZE: usize = 4;

pub struct Cell {
	id: u8,
	pos: Pos,
	tape: [u8; TAPE_SIZE],
	hand: usize,
	energy: i64,
	alive: bool,
}

impl Default for Cell {
	fn default() -> Cell {
		Cell{alive: true, energy: 50, id: 0, pos: Pos::new(), hand: 0, tape: Default::default()}
	}
}

impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		writeln!(f, "  id: {}", self.id)?;
		writeln!(f, " pos: {} {}", self.pos.x, self.pos.y)?;
		write!  (f, "tape: {:02X?}", self.tape)
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
	pub fn get_pos(&self) -> Pos {
		Pos{x: self.pos.x, y: self.pos.y}
	}
	pub fn get_cmd(&mut self) -> Result<u8> {
		let r = self.tape[self.hand];
		self.hand += 1;
		if self.hand == TAPE_SIZE {
			self.hand = 0;
		}
		Ok(r)
	}
	pub fn gain_energy(&mut self, n: i64) {
		self.energy += n;
	}
	pub fn alive(&self) -> bool {
		self.alive
	}
}
