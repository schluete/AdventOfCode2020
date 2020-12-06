//
// https://adventofcode.com/2020/day/6
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

struct Group {
  questions: HashSet<char>
}

impl Group {
  pub fn new() -> Group {
    Group {
      questions: HashSet::new()
    }
  }

  pub fn add_person(&mut self, line: &String) {
    for c in line.chars() {
      self.questions.insert(c);
    }
  }

  pub fn number_of_questions(&self) -> usize {
    self.questions.len()
  }
}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let mut group = Group::new();
  let mut num_yes_questions = 0;
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      if line.chars().count() == 0 {
        num_yes_questions += group.number_of_questions();
        group = Group::new()
      } else {
        group.add_person(&line)
      }
    }
  }
  num_yes_questions += group.number_of_questions();
  println!("all groups together had {} yes questions", num_yes_questions)
}
