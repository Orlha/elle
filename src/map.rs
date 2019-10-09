use std::fmt;
use crate::ext::*;
use std::error::Error;

const MAP_SIZE: usize = 4;

static ERR_BOUNDS: &str = "Map: out of bounds";

pub struct Map {
	//
	data: [[u8; MAP_SIZE]; MAP_SIZE],
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
	fn set_pos(&mut self, x: i64, y: i64) -> Result<()> {
		if (x < 0) | (y < 0) | (x >= self.width()) | (y >= self.height()) {
			return Err(ERR_BOUNDS.into());
		}
		self.pos.x = x;
		self.pos.y = y;
		Ok(())
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in 0..MAP_SIZE {
			for y in 0..MAP_SIZE {
				if y == 0 {
					write!(f, "\t")?;
				}
				if self.pos.x == x as i64 && self.pos.y == y as i64 {
					write!(f, "{} ", 'V')?;
					continue;
				}
				match self.data[x][y] {
					0 => write!(f, "{} ", '·')?,
					_ => write!(f, "{} ", self.data[x][y])?,
				}
			}
			if x < MAP_SIZE - 1 {
				write!(f, "\n")?;
			}
		}
		Ok(())
	}
}
