//
// https://adventofcode.com/2020/day/3
//

use std::fs::File;
use std::io::{self, BufRead};

fn check_slope(lines: &Vec<String>, right: usize, down: usize) -> i64 {
  let mut num_trees = 0;
  let mut line = 0;
  let mut pos: usize = 0;
  while line < lines.len() {
    let chars: Vec<char> = lines[line].chars().collect();
    if chars[pos % chars.len()] == '#' {
      num_trees += 1
    }
    pos += right;
    line += down
  }
  return num_trees
}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let lines: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();
  let num_trees =
    check_slope(&lines, 1, 1) *
    check_slope(&lines, 3, 1) *
    check_slope(&lines, 5, 1) *
    check_slope(&lines, 7, 1) *
    check_slope(&lines, 1, 2);
  println!("we found {} trees along the path", num_trees)
}
