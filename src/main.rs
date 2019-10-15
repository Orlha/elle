// Macro
#![allow(dead_code)]
#![allow(unused_imports)]
// Externals
extern crate termion;
// Modules
mod ext;
mod cmdr;
mod game;
mod map;
mod char;
mod cell;
// Imports
use ext::Result;
use cmdr::*;
use std::env;
// ----
fn run() -> Result<()> {
	let mut engine = Engine::new();
	//engine.req_cmd();
	//engine.clear_screen();
	loop {
		let s = engine.req_cmd();
		engine.clear_screen();
		println!("Log:");
		engine.parse(&s).ok();
		engine.process();
		engine.output();
		if !engine.active() { break;}
	}
	Ok(())
}

fn main() {
	println!("[Started];");
	let argv = &env::args();
	println!("{:?}", argv);
	match run() {
		Ok(()) => println!("Finished successfully;"),
		Err(t) => println!("Finished with error: {}", t),
	}
	println!("[Exited];");
}
