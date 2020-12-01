//
// https://adventofcode.com/2020/day/1
//

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let mut numbers = Vec::<i32>::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      numbers.push(line.parse::<i32>().unwrap());
    }
  }
  numbers.sort();

  while let Some(large) = numbers.pop() {
    for outer in 1..numbers.len() {
      let middle = numbers[numbers.len() - outer];
      for inner in 1..outer {
        let small = numbers[numbers.len() - inner];
        if large + middle + small == 2020 {
          println!("we've found it: {} + {} + {} = 2020, solution is {}",
          large, middle, small, large * middle * small);
        }
      }
    }

    // for i in 1..numbers.len() {
    //   let small = numbers[numbers.len() - i];
    //   if large + small == 2020 {
    //     println!("we've found it: {} + {} = 2020, solution is {}", large, small, large * small);
    //   }
    // }
  }
}
