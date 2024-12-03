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

fn is_safe(report: &Vec<i32>) -> bool {
    let mut last_value = report[0];
    let mut increment = 0;
    // the current increment between two values. Mainly for tracking the direction of travel of the series
    for current_value in report.iter().skip(1) {
        let difference = current_value - last_value;
        if increment * difference < 0 // i.e. difference & increment have different signs
            || difference.abs() < 1
            || difference.abs() > 3
        {
            return false;
        }
        last_value = *current_value;
        increment = difference;
    }
    true
}

fn task1() {
    let reports = read_input();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    for report in reports.iter() {
        if is_safe(report) {
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
        if is_safe(&report) {
            safe_reports += 1;
        } else {
            // check whether any value being left out makes it safe
            let has_safe = 'safe_check: {
                for ignore_idx in 0..report.len() {
                    let mut modified_report = report.clone();
                    modified_report.splice(ignore_idx..ignore_idx + 1, []);
                    if is_safe(&modified_report) {
                        safe_reports += 1; // if there's one version how to make it safe, we're good
                        break 'safe_check true;
                    }
                }
                false // couldn't find a safe report
            };
            if !has_safe {
                unsafe_reports += 1;
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
