use std::{
    i32,
    io::{stdin, Read},
};

use lazy_static::lazy_static;
use regex::Regex;

struct Robot {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

impl Robot {
    fn step(&mut self) {
        self.x += self.v_x;
        self.x %= WIDTH;
        if self.x < 0 {
            self.x += WIDTH;
        };
        self.y += self.v_y;
        self.y %= HEIGHT;
        if self.y < 0 {
            self.y += HEIGHT;
        }
    }
}

lazy_static! {
    static ref ROBOT_REGEX: Regex = Regex::new(r"p=(\d+),(\d+)\s+v=([-\d]+),([-\d]+)").unwrap();
}

fn load_input() -> Vec<Robot> {
    include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let captures = ROBOT_REGEX
                .captures(line)
                .expect(format!("Failed to parse robot in line {idx}").as_str());
            Robot {
                x: captures
                    .get(1)
                    .and_then(|m| m.as_str().parse().ok())
                    .expect(format!("Couldn't parse x position in line {idx}").as_str()),
                y: captures
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .expect(format!("Couldn't parse y position in line {idx}").as_str()),
                v_x: captures
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .expect(format!("Couldn't parse x velocity in line {idx}").as_str()),
                v_y: captures
                    .get(4)
                    .and_then(|m| m.as_str().parse().ok())
                    .expect(format!("Couldn't parse y velocity in line {idx}").as_str()),
            }
        })
        .collect()
}

fn calculate_safety_factor(robots: &Vec<Robot>) -> i32 {
    let (q1, q2, q3, q4) = robots.iter().fold((0, 0, 0, 0), |(q1, q2, q3, q4), robot| {
        if robot.x < 50 {
            if robot.y < 51 {
                return (q1 + 1, q2, q3, q4);
            }
            if robot.y > 51 {
                return (q1, q2 + 1, q3, q4);
            }
        }
        if robot.x > 50 {
            if robot.y < 51 {
                return (q1, q2, q3 + 1, q4);
            }
            if robot.y > 51 {
                return (q1, q2, q3, q4 + 1);
            }
        }
        (q1, q2, q3, q4)
    });

    q1 * q2 * q3 * q4
}

fn task1() {
    let mut robots = load_input();

    for i in 0..100 {
        robots.iter_mut().for_each(|r| r.step());
    }

    println!(
        "Task 1: safety factor: {}",
        calculate_safety_factor(&robots)
    );
}

fn render_map(robots: &Vec<Robot>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robots.iter().any(|r| r.x == x && r.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn task2() {
    let mut robots = load_input();
    let mut min_safety = i32::MAX;
    for sec in 1..1000000 {
        robots.iter_mut().for_each(|r| r.step());

        let safety = calculate_safety_factor(&robots);
        if safety < min_safety {
            min_safety = safety;
            render_map(&robots);
            println!("Seconds: {sec}");
            stdin().read(&mut [0u8]).unwrap();
        }
    }
}

fn main() {
    task1();
    task2();
}
