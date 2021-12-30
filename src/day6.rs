use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day6-sample.txt"
    } else {
        "./data/day6-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");
    let contents = io::BufReader::new(&file).lines().next().unwrap().unwrap();

    let mut nums = [0; 9];

    for i in contents.split(',').map(|tkn| tkn.parse::<usize>().unwrap()) {
        nums[i] += 1;
    }

    let mut day = 0;

    while day < 256 {
        // println!("{:?}", nums);
        let zero = nums[0];

        for i in 0..6 {
            nums[i] = nums[i + 1];
        }
        nums[6] = zero + nums[7];
        nums[7] = nums[8];
        nums[8] = zero;

        day = day + 1;

        if day == 80 {
            println!("part one: {}", nums.iter().sum::<usize>());
        }
    }

    println!("part two: {}", nums.iter().sum::<usize>());
}
