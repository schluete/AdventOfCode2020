//
// https://adventofcode.com/2020/day/2
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

// return true if the number of `what` characters in the
// `line' is within the range of min...max
fn validate_line_part1(line: &str, min: i32, max: i32, what: char) -> bool {
  let mut map = HashMap::new();
  for c in line.chars() {
    *(map.entry(c).or_insert(0)) += 1;
  }
  let count: &i32 = map.entry(what).or_insert(-1);
  return count >= &min && count <= &max
}

// return true if the `what` character is at either
// `pos1' or `pos2` in the `line`
fn validate_line_part2(line: &str, pos1: usize, pos2: usize, what: char) -> bool {
  let chars: Vec<char> = line.chars().collect();
  let found1 = chars[pos1 - 1] == what;
  let found2 = chars[pos2 - 1] == what;
  return (found1 && !found2) || (!found1 && found2)
}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

  let mut num_passwords = 0;
  let mut valid_passwords = 0;
  for line in io::BufReader::new(file).lines() {
    num_passwords += 1;
    if let Ok(line) = line {
      let cap = re.captures_iter(&line).next();
      if let Some(m) = &cap {
        if validate_line_part2(&m[4],
          m[1].parse().unwrap(),
          m[2].parse().unwrap(),
          m[3].parse().unwrap())
        {
          valid_passwords += 1;
        }
      } else  {
        println!("invalid list <{}>!?", line);
        return;
      }
    }
  }
  println!("we found {} valid password, out of {} (invalid: {}).",
    valid_passwords, num_passwords, num_passwords - valid_passwords);
}
