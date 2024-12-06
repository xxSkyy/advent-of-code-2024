use std::{collections::HashSet, fs};

const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let map: Vec<Vec<String>> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let map_size: isize = map.len().try_into().unwrap();

    let guard_index: isize = input
        .chars()
        .filter(|char| *char != '\n')
        .position(|char| char == '^')
        .unwrap()
        .try_into()
        .unwrap();
    let mut guard_pos: [isize; 2] = [guard_index / map_size, guard_index % map_size];
    let mut guard_dir: [isize; 2] = [-1, 0];

    let mut is_guard_on_map = true;

    let mut positions_trace: Vec<[isize; 2]> = vec![guard_pos.clone()];

    while is_guard_on_map {
        let next_pos = [guard_pos[0] + guard_dir[0], guard_pos[1] + guard_dir[1]];
        if next_pos[0] < 0 || next_pos[1] < 0 || next_pos[0] >= map_size || next_pos[1] >= map_size
        {
            is_guard_on_map = false;
            continue;
        }

        if map[usize::try_from(next_pos[0]).unwrap()][usize::try_from(next_pos[1]).unwrap()] == "#"
        {
            if guard_dir[0] != 0 {
                guard_dir[0] *= -1
            };
            guard_dir.reverse();
            continue;
        }

        guard_pos = next_pos;

        positions_trace.push(guard_pos);
    }

    let unqiue_positions = positions_trace
        .iter()
        .map(|position| format!("{},{}", position[0], position[1]))
        .collect::<HashSet<_>>()
        .len();

    println!("Unique positions: {}", unqiue_positions);
}
