use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day11-sample.txt"
    } else {
        "./data/day11-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let mut lines: Vec<Vec<u32>> = io::BufReader::new(&file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let w = lines[0].len();
    let h = lines.len();

    let mut i = 0;
    let mut flash_ct = 0;
    let mut part_one = 0;

    while true {
    // for i in 0..100 {
        for line in &lines {
            println!("{:?}", line);
        }
        println!("");

        for y in 0..h {
            for x in 0..w {
                lines[y][x] += 1;
            }
        }

        let mut any_flashed = true;
        let mut flashed: Vec<Vec<bool>> = (0..h).map(|_| vec![false; w]).collect();

        while any_flashed {
            any_flashed = false;
            for y in 0..h {
                for x in 0..w {
                    if lines[y][x] > 9 && !flashed[y][x] {
                        any_flashed = true;
                        flash_ct += 1;
                        flashed[y][x] = true;

                        if x > 0 {
                            lines[y][x - 1] += 1;
                            if y > 0 {
                                lines[y - 1][x - 1] += 1;
                            }
                            if y < h - 1 {
                                lines[y + 1][x - 1] += 1;
                            }
                        }
                        if y > 0 {
                            lines[y - 1][x] += 1;
                        }
                        if y < h - 1 {
                            lines[y + 1][x] += 1;
                        }
                        if x < w - 1 {
                            lines[y][x + 1] += 1;
                            if y > 0 {
                                lines[y - 1][x + 1] += 1;
                            }
                            if y < h - 1 {
                                lines[y + 1][x + 1] += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut did_not_flash = false;

        for y in 0..h {
            for x in 0..w {
                if lines[y][x] > 9 {
                    lines[y][x] = 0;
                } else {
                    did_not_flash = true;
                }
            }
        }

        if i == 99 {
            part_one = flash_ct;
        }

        i += 1;

        if !did_not_flash {
            break;
        }
        // for line in &lines {
        //     println!("{:?}", line);
        // }
        // println!("");
    }

    println!("part1: {}", part_one);
    println!("part2: {}", i);
}
