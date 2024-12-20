use std::collections::{HashMap, HashSet};

fn load_input() -> (Vec<String>, Vec<String>) {
    let mut data = include_str!("../input.txt").lines();

    let mut available_towels: Vec<String> = data
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() > 0)
        .collect();
    available_towels.sort_by_key(|s| 100 - s.len());

    data.next();

    let desired_patterns: Vec<String> = data.map(|s| s.to_string()).collect();

    (available_towels, desired_patterns)
}

// fn is_pattern_possible(
//     desired_pattern: &str,
//     available_towels: &Vec<String>,
//     known_bad: &mut HashSet<String>, // known bad patterns that cannot be built
// ) -> bool {
//     if known_bad.contains(desired_pattern) {
//         return false;
//     }
//     for towel in available_towels {
//         if desired_pattern.starts_with(towel) {
//             if desired_pattern.len() == towel.len() {
//                 return true; // complete match, we found something
//             }

//             if is_pattern_possible(&desired_pattern[towel.len()..], available_towels, known_bad) {
//                 return true;
//             }
//         }
//     }
//     known_bad.insert(desired_pattern.to_string());
//     return false;
// }

fn find_possible_solutions(
    desired_pattern: &str,
    available_towels: &Vec<String>,
    known_solutions: &mut HashMap<String, usize>,
) -> usize {
    if let Some(known) = known_solutions.get(desired_pattern) {
        return *known;
    }

    let mut possible_solutions = 0;
    for towel in available_towels {
        if desired_pattern.starts_with(towel) {
            if desired_pattern.len() == towel.len() {
                // exact match, count and continue
                possible_solutions += 1;
            } else {
                possible_solutions += find_possible_solutions(
                    &desired_pattern[towel.len()..],
                    &available_towels,
                    known_solutions,
                );
            }
        }
    }

    known_solutions.insert(desired_pattern.to_string(), possible_solutions);
    possible_solutions
}

fn task1() {
    let (available_towels, desired_patterns) = load_input();

    let total_possible_patterns = desired_patterns
        .iter()
        .filter(|&pattern| {
            find_possible_solutions(&pattern, &available_towels, &mut HashMap::new()) > 0
        })
        .count();

    println!("Task 1: possible patterns: {total_possible_patterns}");
}

fn task2() {
    let (available_towels, desired_patterns) = load_input();

    let total_possible_combinations = desired_patterns.iter().fold(0, |existing, pattern| {
        existing + find_possible_solutions(&pattern, &available_towels, &mut HashMap::new())
    });

    println!("Task 2: possible combinations: {total_possible_combinations}");
}

fn main() {
    task1();
    task2();
}
