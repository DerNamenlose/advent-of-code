type HeightMap = Vec<Vec<i8>>;

fn load_input() -> HeightMap {
    let data = std::fs::read_to_string("input.txt").expect("Could not read file");
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Could not parse digit") as i8)
                .collect()
        })
        .collect()
}

// find the next steps from the given point in the map. returns the number of reachable peaks (i.e. 9s)
fn find_way(
    height_map: &mut HeightMap,
    x: i32,
    y: i32,
    previous_height: i8,
    all_paths: bool,
) -> usize {
    if x < 0 || x >= height_map[0].len() as i32 || y < 0 || y >= height_map.len() as i32 {
        return 0;
    }
    let current_height = height_map[y as usize][x as usize];
    if current_height != previous_height + 1 {
        return 0;
    }
    if current_height == 9 {
        if !all_paths {
            height_map[y as usize][x as usize] = -1; // Mark the peak as already reached as to not count it twice
        }
        return 1;
    }
    // check in all 4 directions. While this technically means, that we will check back the way we came, this massively simplifies the algorithm and
    //   will lead to the same result, because the way we came will always be lower, than the current field, causing the recursion to stop in the next step
    find_way(height_map, x - 1, y, current_height, all_paths)
        + find_way(height_map, x, y - 1, current_height, all_paths)
        + find_way(height_map, x + 1, y, current_height, all_paths)
        + find_way(height_map, x, y + 1, current_height, all_paths)
}

fn task1() {
    let height_map = load_input();
    let score = height_map.iter().enumerate().fold(0, |sum, (y, line)| {
        sum + line.iter().enumerate().fold(0, |line_sum, (x, &height)| {
            if height == 0 {
                let mut copy = height_map.clone();
                let ways = find_way(&mut copy, x as i32, y as i32, -1, false);
                line_sum + ways
            } else {
                line_sum
            }
        })
    });

    println!("Task 1: score: {score}");
}

fn task2() {
    let mut height_map = load_input();
    let score = height_map.iter().enumerate().fold(0, |sum, (y, line)| {
        sum + line.iter().enumerate().fold(0, |line_sum, (x, &height)| {
            if height == 0 {
                let mut copy: Vec<Vec<i8>> = height_map.clone();
                let ways = find_way(&mut copy, x as i32, y as i32, -1, true);
                line_sum + ways
            } else {
                line_sum
            }
        })
    });

    println!("Task 2: rating: {score}");
}

fn main() {
    task1();
    task2();
}
