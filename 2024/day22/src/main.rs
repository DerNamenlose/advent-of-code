#![feature(map_try_insert)]

use std::collections::{HashMap, VecDeque};

fn mix_and_prune(intermediate: i64, number: i64) -> i64 {
    (intermediate ^ number) % 16777216
}

fn next_secret_number(number: i64) -> i64 {
    let mut result = mix_and_prune(number * 64, number);
    result = mix_and_prune(result / 32, result);
    result = mix_and_prune(result * 2048, result);
    result
}

fn nth_secret_number(start: i64, n: u32) -> i64 {
    let mut result = start;
    for _ in 0..n {
        result = next_secret_number(result);
    }
    result
}

fn task1() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap());

    let sum: i64 = numbers.map(|number| nth_secret_number(number, 2000)).sum();

    println!("Task 1: sum {sum}");
}

fn calculate_sequence(number: i64) -> HashMap<Vec<i64>, i64> {
    let mut current_number = number;
    let mut result = HashMap::new();
    let mut current_pattern = VecDeque::new();
    for _ in 0..2000 {
        let next_number = next_secret_number(current_number);
        let next_last_digit = next_number % 10;
        let current_last_digit = current_number % 10;
        current_pattern.push_back(next_last_digit - current_last_digit);
        if current_pattern.len() == 4 {
            // we're only ever interested in the first occurence of a pattern in the sequence, because that's what counts
            let _ = result.try_insert(
                Vec::from_iter(current_pattern.iter().cloned()),
                next_last_digit,
            );
            current_pattern.pop_front(); // throw away the first digit. The pattern is a sliding window over the sequence
        }
        current_number = next_number;
    }
    result
}

fn task2() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap());

    let sequences: Vec<_> = numbers.map(calculate_sequence).collect();

    let mut banana_counters = HashMap::new();
    for sequence in sequences.iter() {
        for (pattern, price) in sequence.iter() {
            banana_counters.insert(
                pattern,
                banana_counters.get(pattern).unwrap_or(&0_i64) + price,
            );
        }
    }

    let sum = banana_counters.values().max();

    println!("Task 2: sum: {sum:?}");
}

fn main() {
    task1();
    task2();
}
