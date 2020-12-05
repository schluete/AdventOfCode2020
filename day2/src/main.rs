//
// https://adventofcode.com/2020/day/2
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn validate_line(line: &str, min: i32, max: i32, what: char) -> bool {
  println!("xxxm {} to {}, for {} in {}", min, max, what, line);

  let mut map = HashMap::new();
  for c in line.chars() {
    *(map.entry(c).or_insert(0)) += 1;
    println!("{}", c)
  }
  let count: &i32 = map.entry(what).or_insert(-1);
  println!("count is {}", count);
  return count >= &min && count <= &max
}


fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let cap = re.captures_iter(&line).next();
      if let Some(m) = &cap {
        validate_line(&m[4],
          m[1].parse().unwrap(),
          m[2].parse().unwrap(),
          m[3].parse().unwrap());
return;
      } else  {
        println!("invalid list <{}>!?", line);
        return;
      }
    }
  }
}
