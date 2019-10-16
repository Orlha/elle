use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::map::*;
use crate::ext::*;
use crate::char::*;
use crate::cell::*;

pub struct Game {
	map: Map,
	char: Char,
	//cells: Vec<Cell>,
	pos: Pos,
	turn: i64,
	cellsm: HashMap<i32, Cell>,
	id_reuse: Vec<i32>,
}

impl Game {
	pub fn new(_x: i64, _y: i64) -> Game {
		//
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
		//let id = (self.cells.len() + 1) as i32;
		let id = if self.id_reuse.len() > 0 {
			self.id_reuse.pop().unwrap()
		} else {
			self.cellsm.len() as i32
		};
		let r = self.map.bind_cell(id);
		match r {
			Ok(t) => {
				self.cellsm.insert(id, Cell::new(id, t));
				/*
				self.cells.push(Cell::new(id, t));
				*/
				println!("Created cell:\n{}", self.cellsm[&(self.cellsm.len() as i32 - 1)]);
				println!("Total cells: {}", self.cellsm.len());
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
		let mut deads: HashSet<i32> = HashSet::new();
		println!("{:?}", deads);
		println!("map len {}", self.cellsm.len());
		//for cell in self.cells.iter_mut() {
		for (id, cell) in self.cellsm.iter_mut() {
			println!("calculation cell");
			if deads.contains(&cell.get_id()) {
				continue;
			}
			x += 1;
			if cell.alive() {
				let mut x = ENERGY_TOP - ENERGY_DROP * cell.get_pos().y;
				if x < 0 { x = 0; }
				cell.gain_energy(x);
				let mut cmd = cell.get_cmd().unwrap() % CMD_SIZE as u8;

				//cmd = 1;
				match cmd {
					0 => {
						println!("Move");
						let cmd = cell.get_cmd().unwrap() % DIR_SIZE as u8;
						let dir = Game::get_direction(cmd).unwrap();
						let r = self.map.cell_move(cell.get_id(), cell.get_pos(), dir).unwrap();
						cell.set_pos(r).unwrap();
						cell.gain_energy(-1);
					}
					1 => {
						println!("Feast");
						let r = self.map.cell_feast(cell.get_id(), cell.get_pos()).unwrap();
						if r != 0 {
							deads.insert(r);
						}
					}
					2 => {
						println!("Scavenge");
					}
					3 => println!("Nothing"),
					_ => return Err("Invalid command;".into()),
				}
				// println!("{}", cell);
			} else {
				self.map.kill_cell(cell.get_pos())?;
				let r = self.map.cell_move(cell.get_id(), cell.get_pos(), Direction::South).unwrap();
				cell.set_pos(r).unwrap();
			}
		}
		println!("Processed {} cells;", x);
		self.turn += 1;

		let mut count_vec: Vec<(&i32)> = deads.iter().collect();
		count_vec.sort_by(|a, b| b.cmp(a));
		println!("deads count: {}", deads.len());
		println!("cells count: {}", self.cellsm.len());
		for x in count_vec {
			println!("removing id {}", *x);
			//self.cells.remove(*x as usize - 1);
			self.cellsm.remove(&(*x - 1));
		}
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
			//cells: Vec::new(),
			cellsm: HashMap::new(),
			id_reuse: Vec::new(),
		}
	}
}
