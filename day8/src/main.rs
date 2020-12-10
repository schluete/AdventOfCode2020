//
// https://adventofcode.com/2020/day/8
//

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Command {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

impl Command {
  fn new(line: &String) -> Self {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let value = parts[1].parse::<i32>().unwrap();

    match parts[0] {
      "jmp" => return Command::Jmp(value),
      "acc" => return Command::Acc(value),
      _ => return Command::Nop(value)
    }
  }

  fn execute(&self, acc: &mut i32, pc: &mut usize) {
    match self {
      Command::Nop(_) => *pc += 1,
      Command::Acc(value) => { *acc += value ; *pc += 1 },
      Command::Jmp(value) => { *pc = ((*pc as i32) + *value) as usize },
    }
  }
}

fn interpreter(program: &Vec<Command>) -> bool {
  let mut acc = 0;
  let mut pc: usize = 0;
  let mut visited = HashSet::<usize>::new();
  loop {
    if pc >= program.len() {
      println!("program terminated, accumulator is {}!", acc);
      break true
    }
    if visited.contains(&pc) {
      println!("we're entering infinity, accumulator is {}", acc);
      break false;
    }
    visited.insert(pc);
    program[pc].execute(&mut acc, &mut pc);
  }
}

fn part2(program: &Vec<Command>) {
  let mut i: usize = 0;
  loop {
    let mut copy = program.clone();
    while i < program.len() {
      i += 1;
      if let Command::Nop(value) = copy[i] {
        println!("changing nop -> jmp at {}", i);
        copy[i] = Command::Jmp(value);
        break;
      } else if let Command::Jmp(value) = copy[i] {
        println!("changing jmp -> nop at {}", i);
        copy[i] = Command::Nop(value);
        break;
      }
    }
    if interpreter(&copy) {
      return;
    }
  }
}

fn main() {
  let file = File::open("input.txt")
    .expect("This should have been a file?!");

  let mut program = Vec::<Command>::new();
  for line in io::BufReader::new(file).lines() {
    program.push(Command::new(&line.unwrap()))
  }
  println!("we read {} commands", program.len());

  interpreter(&program);
  part2(&program)
}
