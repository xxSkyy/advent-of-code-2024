use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_LOCATION: &str = "./input.txt";
const EMPTY_CHAR: char = '.';

#[derive(Debug)]
struct Antenna {
    x: isize,
    y: isize,
}

impl Antenna {
    fn get_antinode(&self, antenna: &Antenna, map_size: isize) -> Option<[isize; 2]> {
        let antinode_location = [self.x + (self.x - antenna.x), self.y + (self.y - antenna.y)];
        let range = 0..map_size;

        if !range.contains(&antinode_location[0]) || !range.contains(&antinode_location[1]) {
            return None;
        }

        Some(antinode_location)
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let map: Vec<_> = input.trim().lines().collect();

    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut antinodes: Vec<[isize; 2]> = vec![];
    let map_size: isize = isize::try_from(map[0].len()).unwrap();

    for (y, line) in map.into_iter().enumerate() {
        let y = isize::try_from(y).unwrap();
        for (x, char) in line.chars().enumerate() {
            let x = isize::try_from(x).unwrap();
            if char != EMPTY_CHAR {
                antennas
                    .entry(char)
                    .or_insert(vec![])
                    .push(Antenna { x, y });
            }
        }
    }

    antennas.keys().for_each(|antenna_id| {
        let antennas_list = antennas.get(antenna_id).unwrap();

        antennas_list.into_iter().for_each(|antenna| {
            antennas_list
                .into_iter()
                .filter(|a| a.x != antenna.x && a.y != antenna.y)
                .for_each(|other_antenna| {
                    if let Some(antinode) = antenna.get_antinode(&other_antenna, map_size) {
                        antinodes.push(antinode);
                    }
                });
        });
    });

    let unique_antinodes = antinodes
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|position| **position)
        .collect::<Vec<[isize; 2]>>();

    println!("Sum: {}", unique_antinodes.len());
}
