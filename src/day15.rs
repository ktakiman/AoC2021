use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn solve(board: &mut Vec<Vec<(usize, usize)>>) -> usize {
    let h = board.len();
    let w = board[0].len();

    board[h - 1][w - 1].1 = board[h - 1][w - 1].0;
    loop {
        let mut modified = false;
        for y in 0..h {
            let yy = h - y - 1;
            for x in 0..w {
                if y == 0 && x == 0 {
                    continue;
                }
                let xx = w - x - 1;
                let mut min = usize::MAX;
                if xx < w - 1 {
                    min = cmp::min(board[yy][xx + 1].1, min);
                }
                if xx > 0 {
                    min = cmp::min(board[yy][xx - 1].1, min);
                }
                if yy < h - 1 {
                    min = cmp::min(board[yy + 1][xx].1, min);
                }
                if yy > 0 {
                    min = cmp::min(board[yy - 1][xx].1, min);
                }

                let at = &mut board[yy][xx];
                if min + at.0 < at.1 {
                    modified = true;
                    at.1 = min + at.0;
                }
            }
        }
        if !modified {
            break;
        }
    }

    &board[0][0].1 - 1
}

fn main() {
    let datafile = match env::args().skip(1).next() {
        Some(_) => "./data/day15-sample.txt",
        None => "./data/day15-data.txt",
    };

    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let mut board: Vec<Vec<(usize, usize)>> = io::BufReader::new(&file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| {
                    (
                        usize::try_from(c.to_digit(10).unwrap()).unwrap(),
                        usize::MAX,
                    )
                })
                .collect()
        })
        .collect();

    let h = board.len();
    let w = board[0].len();

    let mut big_board: Vec<Vec<(usize, usize)>> = board.iter().map(|row| row.clone()).collect();
    for y in 0..5 {
        if y > 0 {
            big_board.extend((0..h).map(|_| Vec::<(usize, usize)>::new()));
        }
        for x in 0..5 {
            if x == 0 && y == 0 {
                continue;
            }
            let ref_x = if x > 0 { w } else { 0 };
            let ref_y = if x > 0 { 0 } else { h };

            for yy in 0..h {
                for xx in 0..w {
                    let val = cmp::max(1, (big_board[y * h - ref_y + yy][x * w - ref_x + xx].0 + 1) % 10);
                    big_board[y * h + yy].push((val, usize::MAX));
                }
            }
        }
    }

    for row in &big_board {
        println!("{:?}", row.iter().map(|pair| pair.0).collect::<Vec<usize>>());
    }

    let part_one = solve(&mut board);
    let part_two = solve(&mut big_board);

    println!("part_one: {:?}", part_one);
    println!("part_two: {:?}", part_two);
}
