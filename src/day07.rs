use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day07-sample.txt"
    } else {
        "./data/day07-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");
    let contents = io::BufReader::new(&file).lines().next().unwrap().unwrap();

    let nums: Vec<i32> = contents.split(',').map(|tkn| tkn.parse::<i32>().unwrap()).collect();

    let min: i32 = *nums.iter().min().unwrap();
    let max: i32 = *nums.iter().max().unwrap();

    println!("{:?}, {:?}", min, max);

    let fuel: i32 = (min..=max).map(|tgt| nums.iter().map(|n| (n - tgt).abs()).sum()).min().unwrap();
    println!("part one: {:?}", fuel);

    fn comp_fuel(d: i32) -> i32 {
        if d == 0 {
            return 0;
        }
        let mut sum = 0;
        for i in 1..=d {
            sum += i;
        }

        return sum;
    }

    let fuel: i32 = (min..=max).map(|tgt| nums.iter().map(|n| comp_fuel((n - tgt).abs())).sum()).min().unwrap();
    println!("part two: {:?}", fuel);
}
