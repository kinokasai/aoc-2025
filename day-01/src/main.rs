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
      Some('L') => Command::Sub(value % 100),
      Some('R') => Command::Add(value % 100),
      _ => panic!("this should not happen")
    }
    }).collect::<Vec<Command>>()
}

fn execute(commands: Vec<Command>) -> i32 {
  let mut counter = 0;
  commands.into_iter().fold(50, |acc, command| {
    let acc = match command {
      Command::Add(i) => (acc + i) % 100,
      Command::Sub(i) => if acc - i < 0 { acc + 100 - i } else { acc - i }
    };
    println!("{:?} -> {acc}!", command);
    if acc == 0 || acc == 100 { counter += 1; };
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
