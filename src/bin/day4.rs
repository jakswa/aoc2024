const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const DIRS: [[isize; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
];

type Grid = Vec<Vec<char>>;

fn main() {
    let input = aoc2024::input("4");
    //let input = SAMPLE.to_string();
    let grid: Grid = input
        .to_string()
        .lines()
        .map(|i| i.chars().collect())
        .collect();
    let part1: isize = (0 as isize..grid[0].len() as isize)
        .map(|x| {
            (0 as isize..grid.len() as isize)
                .map(|y| count_at(&grid, (x, y)))
                .sum::<isize>()
        })
        .sum();
    let part2: isize = (0 as isize..grid[0].len() as isize)
        .map(|x| {
            (0 as isize..grid.len() as isize)
                .map(|y| part2_at(&grid, (x, y)))
                .sum::<isize>()
        })
        .sum();
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn char_at(grid: &Grid, coords: (isize, isize)) -> char {
    if coords.1 < 0 || coords.1 >= grid.len() as isize {
        return 'z';
    }
    if coords.0 < 0 || coords.0 >= grid[0].len() as isize {
        return 'z';
    }
    grid[coords.1 as usize][coords.0 as usize]
}

fn part2_at(grid: &Grid, xy: (isize, isize)) -> isize {
    if char_at(grid, xy) != 'A' {
        return 0;
    }
    let a = (
        char_at(grid, (xy.0 - 1, xy.1 - 1)),
        char_at(grid, (xy.0 + 1, xy.1 + 1)),
    );
    if !(a == ('M', 'S') || a == ('S', 'M')) {
        return 0;
    }
    let b = (
        char_at(grid, (xy.0 + 1, xy.1 - 1)),
        char_at(grid, (xy.0 - 1, xy.1 + 1)),
    );
    if b == ('M', 'S') || b == ('S', 'M') {
        return 1;
    }
    0
}
fn count_at(grid: &Grid, xy: (isize, isize)) -> isize {
    if char_at(grid, xy) != 'X' {
        return 0;
    }
    DIRS.iter()
        .map(|dir| {
            let m = char_at(&grid, (xy.0 + dir[0], xy.1 + dir[1])) == 'M';
            let a = char_at(&grid, (xy.0 + (2 * dir[0]), xy.1 + (2 * dir[1]))) == 'A';
            let s = char_at(&grid, (xy.0 + (3 * dir[0]), xy.1 + (3 * dir[1]))) == 'S';
            if m && a && s {
                return 1;
            }
            0
        })
        .sum()
}
