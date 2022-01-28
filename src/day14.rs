use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashMap;

fn cycle(
    inserts: &HashMap<char, HashMap<char, char>>,
    left: char,
    right: char,
    remaining: u32,
    tally: &mut HashMap<char, u64>,
) {
    let insert = inserts.get(&left).unwrap().get(&right).unwrap();

    *tally.entry(*insert).or_insert(0) += 1;

    if remaining > 1 {
        cycle(inserts, left, *insert, remaining - 1, tally);
        cycle(inserts, *insert, right, remaining - 1, tally);
    }
}

fn part_one(polymer: &Vec<char>, inserts: &HashMap<char, HashMap<char, char>>) {
    let repeat = 10; // setting to 40 for part_two won't cut it, it takes too long

    let mut tally = HashMap::<char, u64>::new();

    for i in 0..polymer.len() {
        let left = polymer[i];

        *tally.entry(left).or_insert(0) += 1;

        if i < polymer.len() - 1 {
            let right = polymer[i + 1];
            cycle(&inserts, left, right, repeat, &mut tally);
        }
    }

    let min = tally.values().min().unwrap();
    let max = tally.values().max().unwrap();
    // let part_one = String::from_iter(polymer.iter());

    println!("part one: {:?}", max - min);
}

fn cycle2<'a>(
    left: char,
    right: char,
    repeat: i32,
    inserts: &HashMap<char, HashMap<char, char>>,
    count_map: &'a mut HashMap<(char, char, i32), HashMap<char, u64>>
) -> &'a HashMap<char, u64> 
{
    if !count_map.contains_key(&(left, right, repeat)) {
        let middle = inserts.get(&left).unwrap().get(&right).unwrap();

        let mut counts = HashMap::from([(*middle, 1)]);

        if repeat > 1 {
            let left_branch = cycle2(left, *middle, repeat - 1, inserts, count_map);
            for key in left_branch.keys() {
                *counts.entry(*key).or_insert(0) += left_branch.get(key).unwrap();
            }

            let right_branch = cycle2(*middle, right, repeat - 1, inserts, count_map);
            for key in right_branch.keys() {
                *counts.entry(*key).or_insert(0) += right_branch.get(key).unwrap();
            }
        }

        count_map.insert((left, right, repeat), counts);
    }

    return count_map.get(&(left, right, repeat)).unwrap();
}

fn part_two(polymer: &Vec<char>, inserts: &HashMap<char, HashMap<char, char>>) {
    // Seems crazy... not sure if this will give enough performance gain for 40 iterations...
    // (this worked!!!  even up to 60 iterations worked, higher than that cause the panic)
    // This maps left, right charater and depth to a map which tells me which character gets added
    // how many times.
    // The main idea is to avoid the counting of the cases which have already been came across.
    let mut count_map = HashMap::<(char, char, i32), HashMap<char, u64>>::new();

    let mut tally = HashMap::<char, u64>::new();
    
    for i in 0..polymer.len() {
        let left = polymer[i];

        *tally.entry(left).or_insert(0) += 1;

        if i < polymer.len() - 1 {
            let right = polymer[i + 1];
            let count = cycle2(left, right, 40, &inserts, &mut count_map);
            for key in count.keys() {
                *tally.entry(*key).or_insert(0) += count.get(key).unwrap();
            }
        }

        println!(".");
    }

    for key in count_map.keys() {
        println!("{:?} --> {:?}", key, count_map.get(key).unwrap());
    }
    

    let min = tally.values().min().unwrap();
    let max = tally.values().max().unwrap();
    // let part_one = String::from_iter(polymer.iter());

    println!("part two: {:?}", max - min);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day14-sample.txt"
    } else {
        "./data/day14-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let polymer: Vec<char> = lines[0].chars().collect();

    let inserts: HashMap<char, HashMap<char, char>> =
        lines.iter().skip_while(|line| line.len() > 0).skip(1).fold(
            HashMap::<char, HashMap<char, char>>::new(),
            |mut acc, line| {
                let mut itr = line.split(' ');
                let from: Vec<char> = itr.next().unwrap().chars().collect();
                let to = itr.skip(1).next().unwrap().chars().next().unwrap();

                match acc.get_mut(&from[0]) {
                    Some(val) => {
                        val.insert(from[1], to);
                    }
                    None => {
                        acc.insert(from[0], HashMap::from([(from[1], to)]));
                    }
                }
                acc
            },
        );

    // println!("{:?}", polymer);
    // println!();
    // println!("{:?}", inserts);

    part_one(&polymer, &inserts);

    part_two(&polymer, &inserts);
}
