use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day10-sample.txt"
    } else {
        "./data/day10-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<Vec<char>> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let mut part_one = 0;
    let mut part_two = Vec::<u64>::new();

    for line in &lines {
        let mut stack = Vec::<char>::new();
        let mut incomplete = true;

        for c in line {
           let score = match c {
               '(' => { stack.push(')'); 0 },
               '[' => { stack.push(']'); 0 },
               '{' => { stack.push('}'); 0 },
               '<' => { stack.push('>'); 0 },
               ')' => if stack.pop() == Some(')') { 0 } else { 3 },
               ']' => if stack.pop() == Some(']') { 0 } else { 57 },
               '}' => if stack.pop() == Some('}') { 0 } else { 1197 },
               '>' => if stack.pop() == Some('>') { 0 } else { 25137 },
               _ => 0
            };

           if score > 0 {
               part_one += score;
               incomplete = false;
               break;
           }
        }

        if incomplete {
            // println!("{:?}", stack);
            let mut score: u64 = 0;
            for c in stack.iter().rev() {
                score = score * 5 + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                };
            }

            part_two.push(score);
        }
    }

    println!("part1: {}", part_one);

    part_two.sort();
    println!("part2: {}", part_two[part_two.len() / 2]);
    // println!("part2: {:?}", part_two);
}
