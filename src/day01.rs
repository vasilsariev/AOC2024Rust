use std::{collections::HashMap, fs};

pub fn solve() -> (String, String) {
    let contents = fs::read_to_string("inputs/day01.txt").expect("No such file");
    let (first_id_list, second_id_list) = organize(&contents);

    let distance = find_distance(&first_id_list, &second_id_list);
    let duplicates = calculate_duplicates(&first_id_list, &second_id_list);

    (distance.to_string(), duplicates.to_string())
}

fn calculate_duplicates(first_id_list: &Vec<i32>, second_id_list: &Vec<i32>) -> i32 {
    let mut duplicates_map: HashMap<i32, i32> = HashMap::new();
    for num in second_id_list {
        let output = duplicates_map.entry(*num).or_default();
        *output += 1;
    }
    let mut score = 0;
    for num in first_id_list {
        if let Some(count) = duplicates_map.get(&num) {
            score += num * count;
        }
    }
    score
}

fn organize(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut first_id_list: Vec<i32> = Vec::new();
    let mut second_id_list: Vec<i32> = Vec::new();

    let numbers = split_to_numbers(input);

    for i in 0..numbers.len() {
        if i % 2 == 0 {
            first_id_list.push(numbers[i]);
        } else {
            second_id_list.push(numbers[i]);
        }
    }
    first_id_list.sort();
    second_id_list.sort();
    (first_id_list, second_id_list)
}

fn find_distance(first_id_list: &Vec<i32>, second_id_list: &Vec<i32>) -> i32 {
    let mut distance = 0;

    for i in 0..first_id_list.len() {
        distance += (first_id_list[i] - second_id_list[i]).abs();
    }
    distance
}

fn split_to_numbers(input: &String) -> Vec<i32> {
    return input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
}
