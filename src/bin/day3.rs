fn main() {
    let input = aoc2024::input("3");
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let part1: i64 = re
        .captures_iter(&input)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum();

    let part2: i64 = input
        .split("do()")
        .map(|spl| {
            let do_it = spl.split("don't").next().unwrap();

            re.captures_iter(&do_it)
                .map(|cap| {
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                        * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
                })
                .sum::<i64>()
        })
        .sum();
    println!("part1: {part1}");
    println!("part2: {part2}");
}
