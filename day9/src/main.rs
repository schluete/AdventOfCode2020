//
// https://adventofcode.com/2020/day/9
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn combinations(numbers: &Vec<i64>, offset: usize, preamble: usize) -> HashMap<i64, (i64, i64)> {
  let mut sums = HashMap::<i64, (i64, i64)>::new();

  let nums = &numbers[offset .. (offset + preamble)];
  for (i, n1) in nums.iter().enumerate() {
    for n2 in nums[i + 1 .. nums.len()].iter() {
      sums.insert(n1 + n2, (*n1, *n2));
    }
  }
  return sums
}

fn contiguous_set_for_sum(numbers: &Vec<i64>, sum: i64) -> Option<Vec<i64>> {
  let mut result = Vec::<i64>::new();
  for i in 0..numbers.len() {
    result = Vec::<i64>::new();
    result.push(numbers[i]);
    let mut curr_sum = numbers[i];

    for u in i + 1 .. numbers.len() {
      curr_sum += numbers[u];
      result.push(numbers[u]);
      if curr_sum == sum {
        result.sort();
        return Some(result)
      } else if curr_sum > sum {
        break
      }
    }
  }
  result.sort();
  return None
}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");
  let preamble = 25;

  let mut numbers = Vec::<i64>::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      numbers.push(line.parse::<i64>().unwrap())
    }
  }
  println!("we read {} numbers", numbers.len());

  let mut failed = 0;
  for (i, num) in numbers[preamble .. numbers.len()].iter().enumerate() {
    let combs = combinations(&numbers, i, preamble);
    if !combs.contains_key(num) {
      failed = *num;
      println!("{} is not a sum from the other numbers?!", num);
      break
    }
  }

  let smallest_range = contiguous_set_for_sum(&numbers, failed).unwrap();
  let result = smallest_range[0] + smallest_range[smallest_range.len() - 1];
  println!("range length is {}, with the sum being {}", smallest_range.len(), result)
}
