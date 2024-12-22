use std::collections::HashMap;

type Cache = HashMap<(u64, u64), u64>;
fn main() {
    let input = aoc2024::input("11");
    //let input = SAMPLE;
    let stones = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|i| i.parse::<u64>().unwrap());
    let mut cache: Cache = HashMap::new();
    println!(
        "dig: {}",
        stones.map(|s| dig(s, 75, &mut cache)).sum::<u64>()
    );
}

fn dig(stone: u64, steps: u64, cache: &mut Cache) -> u64 {
    let mut res = None;
    if steps == 0 {
        return 1;
    }
    if let Some(hit) = cache.get(&(stone, steps)) {
        return *hit;
    }
    if stone == 0 {
        res = Some(dig(1, steps - 1, cache));
    }
    let stone_s = stone.to_string();
    if stone_s.len() % 2 == 0 {
        let l = stone_s[0..(stone_s.len() / 2)].parse::<u64>().unwrap();
        let r = stone_s[(stone_s.len() / 2)..].parse::<u64>().unwrap();
        res = Some(dig(l, steps - 1, cache) + dig(r, steps - 1, cache));
    }
    if res.is_none() {
        res = Some(dig(stone * 2024, steps - 1, cache));
    }
    cache.insert((stone, steps), res.unwrap());
    res.unwrap()
}

const SAMPLE: &str = "125 17";
