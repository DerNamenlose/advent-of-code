use std::collections::{HashMap, HashSet};

struct Antenna {
    x: usize,
    y: usize,
    marker: char,
}

struct AntennaMap {
    width: usize,
    height: usize,
    antennas: Vec<Antenna>,
}

#[derive(Hash, PartialEq, Eq)]
struct Antinode {
    x: usize,
    y: usize,
}

fn load_input() -> AntennaMap {
    let input = std::fs::read_to_string("./input.txt").expect("Should be able to read input file");
    let lines: Vec<&str> = input.lines().collect();
    let antennas: Vec<Antenna> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch != '.' {
                    Some(Antenna { x, y, marker: ch })
                } else {
                    None
                }
            })
        })
        .collect();
    AntennaMap {
        width: lines.iter().next().unwrap().len(),
        height: lines.len(),
        antennas,
    }
}

fn group_antennas(antennas: &Vec<Antenna>) -> HashMap<char, Vec<&Antenna>> {
    let mut map = HashMap::new();
    for antenna in antennas.into_iter() {
        if !map.contains_key(&antenna.marker) {
            map.insert(antenna.marker, vec![]);
        }
        map.get_mut(&antenna.marker).unwrap().push(antenna);
    }
    map
}

fn get_nth_antinode(antenna_a: &Antenna, antenna_b: &Antenna, n: usize) -> Option<Antinode> {
    if (n + 1) * antenna_a.x >= n * antenna_b.x && (n + 1) * antenna_a.y >= n * antenna_b.y {
        Some(Antinode {
            x: (n + 1) * antenna_a.x - n * antenna_b.x,
            y: (n + 1) * antenna_a.y - n * antenna_b.y,
        })
    } else {
        None
    }
}

fn get_antinodes(
    antennas: &Vec<&Antenna>,
    width: usize,
    height: usize,
    max_n: usize,
    include_self: bool,
) -> Vec<Antinode> {
    let mut antinodes = vec![];
    for (idx, antenna_a) in antennas.iter().enumerate() {
        for antenna_b in antennas[idx + (if include_self { 0 } else { 1 })..].iter() {
            for n in 1..=max_n {
                antinodes.push(get_nth_antinode(antenna_a, antenna_b, n));
                antinodes.push(get_nth_antinode(antenna_b, antenna_a, n));
            }
        }
    }
    antinodes
        .into_iter()
        .filter_map(|node_opt| node_opt)
        .filter(|node| node.x < width && node.y < height)
        .collect()
}

fn task1() {
    let antenna_map = load_input();
    let by_frequency = group_antennas(&antenna_map.antennas);
    let all_antinodes = by_frequency
        .iter()
        .flat_map(|entry| get_antinodes(entry.1, antenna_map.width, antenna_map.height, 1, false));
    let unique_antinodes: HashSet<Antinode> = all_antinodes.collect();

    println!("Task 1: Unique nodes: {}", unique_antinodes.len());
}

fn task2() {
    let antenna_map = load_input();
    let by_frequency = group_antennas(&antenna_map.antennas);
    let all_antinodes = by_frequency
        .iter()
        .flat_map(|entry| get_antinodes(entry.1, antenna_map.width, antenna_map.height, 50, true)); // arbitrarily high n to make sure we get everything inside the map
    let unique_antinodes: HashSet<Antinode> = all_antinodes.collect();

    println!("Task 2: Unique nodes: {}", unique_antinodes.len());
}

fn main() {
    task1();
    task2();
}
