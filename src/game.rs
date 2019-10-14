use std::fmt;

use crate::map::*;
use crate::ext::*;
use crate::char::*;
use crate::cell::*;

pub struct Game {
	map: Map,
	char: Char,
	cells: Vec<Cell>,
	pos: Pos,
}

pub enum Direction {
	North,
	East,
	South,
	West,
}


impl Game {
	pub fn new(_x: i64, _y: i64) -> Game {
		Game {..Default::default()}
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
	pub fn spawn_cell(&mut self) -> Result<()> {
		let id = (self.cells.len() + 1) as u8;
		let r = self.map.bind_cell(id);
		println!("{}", CMD_SIZE);
		match r {
			Ok((t)) => {
				self.cells.push(Cell::new(id, t));
				println!("Created cell: {}", self.cells[self.cells.len() - 1]);
				return Ok(());
			}
			Err(t) => {
				return Err("no space".into());
			}
		}
		println!("Cells: {}", self.cells.len());
		Ok(())
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.map)?;
		Ok(())
	}
}

impl Default for Game {
	fn default() -> Game {
		Game {map: Map::new(), char: Char::new(), pos: Pos::new(), cells: Vec::new() /*Default::default*/ }
	}
}
