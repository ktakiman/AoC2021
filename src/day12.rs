use std::env;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    name: String,
    is_cap: bool,
    is_start: bool,
    is_end: bool,
    link: Vec<usize>,
}

fn create_node(name: &str) -> Node {
    Node {
        name: String::from(name),
        is_cap: name.chars().next().unwrap().is_uppercase(),
        link: Vec::<usize>::new(),
        is_start: name == "start",
        is_end: name == "end",
    }
}

fn add_if_new(name: &str, graph: &mut Vec<Node>) -> usize {
    let mut i: usize = 0;
    for node in graph.iter() {
        if node.name == name {
            return i;
        }
        i += 1;
    }

    graph.push(create_node(name));
    return graph.len() - 1;
}

fn link(i: usize, j: usize, graph: &mut Vec<Node>) {
    graph[i].link.push(j);
    graph[j].link.push(i);
}

// #[derive(Debug)]
// struct Walker {
//     visited: HashSet<usize>
// }

fn walk(visited: &mut HashSet<usize>, to: usize, graph: &Vec<Node>, allow_two_visits: bool) -> usize {
    let cur = &graph[to];
    if cur.is_end {
        return 1;
    }

    let mut completed = 0;
    visited.insert(to);

    for next in cur.link.iter() {
        if graph[*next].is_cap || !visited.contains(next) {
            completed += walk(&mut visited.clone(), *next, graph, allow_two_visits);
        } else if visited.contains(next) && allow_two_visits && !graph[*next].is_start {
            completed += walk(&mut visited.clone(), *next, graph, false);
        }
    }

    return completed;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let datafile = if args.len() > 1 {
        if args[1] == "1" {
            "./data/day12-sample-1.txt"
        } else if args[1] == "2" {
            "./data/day12-sample-2.txt"
        } else {
            "./data/day12-sample-3.txt"
        }
    } else {
        "./data/day12-data.txt"
    };
    println!("input file: {:?}", datafile);

    let file = File::open(datafile).expect("failed to open a file");

    let lines: Vec<Vec<String>> = io::BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap().split('-').map(|s| String::from(s)).collect())
        .collect();

    let mut graph: Vec<Node> = Vec::new();

    for line in &lines {
        let i = add_if_new(&line[0], &mut graph);
        let j = add_if_new(&line[1], &mut graph);

        link(i, j, &mut graph);
    }

    for node in &graph {
        println!("{:?}", node);
    }

    let start = graph.iter().position(|node| node.is_start).unwrap();
    let part_one = walk(&mut HashSet::<usize>::new(), start, &graph, false);
    let part_two = walk(&mut HashSet::<usize>::new(), start, &graph, true);

    println!("part_one: {:?}", part_one);
    println!("part_two: {:?}", part_two);
}
