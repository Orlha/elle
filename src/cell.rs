use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::ext::*;

const TAPE_SIZE: usize = 32;

pub struct Cell {
	id: i32,
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
		writeln!(f, " nrg: {}", self.energy)?;
		//write!  (f, "tape: {:02X?}", self.tape)
		write!(f, "lives: {}", self.alive)
	}
}

impl Cell {
	pub fn new(id: i32, pos: Pos) -> Cell {
		let mut c = Cell{..Default::default()};
		c.id = id;
		c.pos = pos;
		let rnd = get_rand(TAPE_SIZE).unwrap();
		for i in 0..TAPE_SIZE {
			// c.tape[i] = rnd[i] % CMD_SIZE as u8;
			c.tape[i] = rnd[i];
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
	pub fn get_id(&self) -> i32 {
		self.id
	}
	pub fn get_cmd(&mut self) -> Result<u8> {
		let r = self.tape[self.hand];
		self.hand += 1;
		if self.hand == TAPE_SIZE {
			self.hand = 0;
		}
		Ok(r)
	}
	pub fn may_divide(&self) -> bool {
		if self.energy > 95 {
			true
		} else {
			false
		}
	}
	pub fn gain_energy(&mut self, n: i64) {
		let energy: &mut i64 = &mut self.energy;
		*energy += n;
		if *energy <= 0 {
			self.alive = false;
			*energy = 0;
			return;
		}
		if *energy > 100 {
			*energy = 100;
			return;
		}
	}
	pub fn alive(&self) -> bool {
		self.alive
	}
}
