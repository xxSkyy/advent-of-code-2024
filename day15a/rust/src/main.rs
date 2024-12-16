use std::fs;

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
}

type Map = Vec<Entity>;

impl Entity {
    fn step(&mut self, map: &mut Map, x: isize, y: isize) -> bool {
        if self.solid {
            return false;
        }

        let next_position = Position {
            x: (self.position.x as isize + x) as usize,
            y: (self.position.y as isize + y) as usize,
        };

        let obstacle_index = map.iter().position(|entity| {
            entity.position.x == next_position.x && entity.position.y == next_position.y
        });

        if let Some(obstacle_index) = obstacle_index {
            let mut obstacle = map[obstacle_index].clone();

            if !obstacle.step(map, x, y) {
                return false;
            }

            map[obstacle_index] = obstacle;
        }

        self.position = next_position;

        return true;
    }
}

type Movement = [isize; 2];

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
                position: Position { x, y },
            };

            match char {
                '#' => map.push(entity),
                'O' => {
                    entity.solid = false;

                    map.push(entity)
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

    println!("{:?} \n {:?}", map, movements);

    let robot_index = map.iter().position(|entity| entity.movable).unwrap();

    movements.iter().for_each(|movement| {
        let mut robot = map[robot_index].clone();
        robot.step(&mut map, movement[0], movement[1]);

        map[robot_index] = robot;

        println!("{:?} -> {:?}", movement, map[robot_index]);
    });

    let sum = map
        .iter()
        .filter(|entity| !entity.movable && !entity.solid)
        .fold(0, |acc, entity| {
            acc + entity.position.y * 100 + entity.position.x
        });

    println!("Sum: {:?}", sum);
}
