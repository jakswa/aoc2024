fn main() {
    //let input = SAMPLE;
    let input = aoc2024::input("6");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("part1: {}", get_count(grid.clone()));

    let mut block_spots = 0;
    (0..grid.len()).for_each(|aa| {
        (0..grid[aa].len()).for_each(|bb| {
            if grid[aa][bb] != '.' {
                return;
            }
            let mut newgrid = grid.clone();
            newgrid[aa][bb] = '#';
            if get_count(newgrid) == -1 {
                block_spots += 1;
            }
        });
    });
    println!("part2: {}", block_spots);
}

fn get_count(mut grid: Vec<Vec<char>>) -> isize {
    let a = (0..grid.len())
        .find(|ind| grid[*ind].contains(&'^'))
        .unwrap() as isize;
    let b = (0..grid[a as usize].len())
        .find(|ind| grid[a as usize][*ind] == '^')
        .unwrap() as isize;
    let mut pos = (a, b);
    let mut cnt = 1;
    let mut last_fresh = 0;
    loop {
        if last_fresh > 10_000 {
            return -1;
        }
        let dir = grid[pos.0 as usize][pos.1 as usize];
        let mov = match dir {
            '^' => (pos.0 - 1, pos.1),
            '>' => (pos.0, pos.1 + 1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            _ => panic!("what? {:?}", dir),
        };
        if mov.0 < 0 || mov.0 as usize >= grid.len() || mov.1 < 0 || mov.1 as usize >= grid[0].len()
        {
            break;
        }
        let nxt = grid[mov.0 as usize][mov.1 as usize];
        if nxt != '#' {
            grid[mov.0 as usize][mov.1 as usize] = dir;
            if nxt == '.' {
                cnt = cnt + 1;
            } else {
                last_fresh = last_fresh + 1;
            }
            pos = mov;
        } else {
            grid[pos.0 as usize][pos.1 as usize] = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("nope"),
            }
        }
    }
    cnt
}

const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
