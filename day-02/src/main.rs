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
      // println!("{:?}", arr);
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
  // println!("range: {:?}", range);
  let mut counter = 0;
  for n in range.low..=range.high {
    let digits = digits(n) as usize;
    // println!("processing: {n} | digits: {digits}");
    let str = n.to_string().chars().collect::<Vec<char>>();
    for i in (1..=digits/2).rev() {
      let chunks = str
        .chunks(i)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();
      // println!("chunks: {:?}", &chunks);
      let goal = &chunks[0];
      let equal = chunks.iter().fold(true, |acc, e| acc && e == goal);
      if equal {
        // println!("bad id: {n}");
        counter += n;
        break;
      }
    }
  }
  counter
}

fn main() {
  let ranges = parse(&args().nth(1).unwrap());
  // let counter = count_invalid(&ranges[2]);
  let counter = ranges
    .iter()
    .map(|range| count_invalid(range))
    .reduce(|a, b| a + b)
    .unwrap_or(0);
  println!("counter: {counter}");
  // println!("{:?}", ranges);
}
