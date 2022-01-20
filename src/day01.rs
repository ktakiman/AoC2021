use std::env;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn part_one(nums: Vec<i32>) -> i32 {
    let mut prev: i32 = std::i32::MAX;
    let mut ct: i32 = 0;

    for num in nums {
        if num > prev {
            ct = ct + 1;
        }
        prev = num;
    }
    ct
}

fn part_two(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut ct: i32 = 0;

    let mut i = 0;

    while i < len - 3 {
        let a = nums[i] + nums[i + 1] + nums[i + 2];
        let b = nums[i + 1] + nums[i + 2] + nums[i + 3]; 
        println!("{} < {} ?", a, b);
        if a < b {
            ct = ct + 1;
        }

        i = i + 1;
    }
    ct
}

fn main() {
    let path = env::current_dir();
    println!("{:?}", path);

    // open returns io::result
    let file = File::open("./data/day01-data.txt").expect("failed to open a file");

    let lines = io::BufReader::new(file).lines();

    let nums: Vec<i32> = lines
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let ct = part_two(nums);
    println!("final answer is {}", ct);
}
