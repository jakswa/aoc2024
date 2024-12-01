fn main() {
    let input = aoc2024::input("1");
    let mut inps: Vec<Vec<i32>> = vec![vec![], vec![]];
    input.lines().for_each(|line| {
        let mut ll: Vec<i32> = line
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        inps[1].push(ll.pop().unwrap());
        inps[0].push(ll.pop().unwrap());
    });

    inps[0].sort();
    inps[1].sort();

    part1(&inps);
    part2(&inps);
}

fn part2(inps: &Vec<Vec<i32>>) {
    let res: i32 = (0..inps[0].len())
        .map(|ind| {
            let num = inps[0][ind];
            let cnt = inps[1].iter().filter(|i| **i == num).count();
            num * cnt as i32
        })
        .sum();
    println!("wee: {res}");
}

fn part1(inps: &Vec<Vec<i32>>) {
    let res: i32 = (0..inps[0].len())
        .map(|ind| (inps[0][ind] - inps[1][ind]).abs())
        .sum();
    println!("wee: {res}");
}
