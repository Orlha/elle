extern crate termion;

struct Game {
	x: i32,
	y: i32,
}

impl Game {
	fn rdo(&self) {
		return;
	}
}

fn clear_screen()
{
	//
	println!("{}", termion::clear::All);
}

fn main() {
	clear_screen();
	println!("Hello, world!");
}
