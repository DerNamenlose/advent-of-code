use std::usize;

type BlockId = usize;
type Disk = Vec<Option<BlockId>>;

fn load_input() -> Disk {
    let input = std::fs::read_to_string("input.txt").expect("Should be able to read input file");
    // let input = "2333133121414131402";

    input
        .chars()
        .enumerate()
        .flat_map(|(idx, ch)| {
            let is_empty = idx % 2 == 1;
            if let Some(num) = ch.to_digit(10) {
                let id = idx / 2;
                vec![if is_empty { None } else { Some(id) }; num as usize]
            } else {
                vec![] // ignoring invalid characters in the input (at the end)
            }
        })
        .collect()
}

fn print_disk(d: &Disk) {
    for ch in d {
        if let Some(ch) = ch {
            print!("{}", ch);
        } else {
            print!(".");
        }
    }
    println!("");
}

fn task1() {
    let mut disk = load_input();
    // print_disk(&disk);

    let mut free_idx = 0;
    let mut last_block_idx = disk.len() - 1;

    while free_idx < last_block_idx {
        // find the first free block
        while free_idx < last_block_idx && disk[free_idx].is_some() {
            free_idx += 1;
        }
        // find the last used block
        while last_block_idx > free_idx && disk[last_block_idx].is_none() {
            last_block_idx -= 1;
        }

        // found something to swap
        if free_idx < last_block_idx {
            disk[free_idx] = disk[last_block_idx];
            disk[last_block_idx] = None;
        }
    }

    // calculate the checksum
    let checksum = disk.iter().enumerate().fold(0, |checksum, (idx, block)| {
        if let Some(value) = block {
            checksum + idx * value
        } else {
            checksum
        }
    });

    println!("Task 1: checksum: {checksum}");
}

fn task2() {
    let mut disk = load_input();

    let mut movable_file_idx = disk.len() - 1;
    let mut current_file_id;

    while movable_file_idx > 0 {
        // find the end of the next file
        while movable_file_idx > 0 && disk[movable_file_idx].is_none() {
            movable_file_idx -= 1;
        }
        current_file_id = disk[movable_file_idx];
        // find the start of the file
        let mut file_length = 0;
        while movable_file_idx > 0 && disk[movable_file_idx] == current_file_id {
            movable_file_idx -= 1;
            file_length += 1;
        }
        movable_file_idx += 1; // one step back because the while loop move _past_ the start of the file
                               // find block from the start, that is big enough for the file
        let target_idx = 'space_search: {
            for idx in 0..movable_file_idx {
                if (0..file_length).all(|target_offset| disk[idx + target_offset].is_none()) {
                    break 'space_search idx;
                }
            }
            movable_file_idx
        };
        if target_idx < movable_file_idx {
            // found a block, move the file
            for offset in 0..file_length {
                disk[target_idx + offset] = disk[movable_file_idx + offset];
                disk[movable_file_idx + offset] = None;
            }
        } else {
            movable_file_idx -= 1; // step before the file that we were unable to move
        }
    }

    // calculate the checksum
    let checksum = disk.iter().enumerate().fold(0, |checksum, (idx, block)| {
        if let Some(value) = block {
            checksum + idx * value
        } else {
            checksum
        }
    });

    println!("Task 2: checksum: {checksum}");
}

fn main() {
    task1();
    task2();
}
