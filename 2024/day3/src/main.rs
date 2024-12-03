use regex::Regex;

fn task1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mul_inst_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum = mul_inst_regex
        .captures_iter(&input)
        .fold(0, |existing, captures| {
            let op1 = captures.get(1).unwrap();
            let op2 = captures.get(2).unwrap();

            existing + op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap()
        });

    println!("Task 1: Sum: {}", sum);
}

fn task2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let inst_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut is_enabled = true;
    for instruction in inst_regex.captures_iter(&input) {
        match instruction.get(0).and_then(|m| Some(m.as_str())) {
            Some("do()") => {
                is_enabled = true;
            }
            Some("don't()") => {
                is_enabled = false;
            }
            Some(_) => {
                if is_enabled {
                    let op1 = instruction.get(1).unwrap();
                    let op2 = instruction.get(2).unwrap();

                    sum +=
                        op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap()
                }
            }
            None => panic!("No clue, what I matched!"),
        }
    }
    println!("Task 2: Sum: {}", sum);
}

fn main() {
    task1();
    task2();
}
