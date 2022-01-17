use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn perm(to_fill: &Vec<usize>, to_select: &Vec<usize>, list: &mut Vec<Vec<usize>>) {
    if to_select.len() == 1 {
        let mut clone = to_fill.clone();
        clone.push(to_select[0]);
        list.push(clone);
    } else {
        for i in 0..to_select.len() {
            let mut clone_fill = to_fill.clone();
            let mut clone_select = to_select.clone();
            clone_fill.push(clone_select[i]);
            clone_select.remove(i);
            perm(&clone_fill, &clone_select, list);
        }
    }
}

fn generate_signals(perm: &Vec<usize>) -> Vec<String> {
    let chars: Vec<char> = "abcdefg".chars().collect();

    let patterns: [Vec<usize>; 10] = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    patterns
        .iter()
        .map(|pat| {
            let mut sig: Vec<char> = pat.iter().map(|i| chars[perm[*i]]).collect();
            sig.sort();
            String::from_iter(sig.iter())
        })
        .collect()

    // String::from_iter(pat.iter().map(|i| chars[perm[*i]]))).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day8-sample.txt"
    } else {
        "./data/day8-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let to_fill = Vec::new();
    let to_use = vec![0, 1, 2, 3, 4, 5, 6];
    let mut perms: Vec<Vec<usize>> = Vec::new();

    // generate all permutations of 0 to 7
    // these list all the possible ways of assigning 'a' to 'g' to at certain positions
    perm(&to_fill, &to_use, &mut perms);

    let all_possible_signals: Vec<Vec<String>> =
        perms.iter().map(|perm| generate_signals(perm)).collect();

    let mut part_one: usize = 0;
    let mut part_two: usize = 0;

    for line in &lines {
        let mut secs = line.split('|').map(|sec| {
            sec.split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| {
                    let mut cs: Vec<char> = s.chars().collect();
                    cs.sort();
                    String::from_iter(cs.iter())
                })
                .collect::<Vec<String>>()
        });

        let sec_one = secs.next().unwrap();
        let sec_two = secs.next().unwrap();

        fn matches(one: &Vec<String>, two: &Vec<String>) -> bool {
            for s in one {
                if !two.contains(&s) {
                    return false;
                }
            }

            true
        }

        let matched = all_possible_signals
            .iter()
            .find(|signal| matches(&signal, &sec_one))
            .unwrap();

        let sec_one_trans: Vec<usize> = sec_one
            .iter()
            .map(|s| matched.iter().position(|m| m == s).unwrap())
            .collect();
        let sec_two_trans: Vec<usize> = sec_two
            .iter()
            .map(|s| matched.iter().position(|m| m == s).unwrap())
            .collect();

        println!("{:?}: {:?}", sec_one_trans, sec_two_trans);

        part_one += sec_two_trans
            .iter()
            .filter(|i| [1, 4, 7, 8].contains(i))
            .count();

        let solved: usize = sec_two_trans
            .iter()
            .enumerate()
            .map(|(idx, n)| 10usize.pow(u32::try_from(3 - idx).unwrap()) * n)
            .sum();
        part_two += solved;

        // let ct = sec_two
        //     .iter()
        //     .map(|s| s.len())
        //     .filter(|l| l == &2 || l == &3 || l == &4 || l == &7)
        //     .count();

        // part_one += ct;
        //
        // break;
    }

    //let mut perms = new Vec<usize> perms;

    // perm(

    println!("{:?}", part_one);
    println!("{:?}", part_two);
}
