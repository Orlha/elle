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
	turn: i64,
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
		let id = (self.cells.len() + 1) as i32;
		let r = self.map.bind_cell(id);
		match r {
			Ok(t) => {
				self.cells.push(Cell::new(id, t));
				println!("Created cell:\n{}", self.cells[self.cells.len() - 1]);
				println!("Total cells: {}", self.cells.len());
				return Ok(());
			}
			Err(_t) => {
				return Err("no space".into());
			}
		}
	}
	pub fn get_direction(dir: u8) -> Result<Direction> {
		match dir {
			0 => Ok(Direction::North),
			1 => Ok(Direction::East),
			2 => Ok(Direction::South),
			3 => Ok(Direction::West),
			_ => Err("Invalid direction;".into()),
		}
	}
	pub fn world_tick(&mut self) -> Result <()> {
		let mut x = 0;
		for cell in self.cells.iter_mut() {
			x += 1;
			if cell.alive() {
				let mut x = ENERGY_TOP - ENERGY_DROP * cell.get_pos().y;
				if x < 0 { x = 0; }
				cell.gain_energy(x);
				let cmd = cell.get_cmd().unwrap() % CMD_SIZE as u8;

				match cmd {
					0 => {
						let cmd = cell.get_cmd().unwrap() % DIR_SIZE as u8;
						let dir = Game::get_direction(cmd).unwrap();
						let r = self.map.move_cell(cell.get_pos(), dir).unwrap();
						cell.set_pos(r).unwrap();
						cell.gain_energy(-1);
						println!("Move");
					}
					1 => println!("Feast"),
					2 => println!("Nothing"),
					_ => return Err("Invalid command;".into()),
				}
				// println!("{}", cell);
			} else {
				self.map.kill_cell(cell.get_pos())?;
				let r = self.map.move_cell(cell.get_pos(), Direction::South).unwrap();
				cell.set_pos(r).unwrap();
			}
		}
		println!("Processed {} cells;", x);
		self.turn += 1;
		Ok(())
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Turn: {}", self.turn)?;
		write!(f, "{}", self.map)?;
		Ok(())
	}
}

impl Default for Game {
	fn default() -> Game {
		Game {
			turn: 0,
			map: Map::new(),
			char: Char::new(),
			pos: Pos::new(),
			cells: Vec::new()
		}
	}
}
