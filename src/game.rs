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
			Ok(t) => {
				self.cells.push(Cell::new(id, t));
				println!("Created cell: {}", self.cells[self.cells.len() - 1]);
				println!("Cells: {}", self.cells.len());
				return Ok(());
			}
			Err(_t) => {
				return Err("no space".into());
			}
		}
	}
	fn cell_tick(map: &mut Map, cell: &mut Cell, cmd: u8) {
		match cmd {
			0 => {
				let r = map.move_cell(cell.get_pos(), Action::MoveNorth).unwrap();
				cell.set_pos(r).unwrap();
			}
			1 => {
				let r =map.move_cell(cell.get_pos(), Action::MoveEast).unwrap();
				cell.set_pos(r).unwrap();
			}
			2 => {
				let r =map.move_cell(cell.get_pos(), Action::MoveSouth).unwrap();
				cell.set_pos(r).unwrap();
			}
			3 => {
				let r = map.move_cell(cell.get_pos(), Action::MoveWest).unwrap();
				cell.set_pos(r).unwrap();
			}
			_ => {
				
			}
		}
		//let x = cell.get_pos();
		//map.move_cell(cell.get_pos(), Action::MoveNorth);
	}
	pub fn world_tick(&mut self) -> Result <()> {
		//let mut x = 0;
		for cell in self.cells.iter_mut() {
			//println!("Cell {} got cmd: {}", x, cell.get_cmd().unwrap());
			//x += 1;
			let cmd = cell.get_cmd().unwrap();
			Game::cell_tick(&mut self.map, cell, cmd);
		}
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
