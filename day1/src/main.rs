//
// https://adventofcode.com/2020/day/1
//

use std::fs::File;
use std::io;
// use std::io::{self, BufRead};
// use std::path::Path;

fn main() {
  println!("Hello, world!");

  // let file = File::open("input.txt")?;
  let file = File::open("input.txt");

  let lines = Ok(io::BufReader::new(file).lines());
}
