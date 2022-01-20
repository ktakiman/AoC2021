use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./data/day03-data.txt").expect("failed to open a file");

    let mut lines: Vec<String> = io::BufReader::new(&file).lines().map(|line| line.unwrap()).collect();

    let line_chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // let lines: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
    let num_lines = lines.len();

    let len = line_chars[0].len();

    let mut cts = vec![0; len];

    let mut pos: usize;

    for line_char in line_chars {
        pos = 0;
        loop {
            if line_char[pos] == '1' {
                cts[pos] = cts[pos] + 1;
            }

            pos = pos + 1;

            if pos == len {
                break;
            }
        }
    }

    pos = 0;
    let mut gam = String::new();
    let mut eps = String::new();

    println!("{:?}", cts);

    loop {
        if cts[pos] < num_lines / 2 {
            gam.push('0');
            eps.push('1');
        } else {
            gam.push('1');
            eps.push('0');
        }

        pos = pos + 1;
        if pos == len {
            break;
        }
    }

    // part one
    let n_gam = u32::from_str_radix(&gam, 2).unwrap();
    let n_eps = u32::from_str_radix(&eps, 2).unwrap();

    println!("part one: {:?}, {:?}", gam, eps);
    println!("part one: {:?}, {:?}, {}", n_gam, n_eps, n_gam * n_eps);

    // part two
    
    fn eliminate(lines: &mut Vec<String>, pos: usize, apply_high: bool) {
        println!("{:?}", pos);

        let ct = lines.len();
        let doubled_ones = lines.iter().filter(|&line| &line[pos..pos + 1] == "1").count() * 2;

        let to_select: &str;

        if doubled_ones < ct {
            to_select = if apply_high { "0" } else { "1" };
        } else {
            to_select = if apply_high { "1" } else { "0" };
        }

        lines.retain(|line| &line[pos..pos + 1] == to_select);

        println!("{}", lines.len());
    }

    let mut pos: usize = 0;
    let ogr: &str;
    let mut scr: &str = "";

    let mut lines_copy: Vec<String> = lines.clone();

    loop {
        eliminate(&mut lines_copy, pos, true);

        if lines_copy.len() == 1 {
            ogr = &lines_copy[0];
            break;
        }

        pos = pos + 1;
    }

    println!("{}", ogr);

    pos = 0;

    loop {
        eliminate(&mut lines, pos, false);

        if lines.len() == 1 {
            scr = &lines[0];
            break;
        }

        if lines.len() == 0 {
            break;
        }

        pos = pos + 1;
    }

    // println!("{}", ogr);

    // let mut ogr = String::new();
    // let mut scr = String::new();

    // pos = len;

    // loop {
    //     if ogr.len() > 0 && scr.len() > 0 {
    //         break;
    //     }

    //     if ogr.len() == 0 {
    //         let gam = &gam[0..pos];
    //         println!("try ogr = {:?}", gam);
    //         let found = lines.iter().find(|&line| &line[0..pos] == gam);

    //         if found.is_some() {
    //             ogr = found.unwrap().to_string();
    //             println!("ogr ~ {:?}", found);
    //         }
    //     }

    //     if scr.len() == 0 {
    //         let eps = &eps[0..pos];
    //         println!("try eps = {:?}", eps);
    //         let found = lines.iter().find(|&line| &line[0..pos] == eps);

    //         if found.is_some() {
    //             scr = found.unwrap().to_string();
    //             println!("scr ~ {:?}", found);
    //         }
    //     }
    //     pos = pos - 1;
    // }

    let n_ogr = u32::from_str_radix(&ogr, 2).unwrap();
    let n_scr = u32::from_str_radix(&scr, 2).unwrap();

    println!("part two: {:?}, {:?}", ogr, scr);
    println!("part two: {:?}, {:?}, {}", n_ogr, n_scr, n_ogr * n_scr);

}
