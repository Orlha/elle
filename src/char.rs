use crate::ext::*;

pub struct Char {
	pos: Pos,
}

impl Char {
	pub fn new() -> Char {
		Char {pos: Pos::new()}
	}
	pub fn pos(&self) -> Pos {
		return self.pos;
	}
	pub fn set_pos(&mut self, pos: &Pos) {
		self.pos = *pos;
	}
}
