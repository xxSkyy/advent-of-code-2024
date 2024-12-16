use std::{fs, thread::sleep, time::Duration};

use colored::Colorize;

const VISUALISATION: bool = true;
const DELAY_MS: u64 = 5;

const INPUT_LOCATION: &str = "./input.txt";

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    movable: bool,
    solid: bool,
    position: Position,
    child: bool,
    parent: bool,
}

type Map = Vec<Entity>;

impl Entity {
    fn step(
        &mut self,
        map: &mut Map,
        x: isize,
        y: isize,
        from: bool,
        master: bool,
    ) -> (bool, Vec<(usize, Entity)>) {
        if self.solid {
            return (false, vec![]);
        }

        let next_position = Position {
            x: (self.position.x as isize + x) as usize,
            y: (self.position.y as isize + y) as usize,
        };

        let mut update_indexes: Vec<(usize, Entity)> = vec![];
        let mut can_move = true;

        if (self.child || self.parent) && !from && x == 0 {
            let diff: isize = if self.child { -1 } else { 1 };

            let related_index = map
                .iter()
                .position(|entity| {
                    entity.position.x == (self.position.x as isize + diff) as usize
                        && entity.position.y == self.position.y
                })
                .unwrap();

            let mut parent = map[related_index].clone();

            let (can_related_move, mut child_update_indexes) = parent.step(map, x, y, true, false);

            update_indexes.append(&mut child_update_indexes);
            can_move = can_related_move && can_move;

            update_indexes.push((related_index, parent));
        }

        let obstacle_index = map.iter().position(|entity| {
            entity.position.x == next_position.x && entity.position.y == next_position.y
        });

        if let Some(obstacle_index) = obstacle_index {
            let mut obstacle = map[obstacle_index].clone();

            let (can_related_move, mut child_update_indexes) =
                obstacle.step(map, x, y, false, false);

            update_indexes.append(&mut child_update_indexes);
            can_move = can_related_move && can_move;

            update_indexes.push((obstacle_index, obstacle));
        }

        if !can_move {
            return (false, vec![]);
        };

        self.position = next_position;

        if master {
            for (index, item) in update_indexes.iter() {
                map[*index] = *item;
            }
        }

        return (true, update_indexes);
    }
}

type Movement = [isize; 2];

#[allow(dead_code)]
fn debug_map(map: &Map, index: &usize, max: &usize) {
    let size = 50;

    let mut debug_map = vec![];
    for _ in 0..size {
        let x = (0..size * 2)
            .map(|_| ".".truecolor(50, 50, 50).to_string())
            .collect::<Vec<_>>();
        debug_map.push(x);
    }
    map.iter().for_each(|entity| {
        let mut symbol = "#".truecolor(150, 150, 150).to_string();
        if entity.movable {
            symbol = "@".bright_yellow().to_string();
        }

        if !entity.solid && !entity.movable {
            if entity.parent {
                symbol = "[".bright_green().to_string();
            } else {
                symbol = "]".bright_green().to_string();
            }
        }

        debug_map[entity.position.y][entity.position.x] = symbol;
    });

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "{}",
        debug_map
            .iter()
            .map(|line| line.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("{}                                Frame: {:05} / {:05}  FPS: {:03}                                  {}", "##".truecolor(150, 150, 150),index,max, 1000/DELAY_MS, "##".truecolor(150, 150, 150));

    for _ in 0..size * 2 {
        print!("{}", "#".truecolor(150, 150, 150));
    }
    println!("");

    sleep(Duration::from_millis(DELAY_MS));
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let input = input.trim().split("\n\n").collect::<Vec<_>>();

    let mut map: Map = vec![];
    let mut movements: Vec<Movement> = vec![];

    input[0].lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let mut entity = Entity {
                movable: false,
                solid: true,
                position: Position { x: x * 2, y },
                child: false,
                parent: false,
            };

            match char {
                '#' => {
                    map.push(entity);

                    entity.position.x += 1;
                    map.push(entity);
                }
                'O' => {
                    entity.solid = false;
                    entity.parent = true;

                    map.push(entity);

                    entity.position.x += 1;
                    entity.parent = false;
                    entity.child = true;

                    map.push(entity);
                }
                '@' => {
                    entity.movable = true;
                    entity.solid = false;

                    map.push(entity)
                }
                _ => (),
            };
        })
    });

    input[1]
        .lines()
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .for_each(|movement| {
            match movement {
                '<' => movements.push([-1, 0]),
                '^' => movements.push([0, -1]),
                '>' => movements.push([1, 0]),
                'v' => movements.push([0, 1]),
                _ => (),
            };
        });

    let robot_index = map.iter().position(|entity| entity.movable).unwrap();

    movements.iter().enumerate().for_each(|(index, movement)| {
        let mut robot = map[robot_index].clone();
        robot.step(&mut map, movement[0], movement[1], false, true);

        map[robot_index] = robot;

        if VISUALISATION {
            debug_map(&map, &index, &movements.len());
        }
    });

    let sum = map
        .iter()
        .filter(|entity| !entity.movable && !entity.solid && entity.parent)
        .fold(0, |acc, entity| {
            acc + entity.position.y * 100 + entity.position.x
        });

    println!("Sum: {:?}", sum);
}
