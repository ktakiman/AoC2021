use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn check(x: usize, y: usize, w: usize, h: usize, map: &Vec<Vec<u32>>, checked: &mut Vec<Vec<bool>>) -> u32 {
    if checked[y][x] || map[y][x] == 9 {
        return 0;
    } 
    checked[y][x] = true;
    let mut ct = 1;

    if x > 0 {
        ct += check(x - 1, y, w, h, map, checked);
    }
    if x < (w - 1) {
        ct += check(x + 1, y, w, h, map, checked);
    }
    if y > 0 {
        ct += check(x, y - 1, w, h, map, checked);
    }
    if y < (h - 1) {
        ct += check(x, y + 1, w, h, map, checked);
    }

    return ct;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day9-sample.txt"
    } else {
        "./data/day9-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let map: Vec<Vec<u32>> = io::BufReader::new(&file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let w = map[0].len();
    let h = map.len();

    let mut part1 = 0;

    for y in 0..h {
        for x in 0..w {
            let n = map[y][x];
            if (x > 0 && map[y][x - 1] <= n)
                || (x < w - 1 && map[y][x + 1] <= n)
                || (y > 0 && map[y - 1][x] <= n)
                || (y < h - 1 && map[y + 1][x] <= n)
            {
                continue;
            }

            part1 += n + 1;
        }
    }

    println!("part1: {:?}", part1);

    let mut basins = Vec::<u32>::new();
    let mut checked: Vec<Vec<bool>> = (0..h).map(|y| vec![false; w]).collect();

    for y in 0..h {
        for x in 0..w {
            let basin = check(x, y, w, h, &map, &mut checked);
            if basin > 0 {
                basins.push(basin);
            }
        }
    }

    basins.sort();
    basins.reverse();

    println!("{:?}", basins);

    println!("part2: {:?}", basins[0] * basins[1] * basins[2]);

}
