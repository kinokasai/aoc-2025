use std::{env::args, fs};

fn parse(filename: &str) -> Vec<Vec<u64>> {
  let s = fs::read_to_string(filename).unwrap();
  s.trim()
    .split("\n")
    .map(|s| {
      s.chars()
        .map(|c| char::to_digit(c, 10).unwrap())
        .map(|n| n as u64)
        .collect::<Vec<u64>>()
    })
    .collect::<Vec<Vec<u64>>>()
}

fn get_min_idx(slice: &[u64]) -> usize {
  let mut idx = 0;
  let mut min = slice[0];
  for i in 0..slice.len() {
    if slice[i] < min {
      idx = i;
      min = slice[i];
    }
  }
  idx
}

fn shift_right(slice: &mut [u64], i: usize) {
  for i in (1..=i).rev() {
    slice[i] = slice[i - 1]
  }
}

fn max_power(bank: Vec<u64>, digits: usize) -> u64 {
  println!("bank: {:?}", bank);
  let max_power = &mut bank.clone()[bank.len() - digits..];
  println!("max_power: {:?}", max_power);
  for i in bank.iter().rev().skip(digits) {
    if *i < max_power[0] {
      continue;
    }
    let idx = get_min_idx(max_power);
    shift_right(max_power, idx);
    max_power[0] = *i;
    println!("max_power: {:?}", max_power);
  }
  let (_, power ) = max_power.iter().rev().fold((1u64, 0u64), |(exp, acc), e|
    (exp * 10,acc + (e * exp)));
  power
}

fn main() {
  let banks = parse(&args().nth(1).unwrap());
  let power = banks
    .into_iter()
    .map(|bank| max_power(bank, 12))
    .reduce(|acc, e| acc + e)
    .unwrap_or(0);
  println!("{power}");
}
