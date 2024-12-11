use std::collections::HashMap;

fn load_input() -> Vec<u64> {
    std::fs::read_to_string("./input.txt")
        .expect("Read file failed")
        .trim()
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn cache_value(cache: &mut HashMap<(u64, u32), u64>, stone: u64, run: u32, count: u64) {
    if stone < 1000 {
        cache.insert((stone, run), count);
    }
}

fn count(stone: u64, run: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if cache.contains_key(&(stone, run)) {
        return cache.get(&(stone, run)).unwrap().clone();
    }

    if run == 0 {
        return 1;
    }

    if stone == 0 {
        let c = count(1, run - 1, cache);
        cache_value(cache, stone, run, c);
        return c;
    }

    let val_str = stone.to_string();
    if val_str.len() % 2 == 0 {
        let c = count(
            val_str[0..val_str.len() / 2].parse::<u64>().unwrap(),
            run - 1,
            cache,
        ) + count(
            val_str[val_str.len() / 2..].parse::<u64>().unwrap(),
            run - 1,
            cache,
        );
        cache_value(cache, stone, run, c);
        return c;
    }
    let c = count(stone * 2024, run - 1, cache);
    cache_value(cache, stone, run, c);
    return c;
}

fn task1() {
    let stones = load_input();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let count = stones
        .iter()
        .fold(0, |sum, &stone| sum + count(stone, 25, &mut cache));

    println!("Task 1: # stones: {}", count);
}

fn task2() {
    let stones = load_input();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let count = stones
        .iter()
        .fold(0, |sum, &stone| sum + count(stone, 75, &mut cache));

    println!("Task 2: # stones: {}", count);
}

fn main() {
    task1();
    task2();
}
