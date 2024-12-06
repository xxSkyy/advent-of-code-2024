use std::{collections::HashSet, fs};

const INPUT_LOCATION: &str = "./input.txt";

fn is_out_of_bounds(map_size: isize, pos: [isize; 2]) -> bool {
    pos[0] < 0 || pos[1] < 0 || pos[0] >= map_size || pos[1] >= map_size
}

fn will_guard_loop(map: Vec<Vec<String>>, init_pos: [isize; 2], init_dir: [isize; 2]) -> bool {
    let map_size: isize = map.len().try_into().unwrap();

    let mut guard_pos = init_pos.clone();
    let mut guard_dir = init_dir.clone();

    let mut is_guard_on_map = true;
    let mut is_guard_looped = false;

    let mut position_history: Vec<[isize; 4]> = vec![];

    while is_guard_on_map && !is_guard_looped {
        let next_pos = [guard_pos[0] + guard_dir[0], guard_pos[1] + guard_dir[1]];

        if is_out_of_bounds(map_size, next_pos) {
            is_guard_on_map = false;
            continue;
        }

        if map[usize::try_from(next_pos[0]).unwrap()][usize::try_from(next_pos[1]).unwrap()]
            == "#".to_string()
        {
            guard_dir[0] *= -1;
            guard_dir.reverse();

            continue;
        }

        guard_pos = next_pos;

        let guard_data = [guard_pos[0], guard_pos[1], guard_dir[0], guard_dir[1]];

        if position_history.contains(&guard_data) {
            is_guard_looped = true;
            continue;
        }

        position_history.push(guard_data);
    }

    is_guard_looped
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let init_map: Vec<Vec<String>> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let map_size: isize = init_map.len().try_into().unwrap();

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

        if init_map[usize::try_from(next_pos[0]).unwrap()][usize::try_from(next_pos[1]).unwrap()]
            == "#"
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
        .collect::<HashSet<_>>()
        .iter()
        .map(|position| **position)
        .collect::<Vec<[isize; 2]>>();

    let guard_pos: [isize; 2] = [guard_index / map_size, guard_index % map_size];
    let guard_dir: [isize; 2] = [-1, 0];

    let mut obstacle_positions = 0;

    for position in unqiue_positions {
        let mut map = init_map.clone();
        map[usize::try_from(position[0]).unwrap()][usize::try_from(position[1]).unwrap()] =
            "#".to_string();

        if will_guard_loop(map, guard_pos, guard_dir) {
            obstacle_positions += 1;
        }
    }

    println!("Obstacle count: {}", obstacle_positions);
}
