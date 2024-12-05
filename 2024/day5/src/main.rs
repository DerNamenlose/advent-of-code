use std::cmp::Ordering;

struct PageOrderingRule {
    first: u32,
    second: u32,
}

impl From<&str> for PageOrderingRule {
    fn from(s: &str) -> Self {
        let mut parts = s.split("|");
        Self {
            first: parts
                .next()
                .expect("Missing first rule part")
                .parse()
                .expect("Cannot parse first rule part"),
            second: parts
                .next()
                .expect("Missing second rule part")
                .parse()
                .expect("Cannot parse second rule part"),
        }
    }
}

fn is_correct(update: &Vec<u32>, rules: &Vec<PageOrderingRule>) -> bool {
    for idx in 0..update.len() {
        let entry = update[idx];
        let before_rules = rules.iter().filter(|rule| rule.first == entry);
        let after_rules = rules.iter().filter(|rule| rule.second == entry);

        // none of the elements before the checked one should come after it
        let before_correct = before_rules.clone().all(|rule| {
            update[..idx]
                .iter()
                .all(|&candidate| candidate != rule.second)
        });
        // none of the elements after the checked one should actually come before it
        let after_correct = after_rules.clone().all(|rule| {
            update[idx..]
                .iter()
                .all(|&candidate| candidate != rule.first)
        });
        if !before_correct || !after_correct {
            return false;
        }
    }
    true
}

fn load_input() -> (Vec<PageOrderingRule>, Vec<Vec<u32>>) {
    let data = std::fs::read_to_string("./input.txt").expect("Cannot read file");
    let mut rules: Vec<PageOrderingRule> = vec![];
    let mut lines = data.lines();

    loop {
        match lines.next() {
            None => break,
            Some(text) => {
                if text.contains("|") {
                    rules.push(text.into());
                } else {
                    break; // no longer a rule. We've moved to the next part of the file
                }
            }
        }
    }

    let updates: Vec<Vec<u32>> = lines
        .map(|line| {
            line.split(",")
                .map(|entry| entry.parse().expect("Cannot parse entry"))
                .collect()
        })
        .collect();

    (rules, updates)
}

fn task1() {
    let (rules, updates) = load_input();
    let correct_updates = updates.iter().filter(|update| is_correct(&update, &rules));
    let sum = correct_updates.fold(0, |existing, update| existing + update[update.len() / 2]);

    println!("Task 1: Sum: {sum}");
}

fn task2() {
    let (rules, updates) = load_input();
    let incorrect_updates = updates
        .iter()
        .filter(|&update| !is_correct(&update, &rules));

    let corrected_updates = incorrect_updates.into_iter().map(|update| {
        let mut copy = update.clone(); // TODO don't like the clone here, but sort_by requires a mutable reference
        copy.sort_by(|a, b| {
            // basically evaluate every rule to sort the thing
            for rule in rules.iter() {
                if rule.first == *a && rule.second == *b {
                    // correct order
                    return Ordering::Less;
                }
                if rule.first == *b && rule.second == *a {
                    // wrong way around
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });
        copy
    });

    let sum = corrected_updates.fold(0, |existing, update| existing + update[update.len() / 2]);

    println!("Task 2: Sum: {sum}");
}

fn main() {
    task1();
    task2();
}
