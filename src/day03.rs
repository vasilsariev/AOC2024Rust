use regex::Regex;
use std::fs;

pub fn solve() -> (String, String) {
    let contents = fs::read_to_string("inputs/day03.txt").expect("No such file");
    let answer1 = multiply(&contents);
    let parts = divide_string(&contents);
    let mut answer2 = 0;
    for part in parts {
        answer2 += multiply(part);
    }
    (answer1.to_string(), answer2.to_string())
}

fn divide_string(input: &String) -> Vec<&str> {
    let mut result = Vec::new();
    let mut pos = 0;
    let mut enabled = true;
    while pos < input.len() {
        if enabled {
            if let Some(dont_pos) = input[pos..].find("don't()") {
                let absolute_dont_pos = pos + dont_pos;
                result.push(&input[pos..absolute_dont_pos]);
                pos = absolute_dont_pos + 7;
                enabled = false;
            } else {
                result.push(&input[pos..]);
                break;
            }
        } else {
            if let Some(do_pos) = input[pos..].find("do()") {
                pos = pos + do_pos + 4;
                enabled = true;
            } else {
                break;
            }
        }
    }
    result
}

fn multiply(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    let mut product = 0;
    for task in re.captures_iter(input) {
        let x: i32 = task[1].parse().unwrap();
        let y: i32 = task[2].parse().unwrap();
        product += x * y;
    }
    product
}
