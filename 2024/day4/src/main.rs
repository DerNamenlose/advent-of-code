fn get_words_at(
    text: &Vec<char>,
    line_length: isize,
    num_lines: isize,
    top: isize,
    left: isize,
) -> Vec<String> {
    // the jump offsets to get to the next character in any direction. 0 entries are ignored later on, so we use them as markers when getting next to the borders
    let mut offsets = vec![
        1,                // right
        line_length + 1,  // down-right
        line_length,      // down
        line_length - 1,  // down-left
        -1,               // left
        -line_length - 1, // up-left
        -line_length,     // up
        -line_length + 1, // up-right
    ];
    // remove invalid jump directions (because the word wouldn't fit anyway)
    if top < 3 {
        offsets[5] = 0;
        offsets[6] = 0;
        offsets[7] = 0;
    };
    if top > num_lines - 4 {
        offsets[1] = 0;
        offsets[2] = 0;
        offsets[3] = 0;
    };
    if left < 3 {
        offsets[3] = 0;
        offsets[4] = 0;
        offsets[5] = 0;
    };
    if left > line_length - 5
    /* don't try the final \n */
    {
        offsets[0] = 0;
        offsets[1] = 0;
        offsets[7] = 0;
    };
    let mut result = vec![];
    for &offset in offsets.iter() {
        if offset != 0 {
            let mut string = String::new();
            for i in 0..4 {
                let index = top * line_length + left + offset * i;
                if index >= text.len() as isize {
                    println!("Offsets: {:#?}, current: {}, index: {}", offsets, offset, i);
                    panic!(
                        "Invalid index: {} = {} * {} + {} + {} * {}",
                        index, top, line_length, left, offset, i
                    );
                }
                string.push(text[index as usize]);
            }
            result.push(string);
        }
    }
    result
}

fn task1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let text: Vec<char> = input.chars().collect(); // makes the offset jumperoo easier in get_words_at
    let line_length = (input.lines().next().unwrap().len() + 1) as isize; // need to include the final \n to get the calculations right
    let num_lines = input.len() as isize / line_length;
    let mut count = 0;
    println!("Text has {} lines of length {}", num_lines, line_length);
    for top in 0..num_lines {
        for left in 0..line_length {
            count += get_words_at(&text, line_length, num_lines, top, left)
                .iter()
                .fold(0, |existing, word| {
                    if word == "XMAS" {
                        existing + 1
                    } else {
                        existing
                    }
                });
        }
    }
    println!("Task 1: total: {}", count);
}

fn task2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let text: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    for row_idx in 1..text.len() - 1 {
        for col_idx in 1..text[row_idx].len() - 1 {
            if text[row_idx][col_idx] == 'A'
                && (
                    // top-left-bottom-right word
                    text[row_idx - 1][col_idx - 1] == 'M' && text[row_idx + 1][col_idx + 1] == 'S'
                        || text[row_idx - 1][col_idx - 1] == 'S'
                            && text[row_idx + 1][col_idx + 1] == 'M'
                )
                && (
                    // bottom-left-top-right word
                    text[row_idx + 1][col_idx - 1] == 'M' && text[row_idx - 1][col_idx + 1] == 'S'
                        || text[row_idx + 1][col_idx - 1] == 'S'
                            && text[row_idx - 1][col_idx + 1] == 'M'
                )
            {
                count += 1;
            }
        }
    }
    println!("Task 2: Total: {}", count);
}

fn main() {
    task1();
    task2();
}
