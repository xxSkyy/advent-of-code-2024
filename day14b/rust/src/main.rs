use std::fs;

use colored::Colorize;
use regex::Regex;

const INPUT_LOCATION: &str = "./input.txt";

const MAP_SIZE: [usize; 2] = [101, 103];
const TICKS: usize = 1000000;

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

    for second in 1..=TICKS {
        robots.iter_mut().for_each(|robot| robot.step());

        let mut should_show_image = false;
        let to_find = 8;

        for robot in robots.iter() {
            let mut found = 0;
            for y_plus in 1..=to_find {
                let exist = robots.iter().find(|other_robot| {
                    other_robot.position.y == robot.position.y + y_plus
                        && other_robot.position.x == robot.position.x
                });
                if let Some(_) = exist {
                    found += 1;
                    continue;
                }
            }

            if found == to_find {
                should_show_image = true;
                break;
            }
        }

        if !should_show_image {
            continue;
        }

        let mut map: Vec<Vec<String>> = vec![vec![]; MAP_SIZE[1]];

        for y in map.iter_mut() {
            for _ in 0..MAP_SIZE[0] {
                y.push(("**".black()).to_string());
            }
        }

        robots.iter().for_each(|robot| {
            let y = robot.position.y;
            let x = robot.position.x;

            map[y][x] = ("**".green()).to_string();
        });

        let display_map = map
            .iter()
            .map(|line| line.join(""))
            .collect::<Vec<_>>()
            .join("\n");

        println!("{} ||", display_map);
        println!("Second: {}", second);
        break;
    }
}
