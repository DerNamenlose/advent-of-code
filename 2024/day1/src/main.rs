fn read_values() -> (Vec<i32>, Vec<i32>) {
    let string = std::fs::read_to_string("./input.txt").unwrap();
    let mut left_vec = vec![];
    let mut right_vec = vec![];

    for line in string.lines() {
        let mut values = line.split_whitespace();
        let left_value: i32 = values.next().unwrap().parse().unwrap();
        left_vec.push(left_value);
        let right_value: i32 = values.next().unwrap().parse().unwrap();
        right_vec.push(right_value);
    }

    return (left_vec, right_vec);
}

fn task1() {
    let (mut left_vec, mut right_vec) = read_values();

    left_vec.sort();
    right_vec.sort();

    let mut sum = 0;
    for idx in 0..left_vec.len() {
        sum = sum + (left_vec[idx] - right_vec[idx]).abs();
    }

    println!("Task 1 Sum: {}", sum);
}

fn task2() {
    let (left_vec, right_vec) = read_values();

    let mut sum = 0;
    for left_number in left_vec.iter() {
        let occurences = right_vec
            .iter()
            .filter(|&candidate| candidate == left_number)
            .count() as i32;
        sum += left_number * occurences;
    }

    println!("Task 2 Sum: {}", sum);
}

fn main() {
    task1();
    task2();
}
