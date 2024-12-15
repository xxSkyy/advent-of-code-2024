use rayon::prelude::*;
use regex::Regex;
use std::{
    fs,
    sync::{Arc, Mutex},
};

const INPUT_LOCATION: &str = "./input.txt";
const BUTTON_A_PRICE: usize = 3;
const BUTTON_B_PRICE: usize = 1;
// const PRIZE_MULTIPLY: usize = 1;
const PRIZE_MULTIPLY: usize = 10000000000000;

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

type Target = Button;

type Combination = [usize; 2];

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Target,
}

fn get_machine_line(line: &str) -> Button {
    let machine_regex = Regex::new(r"([0-9]+).*Y.([0-9]+)").expect("Regex issue");

    let data = machine_regex.captures_iter(line).next().unwrap();

    Button {
        x: data[1].parse::<usize>().unwrap(),
        y: data[2].parse::<usize>().unwrap(),
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_combinations(
    xa: usize,
    xb: usize,
    ya: usize,
    yb: usize,
    target_x: usize,
    target_y: usize,
) -> Vec<Combination> {
    let x: f64 = (target_y as f64 * xa as f64 - ya as f64 * target_x as f64)
        / (yb as f64 * xa as f64 - ya as f64 * xb as f64);
    let y: f64 = (target_x as f64 - xb as f64 * x) / xa as f64;

    if y.fract() != 0.0 || x.fract() != 0.0 {
        return vec![];
    }

    println!("{}, {}", y, x);

    vec![[y as usize, x as usize]]
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let machines = input
        .trim()
        .split("\n\n")
        .map(|machine_txt| {
            let machine_txt = machine_txt.lines().collect::<Vec<_>>();
            let button_a = get_machine_line(machine_txt[0]);
            let button_b = get_machine_line(machine_txt[1]);
            let prize = get_machine_line(machine_txt[2]);

            Machine {
                button_a,
                button_b,
                prize: Target {
                    x: prize.x + PRIZE_MULTIPLY,
                    y: prize.y + PRIZE_MULTIPLY,
                },
            }
        })
        .collect::<Vec<_>>();

    let sum = Arc::new(Mutex::new(0));

    machines
        .par_iter()
        .enumerate()
        .for_each(|(index, machine)| {
            println!("CALCULATING MACHINE: {}/{}", index, machines.len());
            let results = get_combinations(
                machine.button_a.x,
                machine.button_b.x,
                machine.button_a.y,
                machine.button_b.y,
                machine.prize.x,
                machine.prize.y,
            );

            let mut common = results
                .iter()
                .map(|result| result[0] * BUTTON_A_PRICE + result[1] * BUTTON_B_PRICE)
                .collect::<Vec<_>>();

            if common.len() == 0 {
                return;
            }

            common.sort();

            *sum.lock().unwrap() += common.first().unwrap();
        });

    println!("Sum: {:#?}", sum.lock().unwrap());
}
