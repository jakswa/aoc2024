fn main() {
    let input = aoc2024::input("2");
    //    let input = "7 6 4 2 1
    //1 2 7 8 9
    //9 7 6 2 1
    //1 3 2 4 5
    //8 6 4 4 1
    //1 3 6 7 9";
    let reps: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(" ").map(|i| i.parse::<i32>().unwrap()).collect())
        .collect();
    let part1 = reps.iter().filter(|rep| is_ok(rep)).count();
    println!("part1: {}", part1);

    let part2 = reps.iter().filter(|rep| permute_ok(rep)).count();
    println!("part2: {}", part2);
}

fn permute_ok(rep: &[i32]) -> bool {
    is_ok(rep)
        || (0..rep.len()).any(|ind| {
            let mut um: Vec<i32> = Vec::new();
            if ind > 0 {
                rep[0..ind].iter().for_each(|i| um.push(*i));
            }
            rep[(ind + 1)..].iter().for_each(|i| um.push(*i));
            is_ok(&um[..])
        })
}
fn is_ok(rep: &[i32]) -> bool {
    (rep.windows(2).all(|i| i[0] < i[1]) || rep.windows(2).all(|i| i[0] > i[1]))
        && rep
            .windows(2)
            .all(|i| (1..4).contains(&(i[0] - i[1]).abs()))
}
