use std::{collections::HashSet, fs};

const INPUT_LOCATION: &str = "./input.txt";
const POSSIBLE_DIRECTIONS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
const EMPTY_PLACE: char = 'x';

type Map = Vec<Vec<char>>;
type Side = [isize; 4];

#[derive(Clone, Debug)]
struct Region {
    area: usize,
    sides: HashSet<Side>,
    char: char,
}

fn pathfind(map: &mut Map, [x_curr, y_curr]: &[isize; 2], region: &Region) -> Region {
    if map[*y_curr as usize][*x_curr as usize] != region.char {
        return region.clone();
    }

    let mut new_region = region.clone();
    let checked_char = *new_region
        .char
        .to_lowercase()
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    map[*y_curr as usize][*x_curr as usize] = checked_char;
    new_region.area += 1;

    for [y_dir, x_dir] in POSSIBLE_DIRECTIONS {
        let y_loc = y_curr + y_dir as isize;
        let x_loc = x_curr + x_dir as isize;

        if y_loc < 0 || x_loc < 0 {
            new_region
                .sides
                .insert([*x_curr, *y_curr, x_dir as isize, y_dir as isize]);
            continue;
        }

        let y = map.get(y_loc as usize);
        if let None = y {
            new_region
                .sides
                .insert([*x_curr, *y_curr, x_dir as isize, y_dir as isize]);
            continue;
        }
        let y = y.unwrap();

        let x = y.get(x_loc as usize);
        if let None = x {
            new_region
                .sides
                .insert([*x_curr, *y_curr, x_dir as isize, y_dir as isize]);
            continue;
        }

        let x = x.unwrap().clone();

        if x == region.char {
            let next_region = pathfind(map, &[x_loc, y_loc], &new_region);
            new_region = next_region;
            continue;
        }

        if x == checked_char {
            continue;
        }

        if y_loc != *y_curr || x_loc != *x_curr {
            new_region
                .sides
                .insert([*x_curr, *y_curr, x_dir as isize, y_dir as isize]);
        }
    }

    new_region
}

fn count_sides(region: &Region) -> usize {
    let mut region = region.clone();
    let mut sides_count = 0;

    while region.sides.len() != 0 {
        let elem = region.sides.iter().next().unwrap().clone();
        region.sides.remove(&elem);
        sides_count += 1;

        let direction = match [elem[2], elem[3]] {
            [1, 0] => [0, 1],
            [-1, 0] => [0, 1],
            _ => [1, 0],
        };

        let direction2 = [direction[0] * -1, direction[1] * -1];

        let mut finish_up = false;
        let mut up_loc = [elem[0], elem[1]];
        let mut finish_down = false;
        let mut down_loc = [elem[0], elem[1]];

        while !finish_up {
            up_loc = [up_loc[0] + direction[0], up_loc[1] + direction[1]];
            let next_item = [up_loc[0], up_loc[1], elem[2], elem[3]];

            if !region.sides.contains(&next_item) {
                finish_up = true;
                continue;
            };

            region.sides.remove(&next_item);
        }

        while !finish_down {
            down_loc = [down_loc[0] + direction2[0], down_loc[1] + direction2[1]];
            let next_item = [down_loc[0], down_loc[1], elem[2], elem[3]];

            if !region.sides.contains(&next_item) {
                finish_down = true;
                continue;
            };

            region.sides.remove(&next_item);
        }
    }

    sides_count
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut new_map = map.clone();
    let mut regions: Vec<Region> = vec![];

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            if *value == EMPTY_PLACE {
                return;
            }
            let region = pathfind(
                &mut new_map,
                &[x as isize, y as isize],
                &Region {
                    area: 0,
                    sides: HashSet::new(),
                    char: *value,
                },
            );

            regions.push(region);
        })
    });

    let regions = regions
        .iter()
        .filter(|region| region.area > 0)
        .collect::<Vec<_>>();

    let sum = regions
        .iter()
        .fold(0, |acc, region| acc + count_sides(region) * region.area);

    println!("Sum: {:#?}", sum);
}
