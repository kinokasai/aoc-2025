use std::{env::args, fs};

#[derive(Debug)]
enum Command {
  Add(i32),
  Sub(i32),
}

fn parse(filename: &str) -> Vec<Command> {
  let s = fs::read_to_string(filename).unwrap();
  let s = s.split("\n").filter(|s| s.len() > 0).collect::<Vec<&str>>();
  s.into_iter().map(|s| {
    let idx = s.chars().nth(0);
    let value = s.chars().skip(1).collect::<String>();
    let value = str::parse::<i32>(&value).unwrap();
    match idx {
      Some('L') => Command::Sub(value),
      Some('R') => Command::Add(value),
      _ => panic!("this should not happen")
    }
    }).collect::<Vec<Command>>()
}

fn execute(commands: Vec<Command>) -> i32 {
  let mut counter = 0;
  commands.into_iter().fold(50, |acc, command| {
    let acc = match command {
      Command::Add(i) => {
        let total = acc + i;
        let rem = total % 100;
        let quotient = total / 100;
        counter += quotient;
        rem
      },
      Command::Sub(i) => {
        // We don't want to count 0 -> [xx] as another passage through 0.
        if acc == 0 {counter -= 1;}
        let total = (100 - acc) + i;
        let quotient = total / 100;
        counter += quotient;
        let rem = (100 - (total % 100)) % 100;
        rem
      }
    };
    println!("{:?} -> {acc} | {counter}", command);
    acc
  });
  counter
}

fn main() {
  let filename = args().nth(1).unwrap();
  let commands = parse(&filename);
  let clicks = execute(commands);
  println!("clicked {clicks}!")
}
