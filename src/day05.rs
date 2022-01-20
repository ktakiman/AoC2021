use std::env;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Path {
    from: Point,
    to: Point,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        "./data/day05-sample.txt"
    } else {
        "./data/day05-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    fn to_point(token: &str) -> Point {
        let nums: Vec<usize> = token.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
        Point {
            x: nums[0],
            y: nums[1],
        }
    }

    let mut paths = Vec::new();

    for line in &lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let path = Path {
            from: to_point(&tokens[0]),
            to: to_point(&tokens[2]),
        };
        paths.push(path);
    }

    let max_x = paths.iter().map(|path| cmp::max(path.from.x, path.to.x)).max().unwrap();
    let max_y = paths.iter().map(|path| cmp::max(path.from.y, path.to.y)).max().unwrap();

    let max: usize = cmp::max(max_x, max_y);

    fn play(grid: &mut Vec<usize>, paths: &Vec<Path>, max: usize, diagonal: bool) -> usize {

        fn inc(grid: &mut Vec<usize>, max: usize, x: usize, y: usize) {
            grid[x + y * max] = grid[x + y * max] + 1;
        }

        for path in paths {
            let dx = path.to.x.cmp(&path.from.x);
            let dy = path.to.y.cmp(&path.from.y);

            let mut x = path.from.x;
            let mut y = path.from.y;

            if !diagonal && dx != cmp::Ordering::Equal && dy != cmp::Ordering::Equal {
                continue;
            }

            inc(grid, max, x, y);

            fn shift(delta: cmp::Ordering, n: &mut usize) {
                match delta {
                    cmp::Ordering::Less => *n = *n - 1,
                    cmp::Ordering::Greater => *n = *n + 1,
                    _ => (),
                };
            }

            while x != path.to.x || y != path.to.y {
                if x != path.to.x {
                    shift(dx, &mut x);
                }
                if y != path.to.y {
                    shift(dy, &mut y);
                }

                inc(grid, max, x, y);
            }
        }

        // println!("{:?}", grid);

        grid.iter().filter(|i| i > &&1usize).count()
    }

    let mut grid = vec![0; (max + 1) * (max + 1)];
    let part_one = play(&mut grid, &paths, max, false);
    println!("part_one: {}", part_one);

    grid = vec![0; (max + 1) * (max + 1)];
    let part_two = play(&mut grid, &paths, max, true);
    println!("part_two: {}", part_two);

}
