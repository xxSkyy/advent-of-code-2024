use std::{collections::HashSet, fs};

const INPUT_LOCATION: &str = "./input.txt";
const POSSIBLE_DIRECTIONS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

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

fn remove_adjacent(region: &mut Region, first_el: &[isize; 4], loc: [isize; 2], dir: [isize; 2]) {
    let mut finish = false;
    let mut loc = loc.clone();

    while !finish {
        loc = [loc[0] + dir[0], loc[1] + dir[1]];
        let next_item = [loc[0], loc[1], first_el[2], first_el[3]];

        if !region.sides.contains(&next_item) {
            finish = true;
            continue;
        };

        region.sides.remove(&next_item);
    }
}

fn count_sides(region: &Region) -> usize {
    let mut region = region.clone();
    let mut sides_count = 0;

    while region.sides.len() != 0 {
        let elem = region.sides.iter().next().unwrap().clone();
        region.sides.remove(&elem);

        sides_count += 1;

        let dir = match [elem[2], elem[3]] {
            [1, 0] => [0, 1],
            [-1, 0] => [0, 1],
            _ => [1, 0],
        };

        let dir2 = [dir[0] * -1, dir[1] * -1];

        remove_adjacent(&mut region, &elem, [elem[0], elem[1]], dir);
        remove_adjacent(&mut region, &elem, [elem[0], elem[1]], dir2);
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
