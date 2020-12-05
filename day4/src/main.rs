//
// https://adventofcode.com/2020/day/4
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

struct Validator {
  passport: HashMap<String, String>,
  num_passports: i32,
  num_valid_passports: i32
}

impl Validator {
  pub fn new() -> Validator {
    Validator {
      passport: HashMap::new(),
      num_passports: 0,
      num_valid_passports: 0
    }
  }

  // byr (Birth Year)      - four digits; at least 1920 and at most 2002.
  // iyr (Issue Year)      - four digits; at least 2010 and at most 2020.
  // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
  fn is_valid_year(&self, field: &str, min: i32, max: i32) -> bool {
    if let Some(value) = self.passport.get(field) {
      if let Ok(year) = value.parse::<i32>() {
        return year >= min && year <= max
      }
    }
    return false
  }

  // hgt (Height) - a number followed by either cm or in:
  // If cm, the number must be at least 150 and at most 193.
  // If in, the number must be at least 59 and at most 76.
  fn is_valid_height(&self) -> bool {
    let re = Regex::new(r"^(\d+)(cm|in)").unwrap();
    if let Some(height) = self.passport.get("hgt") {
      if let Some(cap) = re.captures_iter(height).next() {
        let unit = &cap[2];
        let value = &cap[1].parse::<i32>().unwrap();
        return
          (unit == "cm" && *value >= 150 && *value <= 193) ||
          (unit == "in" && *value >= 59 && *value <= 76)
      }
    }
    return false
  }

  // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  // pid (Passport ID) - a nine-digit number, including leading zeroes.
  fn matches(&self, field: &str, pattern: &str) -> bool {
    if let Some(value) = self.passport.get(field) {
      let re = Regex::new(pattern).unwrap();
      return re.is_match(value)
    }
    return false
  }

  fn validate_passport(&mut self) {
    let has_cid = self.passport.contains_key("cid");
    let has_all_fields =
      self.passport.len() == 8 ||
      (self.passport.len() == 7 && !has_cid);
    if has_all_fields &&
      self.is_valid_year("byr", 1920, 2002) &&
      self.is_valid_year("iyr", 2010, 2020) &&
      self.is_valid_year("eyr", 2020, 2030) &&
      self.matches("hcl", r"^#[0-9a-f]{6}$") &&
      self.matches("ecl", r"^(amb|blu|brn|gry|grn|hzl|oth)$") &&
      self.matches("pid", r"^[0-9]{9}$") &&
      self.is_valid_height()
    {
      self.num_valid_passports += 1
    }
    self.num_passports += 1
  }

  pub fn parse(&mut self, filename: &str) -> (i32, i32) {
    let re = Regex::new(r"(\w+):(\S+)").unwrap();

    let file = File::open(filename)
      .expect("This should have been a file?!");
    for line in io::BufReader::new(file).lines() {
      if let Ok(line) = line {
        // println!("{}", line);
        for cap in re.captures_iter(&line) {
          let key = String::from(&cap[1]);
          let value = String::from(&cap[2]);
          self.passport.insert(key, value);
        }

        if line.chars().count() == 0 {
          self.validate_passport();
          self.passport = HashMap::new();
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