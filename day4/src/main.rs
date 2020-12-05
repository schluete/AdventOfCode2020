//
// https://adventofcode.com/2020/day/4
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

struct Validator {

}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let re = Regex::new(r"(\w+):\S+").unwrap();

  let mut passport = HashSet::new();
  let mut num_passwports = 0;
  let mut num_valid_passwports = 0;
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      println!("{}", line);
      for cap in re.captures_iter(&line) {
        // println!("cap: {}, {}", &cap[0], &cap[1]);
        let key = String::from(&cap[1]);
        passport.insert(key);
      }

      if line.chars().count() == 0 {
        let is_valid = passport.len() == 8 || (passport.len() == 7 && !passport.contains("cid"));
        if is_valid {
          num_valid_passwports += 1
        }
        num_passwports += 1;
        passport = HashSet::new();
        // println!("we're done with a password, {}, has cid: {}", passport.len(), passport.contains("cid"));
        // return;
      }
      println!("\n")
    }
  }
  println!("we found {} passports, {} have been valid.", num_passwports, num_valid_passwports)
}