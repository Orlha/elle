use crate::ext;

pub struct Char {
	pub pos: Pos,
}

impl Char {
	pub fn new() -> Char {
		Char {x: 0, y: 0}
	}
	pub fn pos(&self) -> Pos {
		Pos {x: self.x, y: self.y}
	}
}
