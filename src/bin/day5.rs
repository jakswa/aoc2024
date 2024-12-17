use std::collections::HashMap;

fn main() {
    let input = aoc2024::input("5");
    //let input = SAMPLE;
    let mut comps: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut input_iter = input.split("\n\n");
    input_iter.next().unwrap().lines().for_each(|line| {
        let nums = line
            .split("|")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        comps.entry(nums[0]).or_insert(Vec::new()).push(nums[1]);
    });
    let numlines = input_iter.next().unwrap();
    let sum: i64 = numlines
        .lines()
        .map(|line| {
            let nums = line
                .split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let in_order = nums.iter().enumerate().all(|(ind, n)| {
                ((ind + 1)..nums.len()).all(|ind2| {
                    let n2 = nums[ind2 as usize];
                    comps.get(&n).is_some_and(|i| i.contains(&n2))
                })
            });
            if in_order {
                return nums[nums.len() / 2];
            }
            0
        })
        .sum();
    let sum2: i64 = numlines
        .lines()
        .map(|line| {
            let mut nums = line
                .split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let in_order = nums.iter().enumerate().all(|(ind, n)| {
                ((ind + 1)..nums.len()).all(|ind2| {
                    let n2 = nums[ind2 as usize];
                    comps.get(&n).is_some_and(|i| i.contains(&n2))
                })
            });
            if in_order {
                return 0;
            }
            nums.sort_by(|a, b| match comps.get(a).is_some_and(|i| i.contains(b)) {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            });
            nums[nums.len() / 2]
        })
        .sum();
    println!("oo {}", sum);
    println!("oo2 {}", sum2);
}

const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
