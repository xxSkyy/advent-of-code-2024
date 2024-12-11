use std::{collections::HashSet, fs};

const INPUT_LOCATION: &str = "./input.txt";

const LOWEST: u8 = 0;
const HIGHEST: u8 = 9;

const POSSIBLE_DIRECTIONS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

type Map = Vec<Vec<u8>>;

fn pathfind(map: &Map, [x_curr, y_curr]: &[isize; 2], current_number: &u8) -> Vec<[usize; 2]> {
    let mut sum: Vec<[usize; 2]> = vec![];

    for [y_dir, x_dir] in POSSIBLE_DIRECTIONS {
        let y_loc = y_curr + y_dir as isize;
        let x_loc = x_curr + x_dir as isize;

        let desired_number = current_number + 1;

        if y_loc < 0 || x_loc < 0 {
            continue;
        }

        let y = map.get(y_loc as usize);
        if let None = y {
            continue;
        }
        let y = y.unwrap();

        let x = y.get(x_loc as usize);
        if let None = x {
            continue;
        }
        let x = x.unwrap();

        if x == &desired_number {
            if x == &HIGHEST {
                // println!("FINISHED ON VAL {} || {} {}", x, x_loc, y_loc);
                sum.push([x_loc as usize, y_loc as usize]);
                continue;
            }
            // println!("{} {} -> {} {}", x_curr, y_curr, x_loc, y_loc);
            sum.append(&mut pathfind(&map, &[x_loc, y_loc], &desired_number));
        }
    }

    sum
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let map: Map = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap_or(0) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut lowest_coords: Vec<[isize; 2]> = vec![];

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            if *value == LOWEST {
                lowest_coords.push([x as isize, y as isize]);
            }
        })
    });

    let mut sum: usize = 0;

    lowest_coords.iter().for_each(|cords| {
        let summits = pathfind(&map, cords, &LOWEST);
        let unique_summits = summits
            .into_iter()
            .map(|[x, y]| format!("{},{}", x, y))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        sum += unique_summits;
    });

    println!("SUM: {}", sum);
}
