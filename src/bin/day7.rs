fn main() {
    //let input = SAMPLE;
    let input = aoc2024::input("7");
    let part1: i64 = input
        .lines()
        .filter_map(|line| {
            let mut lr = line.split(": ");
            let l = lr.next().unwrap().parse::<i64>().unwrap();
            let mut r = lr
                .next()
                .unwrap()
                .split(" ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            r.reverse();
            if dig(r.pop().unwrap(), r, l) {
                return Some(l);
            } else {
                return None;
            }
        })
        .sum();

    let part2: i64 = input
        .lines()
        .filter_map(|line| {
            let mut lr = line.split(": ");
            let l = lr.next().unwrap().parse::<i64>().unwrap();
            let mut r = lr
                .next()
                .unwrap()
                .split(" ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            r.reverse();
            if dig2(r.pop().unwrap(), r, l) {
                return Some(l);
            } else {
                return None;
            }
        })
        .sum();

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn dig(a: i64, mut rest: Vec<i64>, target: i64) -> bool {
    if rest.len() == 0 {
        return a == target;
    }
    let b = rest.pop().unwrap();
    dig(a * b, rest.clone(), target) || dig(a + b, rest, target)
}

fn dig2(a: i64, mut rest: Vec<i64>, target: i64) -> bool {
    if rest.len() == 0 {
        return a == target;
    }
    let b = rest.pop().unwrap();
    let conc = format!("{}{}", a.to_string(), b.to_string())
        .parse::<i64>()
        .unwrap();
    dig2(a * b, rest.clone(), target)
        || dig2(a + b, rest.clone(), target)
        || dig2(conc, rest, target)
}

const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
