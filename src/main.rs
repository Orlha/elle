#![allow(dead_code)]
#![allow(unused_imports)]
extern crate termion;

mod ext;
mod cmdr;
mod game;
mod map;
mod char;

use ext::*;
use cmdr::*;
use std::fmt;
use std::io;
use std::env;
use std::io::{stdin, stdout, Write};

fn run() -> Result<()> {
	/*
	let x = ext::get_rand(5)?;
	println!("{:?}", x);
	let y = ext::get_rand(5)?;
	println!("{:?}", y);
	*/

	let mut engine = Engine::new();
	let _ = engine.req_cmd();
	engine.clear_screen();
	loop {
		let s = engine.req_cmd();
		engine.clear_screen();
		engine.parse(&s).ok();
		engine.output();
		if !engine.active() {
			break;
		}
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
