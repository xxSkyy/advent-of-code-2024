use std::fs;

const INPUT_LOCATION: &str = "./input.txt";

fn rotate_vec(input_vec: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let size = input_vec.len();

    let mut rotated_vec: Vec<Vec<String>> = vec![vec!["".to_string(); size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated_vec[size - col - 1][row] = input_vec[row][col].clone();
        }
    }

    rotated_vec
}

fn is_coord_letter(
    input_vec: &Vec<Vec<String>>,
    x: usize,
    y: usize,
    letter: &str,
) -> Result<(), ()> {
    let y_pos = input_vec.get(y);
    if y_pos.is_none() {
        return Err(());
    }

    let x_pos = y_pos.unwrap().get(x);
    if x_pos.is_none() {
        return Err(());
    }

    if *x_pos.unwrap() != letter {
        return Err(());
    }

    Ok(())
}

fn count_mases(input_vec: &Vec<Vec<String>>) -> usize {
    let size = input_vec.len();
    let mut mases: usize = 0;

    for x in 0..size {
        for y in 0..size {
            if let Err(_) = is_coord_letter(input_vec, x, y, "M") {
                continue;
            }

            if let Err(_) = is_coord_letter(input_vec, x, y + 2, "M") {
                continue;
            }

            if let Err(_) = is_coord_letter(input_vec, x + 1, y + 1, "A") {
                continue;
            }

            if let Err(_) = is_coord_letter(input_vec, x + 2, y, "S") {
                continue;
            }

            if let Err(_) = is_coord_letter(input_vec, x + 2, y + 2, "S") {
                continue;
            }

            mases += 1;
        }
    }

    mases
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mut input_vec: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect();

    let mut mases_count = 0;

    for _ in 0..4 {
        mases_count += count_mases(&input_vec);
        input_vec = rotate_vec(&input_vec);
    }

    println!("Mases: {}", mases_count);
}
