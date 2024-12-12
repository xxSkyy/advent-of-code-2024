use std::fs;

const INPUT_LOCATION: &str = "./input.txt";
const POSSIBLE_DIRECTIONS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
const EMPTY_PLACE: char = 'x';

type Map = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    char: char,
}

fn pathfind(map: &mut Map, [x_curr, y_curr]: &[isize; 2], region: &Region) -> Region {
    if map[*y_curr as usize][*x_curr as usize] != region.char {
        return *region;
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
            new_region.perimeter += 1;
            continue;
        }

        let y = map.get(y_loc as usize);
        if let None = y {
            new_region.perimeter += 1;
            continue;
        }
        let y = y.unwrap();

        let x = y.get(x_loc as usize);
        if let None = x {
            new_region.perimeter += 1;
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

        new_region.perimeter += 1;
    }

    new_region
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
                    perimeter: 0,
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
        .fold(0, |acc, region| acc + region.area * region.perimeter);

    println!("Sum: {}", sum);
}
