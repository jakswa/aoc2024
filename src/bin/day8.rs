use gcd::Gcd;
use std::collections::{HashMap, HashSet};

fn main() {
    //let input = SAMPLE;
    let input = aoc2024::input("8");
    let mut ants: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let bounds = (
        input.lines().next().unwrap().chars().count() as isize,
        input.lines().count() as isize,
    );

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            ants.entry(c).or_insert(Vec::new()).push((x, y));
        });
    });

    let mut nodes_p1: HashSet<(isize, isize)> = HashSet::new();
    let mut nodes_p2: HashSet<(isize, isize)> = HashSet::new();
    ants.values().for_each(|antlist| {
        antlist.iter().for_each(|pos1| {
            antlist.iter().for_each(|pos2| {
                if pos1 == pos2 {
                    return;
                }
                nodes_p2.insert((pos1.0 as isize, pos1.1 as isize));
                let dist = (
                    (pos2.0 as isize - pos1.0 as isize),
                    (pos2.1 as isize - pos1.1 as isize),
                );
                let node = (pos2.0 as isize + dist.0, pos2.1 as isize + dist.1);
                if node.0 >= 0 && node.0 < bounds.0 && node.1 >= 0 && node.1 < bounds.1 {
                    nodes_p1.insert(node);
                }

                let gcd = (dist.0.abs() as usize).gcd(dist.1.abs() as usize);
                let line_step = (dist.0 / gcd as isize, dist.1 / gcd as isize);
                let mut node2 = (pos2.0 as isize + line_step.0, pos2.1 as isize + line_step.1);
                loop {
                    if node2.0 >= 0 && node2.0 < bounds.0 && node2.1 >= 0 && node2.1 < bounds.1 {
                        nodes_p2.insert(node2);
                        node2.0 += line_step.0;
                        node2.1 += line_step.1;
                    } else {
                        break;
                    }
                }
            });
        });
    });
    println!("part1: {}", nodes_p1.len());
    println!("part2: {}", nodes_p2.len());
}

const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
