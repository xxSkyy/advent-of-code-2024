use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::{Arc, Mutex},
};

const INPUT_LOCATION: &str = "./input.txt";

const TURN_PRICE: u16 = 0;
const FORWARD_PRICE: u8 = 1;
const DIRECTIONS: [[i8; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
const MAP_SIZE: usize = 71;

type Map = HashSet<(usize, usize)>;

#[derive(Clone, Debug)]
struct Reindeer {
    x: usize,
    y: usize,
    direction_index: u8,
    score: usize,
    finished: bool,
    path: Map,
}

impl Reindeer {
    fn can_go(&self, map: &Map, direction: [i8; 2]) -> bool {
        let x_unsafe = (self.x as isize + direction[0] as isize) as isize;
        let y_unsafe = (self.y as isize + direction[1] as isize) as isize;
        if x_unsafe >= MAP_SIZE as isize
            || x_unsafe < 0
            || y_unsafe >= MAP_SIZE as isize
            || y_unsafe < 0
        {
            return false;
        }

        let x = x_unsafe as usize;
        let y = y_unsafe as usize;

        !map.contains(&(x, y)) && !self.path.contains(&(x, y))
    }
    fn can_go_left(&self, map: &Map) -> bool {
        let direction = DIRECTIONS[self.get_direction(-1) as usize];

        self.can_go(map, direction)
    }
    fn can_go_forward(&self, map: &Map) -> bool {
        let direction = DIRECTIONS[self.direction_index as usize];

        self.can_go(map, direction)
    }
    fn can_go_right(&self, map: &Map) -> bool {
        let direction = DIRECTIONS[self.get_direction(1) as usize];

        self.can_go(map, direction)
    }

    fn get_direction(&self, index_dir: i8) -> u8 {
        ((self.direction_index as isize + index_dir as isize).rem_euclid(DIRECTIONS.len() as isize))
            as u8
    }

    fn turn(&mut self, index_dir: i8) {
        self.score += TURN_PRICE as usize;

        self.direction_index = self.get_direction(index_dir);
    }

    fn go_left(&mut self) {
        self.turn(-1);
        self.go_forward();
    }

    fn go_forward(&mut self) {
        self.score += FORWARD_PRICE as usize;

        let direction = DIRECTIONS[self.direction_index as usize];

        self.path.insert((self.x, self.y));

        self.x = (self.x as isize + direction[0] as isize) as usize;
        self.y = (self.y as isize + direction[1] as isize) as usize;
    }

    fn go_right(&mut self) {
        self.turn(1);
        self.go_forward();
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}

fn pathfind(map: &Map, reindeer: &Reindeer, exit: [usize; 2]) -> Vec<Reindeer> {
    let mut reindeers = vec![];
    let mut reindeer = reindeer.clone();

    if reindeer.x == exit[0] && reindeer.y == exit[1] {
        reindeer.finish();
        reindeers.push(reindeer);

        return reindeers;
    }

    let can_go_left = reindeer.can_go_left(map);
    let can_go_right = reindeer.can_go_right(map);
    let can_go_forward = reindeer.can_go_forward(map);

    if !can_go_left && !can_go_right && !can_go_forward {
        return reindeers;
    }

    if can_go_forward {
        let mut reindeer = reindeer.clone();

        reindeer.go_forward();
        reindeers.push(reindeer);
    }

    if can_go_left {
        let mut reindeer = reindeer.clone();

        reindeer.go_left();
        reindeers.push(reindeer);
    }

    if can_go_right {
        let mut reindeer = reindeer.clone();

        reindeer.go_right();
        reindeers.push(reindeer);
    }

    reindeers
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mut map = HashSet::new();
    let mut scores_memo: HashMap<(usize, usize, u8), usize> = HashMap::new();
    let start = [0, 0];
    let exit = [MAP_SIZE - 1, MAP_SIZE - 1];

    for (index, line) in input.trim().lines().enumerate() {
        if index == 1023 {
            break;
        }

        let coords = line
            .split(",")
            .map(|coord| coord.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        map.insert((coords[0], coords[1]));
    }
    let mut reindeers = vec![];

    reindeers.push(Reindeer {
        x: start[0],
        y: start[1],
        direction_index: 1,
        score: 0,
        finished: false,
        path: HashSet::new(),
    });

    let mut finished_reindeers: Vec<Reindeer> = vec![];

    while reindeers.len() > 0 {
        let new_reindeers = Arc::new(Mutex::new(vec![]));
        reindeers.iter().for_each(|reindeer| {
            (*new_reindeers.lock().unwrap()).append(&mut pathfind(&map, reindeer, exit));
        });

        let mut finished_reindeers_current = (*new_reindeers.lock().unwrap())
            .iter()
            .filter(|reindeer| reindeer.finished)
            .map(|reindeer| reindeer.clone())
            .collect::<Vec<_>>();

        finished_reindeers.append(&mut finished_reindeers_current);

        reindeers = (*new_reindeers.lock().unwrap())
            .iter()
            .filter(|reindeer| {
                let memo_key = (reindeer.x, reindeer.y, reindeer.direction_index);
                let memo_score = scores_memo.get(&memo_key);
                if let Some(&memo_score) = memo_score {
                    if memo_score <= reindeer.score {
                        return false;
                    }
                }

                scores_memo.insert(memo_key, reindeer.score);
                true
            })
            .map(|reindeer| reindeer.clone())
            .collect::<Vec<_>>();
    }

    let lowest_score = finished_reindeers
        .iter()
        .map(|reindeer| reindeer.score)
        .min()
        .unwrap();

    println!("Least steps: {:?}", lowest_score);
}
