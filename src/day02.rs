use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn part_one(lines: Vec::<(String, i32)>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in lines {
        let (mv, n) = line;

        if mv == "forward" {
            x = x + n;
        } else if mv == "up" {
            y = y - n;
        } else if mv == "down" {
            y = y + n;
        }
    }

    return x * y;
}

fn part_two(lines: Vec::<(String, i32)>) -> i32 {
    let mut aim: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in lines {
        let (mv, n) = line;

        if mv == "forward" {
            x = x + n;
            y = y + aim * n;
        } else if mv == "up" {
            aim = aim - n;
        } else if mv == "down" {
            aim = aim + n;
        }
    }

    return x * y;
}

fn main() {
    let path = env::current_dir();

    // open returns io::result
    let file = File::open("./data/day02-data.txt").expect("failed to open a file");

    let lines = io::BufReader::new(file).lines();

    let lines: Vec<(String, i32)> = lines.map(|line| {
        let s = line.unwrap();
        let tokens = s.split(' ').collect::<Vec<&str>>();
        // let s: String = String::from(tokens[0]);//.to_string();
        (tokens[0].to_string(), tokens[1].parse::<i32>().unwrap())
    }).collect();
        // .map(|line| line.unwrap()).collect();

    let ans = part_two(lines);
    println!("{}", ans);
}
