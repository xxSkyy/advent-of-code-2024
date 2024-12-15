use regex::Regex;
use std::fs;

const INPUT_LOCATION: &str = "./input.txt";
const BUTTON_A_PRICE: usize = 3;
const BUTTON_B_PRICE: usize = 1;

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

fn get_combinations(step_a: usize, step_b: usize, target: usize) -> Vec<Combination> {
    let mut combinations = vec![];
    let mut clicks_a = 0;
    let mut xa_value = 0;

    while xa_value < target {
        xa_value = step_a * clicks_a;

        if xa_value > target {
            continue;
        }

        if (target - xa_value) % step_b * clicks_a != 0 {
            clicks_a += 1;
            continue;
        }

        let clicks_b = (target - xa_value) / step_b;

        combinations.push([clicks_a, clicks_b]);

        clicks_a += 1;
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
                prize,
            }
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    machines.iter().for_each(|machine| {
        let results_x = get_combinations(machine.button_a.x, machine.button_b.x, machine.prize.x);
        let results_y = get_combinations(machine.button_a.y, machine.button_b.y, machine.prize.y);

        let mut common = results_x
            .iter()
            .filter(|result_x| results_y.contains(&result_x))
            .map(|result| result[0] * BUTTON_A_PRICE + result[1] * BUTTON_B_PRICE)
            .collect::<Vec<_>>();

        if common.len() == 0 {
            return;
        }

        // println!("{:?}", common);

        common.sort();

        sum += common.first().unwrap();
    });

    println!("Sum: {:#?}", sum);
}
