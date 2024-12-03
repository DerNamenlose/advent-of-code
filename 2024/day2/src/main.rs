fn read_input() -> Vec<Vec<i32>> {
    let string = std::fs::read_to_string("./input.txt").unwrap();
    string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_unsafe(report: &Vec<i32>) -> Option<usize> {
    let mut last_value = report[0];
    let mut increment = 0;
    // the current increment between two values. Mainly for tracking the direction of travel of the series
    for (idx, current_value) in report.iter().enumerate().skip(1) {
        let difference = current_value - last_value;
        if increment * difference < 0 // i.e. difference & increment have different signs
            || difference.abs() < 1
            || difference.abs() > 3
        {
            return Some(idx);
        }
        last_value = *current_value;
        increment = difference;
    }
    None
}

fn task1() {
    let reports = read_input();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    for report in reports.iter() {
        if find_unsafe(report).is_none() {
            safe_reports += 1;
        } else {
            unsafe_reports += 1;
        }
    }

    println!(
        "Task1: Total reports: {}, safe: {}, unsafe: {}",
        reports.iter().len(),
        safe_reports,
        unsafe_reports
    );
}

fn task2() {
    let reports = read_input();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    let total = reports.len();
    for report in reports.into_iter() {
        match find_unsafe(&report) {
            Some(idx) => {
                // got to check three cases here: removing the last item of the failed partial sequence, the one before or the one before that (to account for pattern
                //   like  3, 2, 3, 4, 5 at the start of a pattern, which becomes valid when removing index 0, but we'll only notice the issue at position 2)
                let has_safe = 'safe_check: {
                    // make sure we don't actually access before the vector if the first gap is already invalie
                    for i in 0..std::cmp::min(3, idx + 1) {
                        // remove the potentially unsafe item  check if that makes it safe
                        let mut modified_report = report.clone();
                        modified_report.splice(idx - i..idx - i + 1, []);
                        if find_unsafe(&modified_report).is_none() {
                            safe_reports += 1;
                            break 'safe_check true;
                        }
                    }
                    false
                };
                if !has_safe {
                    unsafe_reports += 1;
                }
            }
            None => {
                safe_reports += 1;
            }
        }
    }

    println!(
        "Task2: Total reports: {}, safe: {}, unsafe: {}",
        total, safe_reports, unsafe_reports
    );
}

fn main() {
    task1();
    task2();
}
