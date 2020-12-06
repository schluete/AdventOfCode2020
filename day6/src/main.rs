//
// https://adventofcode.com/2020/day/6
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

struct Group {
  questions: HashSet<char>,
  is_first_person: bool
}

impl Group {
  pub fn new() -> Group {
    Group {
      questions: HashSet::new(),
      is_first_person: true
    }
  }

  pub fn add_person(&mut self, line: &String) {
    let mut questions = HashSet::new();
    for c in line.chars() {
      questions.insert(c);
    }

    if self.is_first_person {
      self.questions = questions;
      self.is_first_person = false
    } else {
      self.questions = self.questions.intersection(&questions).cloned().collect()
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
