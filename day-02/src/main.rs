use std::{env::args, fs};

#[derive(Debug)]
struct Range {
  low: i64,
  high: i64,
}

fn parse(filename: &str) -> Vec<Range> {
  let s = fs::read_to_string(filename).unwrap();
  let s = s.trim();
  let s = s
    .split(",")
    .map(|s| s.split("-").collect::<Vec<&str>>())
    .map(|arr| {
      println!("{:?}", arr);
      Range {
        low: str::parse::<i64>(arr[0]).unwrap(),
        high: str::parse::<i64>(arr[1]).unwrap(),
      }
    })
    .collect::<Vec<Range>>();
  s
}

fn digits(n: i64) -> u32 {
  n.checked_ilog10().unwrap_or(0) + 1
}

fn count_invalid(range: &Range) -> i64 {
  let mut counter = 0;
  for n in range.low..=range.high {
    let digits = digits(n);
    // println!("{digits}");
    if digits % 2 != 0 {
      continue;
    }
    let trunc_idx = 10i64.pow(digits / 2);
    let left = n / trunc_idx;
    let right = n % trunc_idx;
    if left == right {
      counter += n;
    }
    // println!("left: {left} | right: {right}");
  }
  counter
}

fn main() {
  let ranges = parse(&args().nth(1).unwrap());
  // let counter = count_invalid(&ranges[0]);
  let counter = ranges.iter().map(|range| count_invalid(range)).reduce(|a,b| a + b).unwrap_or(0);
  println!("{counter}");
  // println!("{:?}", ranges);
}
