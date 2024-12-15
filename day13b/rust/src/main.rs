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
    // let mut combinations = vec![];
    // let max_clicks_a = target_x / xa;
    //
    // for clicks_a in 0..max_clicks_a {
    //     let xa_value = xa * clicks_a;
    //
    //     // println!("{}/{}", xa_value, target_x);
    //     // println!("{:?}", combinations);
    //
    //     if xa_value > target_x {
    //         continue;
    //     }
    //
    //     if (target_x - xa_value) % xb != 0 {
    //         continue;
    //     }
    //
    //     let clicks_b = (target_x - xa_value) / xb;
    //
    //     if ya * clicks_a + yb * clicks_b == target_y {
    //         combinations.push([clicks_a, clicks_b]);
    //     }
    // }
    //
    // combinations
    //
    //
    //
    // let b = (target_y * xa - ya * target_x) / (yb * xa - ya * xb);
    // let a = (target_x - xb * b) / xa;

    let mut combinations = vec![];
    let gcd_ab = gcd(xa, xb);

    // Iterate over possible values of m (related to b)
    let max_m = target_x / xb;

    for m in (0..=max_m).step_by(gcd_ab) {
        // Calculate potential n value
        let x_remainder = target_x - m * xb;
        if x_remainder % xa == 0 {
            let n = x_remainder / xa;
            if ya * n + yb * m == target_y {
                combinations.push([n, m]);
            }
        }
    }

    combinations
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
