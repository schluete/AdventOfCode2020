//
// https://adventofcode.com/2020/day/4
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

struct Validator {
  passport: HashSet<String>,
  num_passports: i32,
  num_valid_passports: i32
}

impl Validator {
  pub fn new() -> Validator {
    Validator {
      passport: HashSet::new(),
      num_passports: 0,
      num_valid_passports: 0
    }
  }

  fn validate_passport(&mut self) {
    let is_valid =
      self.passport.len() == 8 ||
      (self.passport.len() == 7 && !self.passport.contains("cid"));
    if is_valid {
      self.num_valid_passports += 1
    }
    self.num_passports += 1
  }

  pub fn parse(&mut self, filename: &str) -> (i32, i32) {
    let re = Regex::new(r"(\w+):\S+").unwrap();

    let file = File::open(filename)
      .expect("This should have been a file?!");
    for line in io::BufReader::new(file).lines() {
      if let Ok(line) = line {
        // println!("{}", line);
        for cap in re.captures_iter(&line) {
          let key = String::from(&cap[1]);
          self.passport.insert(key);
        }

        if line.chars().count() == 0 {
          self.validate_passport();
          self.passport = HashSet::new();
        }
      }
    }
    self.validate_passport();
    return (self.num_passports, self.num_valid_passports)
  }
}

fn main() {
  let mut validator = Validator::new();
  let (passports, valid_passports) = validator.parse("input.txt");
  println!("we found {} passports, {} have been valid.", passports, valid_passports)
}