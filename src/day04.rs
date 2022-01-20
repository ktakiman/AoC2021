use std::env;
use std::fs::File;
use std::io::{self, BufRead};

// let's try using struct
#[derive(Debug)]
struct Item {
    num: u32,
    checked: bool,
}

#[derive(Debug)]
struct Bingo {
    size: usize,
    won: bool,
    items: Vec<Item>,
}

impl Bingo {
    fn mark(&mut self, num: u32) -> bool {
        if let Some(idx) = self.items.iter().position(|item| item.num == num) {
            self.items[idx].checked = true;
            // println!("{} is at {:?}", num, idx);

            // check horizontal
            let r: usize = idx / self.size;
            let c: usize = idx % self.size;

            let horz = (0..self.size).all(|i| self.items[r * self.size + i].checked);
            let vert = (0..self.size).all(|i| self.items[i * self.size + c].checked);

            // println!("h = {:?}, v = {:?}", horz, vert);

            return horz || vert;
        }

        false
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for item in &self.items {
            if !item.checked {
                score = score + item.num;
            }
        }
        return score;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day04-sample.txt"
    } else {
        "./data/day04-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let nums: Vec<u32> = lines[0]
        .split(',')
        .map(|tkn| tkn.parse::<u32>().unwrap())
        .collect();

    // println!("{:?}", nums);

    let line_ct = lines.len();
    let mut line_idx = 2;

    let bingo_size = 5;
    let mut boards = Vec::new();

    while line_idx + bingo_size <= line_ct {
        let mut i = 0;
        let mut items = Vec::new();

        while i < bingo_size {
            let row: Vec<Item> = lines[line_idx + i]
                .split_whitespace()
                .map(|tkn| Item {
                    num: tkn.parse::<u32>().unwrap(),
                    checked: false,
                })
                .collect();

            items.extend(row);
            i = i + 1;
        }

        boards.push(Bingo {
            size: bingo_size.try_into().unwrap(),
            won: false,
            items,
        });

        line_idx = line_idx + bingo_size + 1; // +1 for blank line between board rows
    }
    let boards_len = boards.len();

    let mut finished = 0;
    for num in nums {
        for board in &mut boards {
            if !board.won && board.mark(num) {
                board.won = true;
                if finished == 0 {
                    println!("part 1: score: {}", board.score() * num);
                }
                finished = finished + 1;
                if finished == boards_len {
                    println!("part 2: score: {}", board.score() * num);
                    break;
                }
            }
        }
    }

    // println!("{:?}", boards);
}
