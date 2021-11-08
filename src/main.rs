#![allow(dead_code)]
mod puzzles;
mod get_data;

use crate::puzzles::p1509::puzzle;

fn main() {
    println!("{:?}", puzzle());
}
