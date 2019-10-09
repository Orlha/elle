#![allow(dead_code)]
#![allow(unused_imports)]
extern crate termion;

mod ext;
mod cmdr;

use ext::*;
use cmdr::*;
use std::fmt;
use std::io;
use std::env;

fn sstr() {
	let x = "Qwe";
	match x {
		"Qwe" => {
			println!("Qwe yourself!");
			println!("Qwe yourself!");
			println!("Qwe yourself!");
		}
		"biteMe" => println!("Bite my _Rusty_ metal ass!"),
		"killYa" => println!("KILL ALL HUMANS!"),
		"letsGo" => println!("Let's GO ALREADYYY..."),
		_        => println!("Oh. Your. God.")
	}
}

fn run() -> Result<()> {
	let x = ext::get_rand(5)?;
	println!("{:?}", x);

	let y = ext::get_rand(5)?;
	println!("{:?}", y);

	let game = Game::new(1, 2);
	println!("{}", game);

	loop {
		let s = read_string().unwrap();
		cmdr(&s);
		match s.as_ref() {
			"Exit" => {
				break;
			},
			&_ => {
				continue;
			}
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
	sstr();
	println!("[Exited];");
}
