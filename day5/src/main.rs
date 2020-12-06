//
// https://adventofcode.com/2020/day/5
//

use std::fs::File;
use std::io::{self, BufRead};

struct Seat {
  row: i32,
  col: i32
}

impl Seat {
  pub fn new(config: &str) -> Result<Seat, &str> {
    if config.chars().count() != 10 {
      return Err("Invalid seat configuration")
    }

    fn binary_partition(chars: &Vec<char>, length: usize, offset: usize, upper: char) -> i32 {
      (offset..(offset + length)).fold(0, |total, next|
        if chars[next] == upper {
          (total << 1) | 1
        } else {
          (total << 1) & !1
        }) as i32
    }

    let chars: Vec<_> = config.chars().collect();
    Ok(Seat {
      row: binary_partition(&chars, 7, 0, 'B'),
      col: binary_partition(&chars, 3, 7, 'R')
    })
  }

  pub fn seat_id(&self) -> i32 {
    return self.row * 8 + self.col
  }
}

fn main() {
  let mut seat_ids = Vec::new();
  let mut highest_seat_id = 0;

  let file = File::open("input.txt").expect("This should have been a file?!");
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      match Seat::new(&line) {
        Err(err) => println!("unable to parse boarding pass: {:?}", err),
        Ok(seat) => {
          seat_ids.push(seat.seat_id());
          highest_seat_id = highest_seat_id.max(seat.seat_id())
        }
      }
    }
  }
  println!("the highest seat id is {}", highest_seat_id);

  seat_ids.sort();
  for i in 0..(seat_ids.len() - 1) {
    let curr_id = seat_ids[i];
    let next_id = seat_ids[i + 1];
    if next_id == curr_id + 2 {
      println!("your seat id is {}", curr_id + 1);
      return
    }
  }
}