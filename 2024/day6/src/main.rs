type Map = Vec<Vec<char>>;

fn load_map() -> Map {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map: Map = input
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect();
    map
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct GuardPosition {
    x: usize,
    y: usize,
    direction: Direction,
}

fn get_guard_position(map: &Map) -> GuardPosition {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                return GuardPosition {
                    x,
                    y,
                    direction: Direction::Up,
                };
            }
        }
    }
    panic!("Did not find guard start postion");
}

/// Update the guard position based on the guard movement rules. Returns the new position or None if the guard has left the map
fn step_guard(map: &Map, guard: GuardPosition) -> Option<GuardPosition> {
    match guard.direction {
        Direction::Down => {
            if guard.y == map.len() - 1 {
                None
            } else if map[guard.y + 1][guard.x] == '#' {
                // we could optimize the step by already moving where the guard will be next, but this saves us from having to check, whether we're at the edge already
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y,
                    direction: Direction::Left,
                })
            } else {
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y + 1,
                    direction: Direction::Down,
                })
            }
        }
        Direction::Up => {
            if guard.y == 0 {
                None
            } else if map[guard.y - 1][guard.x] == '#' {
                // we could optimize the step by already moving where the guard will be next, but this saves us from having to check, whether we're at the edge already
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y,
                    direction: Direction::Right,
                })
            } else {
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y - 1,
                    direction: Direction::Up,
                })
            }
        }
        Direction::Left => {
            if guard.x == 0 {
                None
            } else if map[guard.y][guard.x - 1] == '#' {
                // we could optimize the step by already moving where the guard will be next, but this saves us from having to check, whether we're at the edge already
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y,
                    direction: Direction::Up,
                })
            } else {
                Some(GuardPosition {
                    x: guard.x - 1,
                    y: guard.y,
                    direction: Direction::Left,
                })
            }
        }
        Direction::Right => {
            if guard.x == map[guard.y].len() - 1 {
                None
            } else if map[guard.y][guard.x + 1] == '#' {
                // we could optimize the step by already moving where the guard will be next, but this saves us from having to check, whether we're at the edge already
                Some(GuardPosition {
                    x: guard.x,
                    y: guard.y,
                    direction: Direction::Down,
                })
            } else {
                Some(GuardPosition {
                    x: guard.x + 1,
                    y: guard.y,
                    direction: Direction::Right,
                })
            }
        }
    }
}

fn task1() {
    let mut map = load_map();

    let mut guard_opt = Some(get_guard_position(&map));
    loop {
        if let Some(guard) = guard_opt {
            map[guard.y][guard.x] = 'X';
            guard_opt = step_guard(&map, guard);
        } else {
            break;
        }
    }

    let places_touched = map.iter().fold(0, |sum, line| {
        sum + line.iter().fold(
            0,
            |line_sum, &ch| if ch == 'X' { line_sum + 1 } else { line_sum },
        )
    });

    println!("Task 1: Places: {places_touched}");
}

fn task2() {
    let map_orig = load_map();
    let mut possible_positions = 0;

    // try out all possible positions and add some obstacle there
    for y in 0..map_orig.len() {
        for x in 0..map_orig[y].len() {
            let mut map = map_orig.clone();
            let mut guard_opt = Some(get_guard_position(&map));

            if map[x][y] == '#' || map[x][y] == '^' {
                continue; // no need to check here, there's already an obstacle or the guard
            }
            let current_element = map[x][y];
            map[x][y] = '#';
            let mut existing_turns = Vec::<GuardPosition>::new();
            loop {
                if let Some(guard) = guard_opt.as_ref() {
                    let new_guard_pos = step_guard(&map, guard.clone());
                    if let Some(np) = new_guard_pos.as_ref() {
                        if np.direction != guard.direction {
                            // we changed direction -> check whether we've changed the direction in the exact same way here before, which would mean we're in a loop
                            if existing_turns.contains(np) {
                                possible_positions += 1;
                                break;
                            } else {
                                existing_turns.push(np.clone()); // remember the turn
                            }
                        }
                    }
                    guard_opt = new_guard_pos;
                } else {
                    break;
                }
            }
            map[x][y] = current_element;
        }
    }

    println!("Task 2: Possible positions: {possible_positions}");
}

fn main() {
    task1();
    task2();
}
