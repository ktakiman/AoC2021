use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day13-sample.txt"
    } else {
        "./data/day13-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut dots: Vec<Vec<i32>> = lines
        .iter()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            line.split(',')
                .map(|tkn| tkn.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let folds: Vec<(char, i32)> = lines
        .iter()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .map(|line| {
            let mut itr = line.split(' ').skip(2).next().unwrap().split('=');
            (
                itr.next().unwrap().chars().next().unwrap(),
                itr.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    // for dot in &dots {
    //     println!("{:?}", dot);
    // }

    // for fold in &folds {
    //     println!("{:?}", fold);
    // }

    //for fold in folds.iter().take(1) {   << use this line to get the answer for part one
    for fold in folds.iter() {
        if fold.0 == 'y' {
            for dot in &mut dots {
                if dot[1] > fold.1 {
                    dot[1] = fold.1 - (dot[1] - fold.1);
                }
            }
        } else {
            for dot in &mut dots {
                if dot[0] > fold.1 {
                    dot[0] = fold.1 - (dot[0] - fold.1);
                }
            }
        }
    }

    // println!();

    // for dot in &dots {
    //     println!("{:?}", dot);
    // }

    dots.sort();
    dots.dedup();

    for dot in &dots {
        println!("{:?}", dot);
    }

    // println!("part_one: {}", dots.len());

    let x_max = dots.iter().map(|dot| dot[0]).max().unwrap();
    let y_max = dots.iter().map(|dot| dot[1]).max().unwrap();

    let map = dots
        .iter()
        .fold(HashSet::<(i32, i32)>::new(), |mut acc, dot| {
            acc.insert((dot[0], dot[1]));
            acc
        });

    for y in 0..=y_max {
        for x in 0..=x_max {
            print!("{}", if map.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }

    // println!();


    // println!("part_one: {}", dots.len());
}
