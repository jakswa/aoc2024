type Topo = Vec<Vec<u8>>;
type Pos = (usize, usize);

fn main() {
    let input = aoc2024::input("10");
    //let input = SAMPLE;
    let map: Topo = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| i.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    let heads: Vec<Pos> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_x, height)| **height == 0)
                .map(|(x, _)| (x, y))
                .collect::<Vec<Pos>>()
        })
        .flatten()
        .collect();

    let nines: Vec<Pos> = vec![];
    let sum: u64 = heads
        .iter()
        .map(|pos| dig(&map, &pos, &mut nines.clone()))
        .sum();
    let trails_sum: u64 = heads.iter().map(|pos| dig_trails(&map, &pos)).sum();
    println!("part1: {sum}");
    println!("part2: {trails_sum}");
}

fn dig(map: &Topo, pos: &Pos, nines: &mut Vec<Pos>) -> u64 {
    if height(map, pos) == 9 && !nines.contains(pos) {
        nines.push(pos.clone());
        return 1;
    }
    adj(map, pos)
        .iter()
        .map(|new_pos| dig(map, new_pos, nines))
        .sum()
}

fn dig_trails(map: &Topo, pos: &Pos) -> u64 {
    if height(map, pos) == 9 {
        return 1;
    }
    adj(map, pos)
        .iter()
        .map(|new_pos| dig_trails(map, new_pos))
        .sum()
}

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
fn adj(map: &Topo, pos: &Pos) -> Vec<Pos> {
    let from_height = height(map, pos);
    DIRS.iter()
        .filter_map(|dir| {
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if new_pos.0 >= 0
                && new_pos.0 < map[0].len() as i32
                && new_pos.1 >= 0
                && new_pos.1 < map.len() as i32
            {
                let npos = (new_pos.0 as usize, new_pos.1 as usize);
                let new_height = height(map, &npos);
                if new_height > from_height && new_height - from_height == 1 {
                    return Some(npos);
                }
            }
            None
        })
        .collect::<Vec<Pos>>()
}

fn height(map: &Topo, pos: &Pos) -> u8 {
    map[pos.1][pos.0]
}

const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
