use std::fmt;
use crate::ext::*;
use std::error::Error;
use std::mem;

const MAP_SIZE: usize = 20;

static ERR_BOUNDS: &str = "Map: out of bounds";

pub struct Map {
	data: [[i32; MAP_SIZE]; MAP_SIZE],
	pos: Pos,
}

impl Map {
	pub fn width(&self) -> i64 {
		MAP_SIZE as i64
	}
	pub fn height(&self) -> i64 {
		MAP_SIZE as i64
	}
	pub fn new() -> Map {
		Map{data: Default::default(), pos: Pos::new()}
	}
	pub fn set_pos(&mut self, x: i64, y: i64) -> Result<()> {
		if (x < 0) | (y < 0) | (x >= self.width()) | (y >= self.height()) {
			return Err(ERR_BOUNDS.into());
		}
		self.pos.x = x;
		self.pos.y = y;
		Ok(())
	}
	pub fn bind_cell(&mut self, id: i32) -> Result<(Pos)> {
		let xc = MAP_SIZE / 2;
		let yc = MAP_SIZE / 2;
		if self.data[yc][xc] == 0 {
			self.data[yc][xc] = id;
			return Ok(Pos::init(xc as i64, yc as i64));
		}
		for _x in 0..10 {
			let xr = (get_rand(1).unwrap()[0] % MAP_SIZE as u8) as usize;
			let yr = (get_rand(1).unwrap()[0] % MAP_SIZE as u8) as usize;
			if self.data[yr][xr] == 0 {
				self.data[yr][xr] = id;
				return Ok(Pos::init(xr as i64, yr as i64));
			}
		}
		Err("Map: couldn't spawn after 10 attempts".into())
	}
	pub fn kill_cell(&mut self, pos: Pos) -> Result<()> {
		let n = &mut self.data[pos.y as usize][pos.x as usize];
		if *n > 0 {
			*n *= -1;
		}
		Ok(())
	}
	pub fn check_borders(&self, pos: Pos) -> Result<()> {
		let x = pos.x;
		let y = pos.y;
		if (x < 0) | (y < 0) | (x >= self.width()) | (y >= self.height()) {
			Err(ERR_BOUNDS.into())
		} else {
			Ok(())
		}
	}
	pub fn check_spot(&self, pos: Pos) -> Result<()> {
		if self.data[pos.y as usize][pos.x as usize] == 0 {
			Ok(())
		} else {
			Err("Spot is taken by another cell".into())
		}
	}
	pub fn move_cell(&mut self, pos: Pos, dir: Direction) -> Result<Pos> {
		let mut npos: Pos = pos;
		match dir {
			Direction::North => npos.y -= 1,
			Direction::East  => npos.x += 1,
			Direction::South => npos.y += 1,
			Direction::West  => npos.x -= 1,
		}
		let r = self.check_borders(npos);
		match r {
			Ok(_t) => (),
			Err(_t) => return Ok(pos),
		}
		match self.check_spot(npos) {
			Ok(_t)  => {
				match npos.y {
					d if d >  pos.y => println!("smth"),
					d if d <  pos.y => println!("smth"),
					d if d == pos.y => println!("smth"),
					_ => (),
				}
				/*
				let (x, y) = self.data.split_at_mut(npos.y as usize);
				mem::swap(&mut x[pos.y as usize][pos.x as usize], &mut y[0][npos.x as usize]);
				*/
				Ok(npos)
			}
			Err(_t) => Ok(pos),
		}
		/*
		println!(" pos {}", pos);
		println!("npos {}", npos);
		if self.data[npos.y as usize][npos.x as usize] == 0 {
			self.data[npos.y as usize][npos.x as usize] = self.data[pos.y as usize][pos.x as usize];
			self.data[pos.y as usize][pos.x as usize] = 0;
			Ok(npos)
		} else {
			Ok(pos)
		}
		*/
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for y in 0..MAP_SIZE {
			for x in 0..MAP_SIZE {
				if x == 0 {
					write!(f, "\t")?;
				}
				/*
				if self.pos.x == x as i64 && self.pos.y == y as i64 {
					write!(f, "{} ", 'V')?;
					continue;
				}
				*/
				match self.data[y][x] {
					0  => write!(f, "{} ", '·')?,
					d if d < 0 => write!(f, "{} ", 'x')?,
					d if d > 0 => write!(f, "{} ", '▸')?,
					_  => write!(f, "{} ", '▸')?,
					//_ => write!(f, "{} ", self.data[y][x])?,
				}
			}
			if y < MAP_SIZE - 1 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}
