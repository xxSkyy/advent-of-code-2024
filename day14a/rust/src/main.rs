use std::fs;

use regex::Regex;

const INPUT_LOCATION: &str = "./input.txt";

const MAP_SIZE: [usize; 2] = [101, 103];
const TICKS: usize = 1000;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn warp(position: isize, size: usize) -> usize {
    if position < 0 {
        return (position + size as isize) as usize;
    }

    (position % size as isize) as usize
}

impl Robot {
    fn step(&mut self) {
        let next_position_x = warp(self.position.x as isize + self.velocity.x, MAP_SIZE[0]);
        let next_position_y = warp(self.position.y as isize + self.velocity.y, MAP_SIZE[1]);

        self.position.x = next_position_x;
        self.position.y = next_position_y;
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");
    let robot_regex = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").expect("Regex issue");

    let mut robots = vec![];

    input.trim().lines().for_each(|line| {
        let robot_data = robot_regex.captures_iter(line).next().unwrap();

        let robot = Robot {
            position: Position {
                x: robot_data[1].parse().unwrap(),
                y: robot_data[2].parse().unwrap(),
            },
            velocity: Velocity {
                x: robot_data[3].parse().unwrap(),
                y: robot_data[4].parse().unwrap(),
            },
        };

        robots.push(robot);
    });

    for _ in 0..TICKS {
        robots.iter_mut().for_each(|robot| robot.step());
    }

    let middle_x = MAP_SIZE[0] / 2;
    let middle_y = MAP_SIZE[1] / 2;

    let mut quadrants = [0; 4];

    robots
        .iter()
        .filter(|robot| robot.position.x != middle_x && robot.position.y != middle_y)
        .for_each(
            |robot| match [robot.position.x < middle_x, robot.position.y < middle_y] {
                [true, true] => quadrants[0] += 1,
                [false, true] => quadrants[1] += 1,
                [true, false] => quadrants[2] += 1,
                [false, false] => quadrants[3] += 1,
            },
        );

    let sum = quadrants.iter().fold(1, |acc, quadrant| acc * quadrant);

    println!("Sum: {:#?}", sum);
}
