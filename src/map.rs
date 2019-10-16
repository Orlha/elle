use std::fmt;
use crate::ext::*;
use crate::cell::*;
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
		let y = pos.y;
		println!("y = {}", y);
		if (y < 0) | (y >= self.height()) {
			println!("got bad borders");
			Err(ERR_BOUNDS.into())
		} else {
			println!("got good borders");
			Ok(())
		}
	}
	pub fn get_spot_status(&self, pos: Pos) -> Result<Spot> {
		let r = self.check_borders(pos);
		match r {
			Ok(_) => (),
			Err(_) => return Ok(Spot::Invalid),
		}
		let x = self.data[pos.y as usize][pos.x as usize];
		match x {
			0 => Ok(Spot::Empty),
			d if d > 0 => Ok(Spot::Alive),
			d if d < 0 => Ok(Spot::Dead),
			_ => Err("Invalid spot value;".into()),
		}
	}
	pub fn cell_move(&mut self, id: i32, pos: Pos, dir: Direction) -> Result<Pos> {
		let mut npos: Pos = pos;
		match dir {
			Direction::North => npos.y -= 1,
			Direction::East  => npos.x += 1,
			Direction::South => npos.y += 1,
			Direction::West  => npos.x -= 1,
		}
		match self.check_borders(npos) {
			Ok(_) => (),
			Err(_) => return Ok(pos),
		}

		match npos.x {
			d if d < 0 => npos.x = MAP_SIZE as i64 - 1,
			d if d == MAP_SIZE as i64 => npos.x = 0,
			_ => (),
		}

		println!("will crash right now");
		println!("npos {}", npos);
		match self.check_spot_status(npos, Spot::Empty) {
			Ok(_) => {
				self.data[pos.y as usize][pos.x as usize] = 0;
				self.data[npos.y as usize][npos.x as usize] = id;
				Ok(npos)
			}
			_ => Ok(pos),
		}
	}
	fn check_spot_status(&mut self, pos: Pos, spot: Spot) -> Result<()> {
		/*
		match self.get_spot_status(pos).unwrap() {
			spot => {
				println!("{:?}", self.get_spot_status(pos).unwrap());
				Ok(())
			}
			_ => Err("No valid status on questioned spot;".into()),
		}
		*/
		let n = self.get_spot_status(pos).unwrap();
		println!("spot status: {:?}", n);
		println!("expc status: {:?}", spot);
		if n == spot {
			println!("Matched!");
			return Ok(());
		} else {
			return Err("Err".into());
		}
	}
	pub fn cell_cell(&self, cell: &mut Cell) {
	}
	pub fn cell_feast(&mut self, id: i32, pos: Pos) -> Result<i32> {
		let mut x: Vec<Pos> = vec!{};
		let pos_north = Pos::init(pos.x, pos.y - 1);
		let mut pos_east  = Pos::init(pos.x + 1, pos.y);
		let pos_south = Pos::init(pos.x, pos.y + 1);
		let mut pos_west  = Pos::init(pos.x - 1, pos.y);

		if pos_east.x == MAP_SIZE as i64 {
			pos_east.x = 0;
		}
		if pos_west.x == -1 {
			pos_west.x = MAP_SIZE as i64 - 1;
		}

		match self.check_spot_status(pos_north, Spot::Alive) {
			Ok(_) => {
				x.push(pos_north);
				println!("added north");
			}
			_ => (),
		}
		match self.check_spot_status(pos_east, Spot::Alive) {
			Ok(_) => {
				x.push(pos_east);
				println!("added east");
			}
			_ => (),
		}
		match self.check_spot_status(pos_south, Spot::Alive) {
			Ok(_) => {
				x.push(pos_south);
				println!("added south");
			}
			_ => (),
		}
		match self.check_spot_status(pos_west, Spot::Alive) {
			Ok(_) => {
				x.push(pos_west);
				println!("added west");
			}
			_ => (),
		}
		if x.len() == 0 {
			return Ok(0);
		}
		println!("cell {} is trying to feast in one of {} directions", id, x.len());
		let r = get_rand(1).unwrap()[0]  as usize % x.len();
		println!("crashing pos {}", x[r]);
		let n = self.data[x[r].y as usize][x[r].x as usize];
		println!("cell {} is dead", n);
		self.data[x[r].y as usize][x[r].x as usize] = 0;
		Ok(n)
	}
	pub fn cell_scavenge(&mut self, id: i32, pos: Pos) -> Result<i32> {
		Ok(5)
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
