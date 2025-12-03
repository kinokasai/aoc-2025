use std::{env::args, fs};

fn parse(filename: &str) -> Vec<Vec<u32>> {
  let s = fs::read_to_string(filename).unwrap();
  s.trim()
    .split("\n")
    .map(|s| {
      s.chars()
        .map(|c| char::to_digit(c, 10).unwrap())
        .collect::<Vec<u32>>()
    })
    .collect::<Vec<Vec<u32>>>()
}

fn max_power(bank: &Vec<u32>) -> u32 {
  println!("bank: {:?}", bank);
  let mut higher;
  let mut lower;
  if bank[0] > bank[1] {
    higher = bank[0];
    lower = bank[1];
  } else {
    higher = bank[1];
    lower = bank[2];
  };

  for i in 2..bank.len() - 1{
    if bank[i] > higher {
      higher = bank[i];
      lower = bank[i+1];
    } else if bank[i] > lower {
      lower = bank[i];
    }
  }
  let last = *bank.last().unwrap();
  if lower < last {
    lower = last;
  }
  println!("higher: {higher} | lower: {lower}"); 
  let power = higher * 10 + lower;
  println!("power: {power}");
  power
}

fn main() {
  let banks = parse(&args().nth(1).unwrap());
  let power = 
    banks.iter().map(|bank|
    max_power(bank)).reduce(|acc,e| acc + e).unwrap_or(0);
  println!("{power}");
}
