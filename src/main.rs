#![allow(dead_code)]
mod get_data;
mod puzzles;

use crate::puzzles::y2022::p02::puzzle;

fn main() {
	println!("{:?}", puzzle());
}
