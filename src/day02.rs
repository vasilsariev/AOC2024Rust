use std::fs;

pub fn solve() -> (String, String) {
    let contents = fs::read_to_string("inputs/day02.txt").expect("No such file");
    let lines = split_to_lines(&contents);
    let levels = split_to_levels(lines);
    let (answer1, vec_unsafe) = find_safe(levels);
    let answer2 = answer1 + find_safe_special(vec_unsafe);

    return (answer1.to_string(), answer2.to_string());
}

// fn find_safe_reports(input: &String) {}

fn split_to_lines(input: &String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in input.lines() {
        output.push(line.to_string());
    }
    output
}

fn split_to_levels(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();
    for line in input {
        let level: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        output.push(level);
    }
    output
}

fn find_safe(input: Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut safe = 0;
    let mut vec_unsafe: Vec<Vec<i32>> = Vec::new();
    'outer: for level in input {
        let mut is_growing = true;
        let mut is_unsafe = false;
        for n in 1..level.len() {
            let n1 = level[n - 1];
            let n2 = level[n];
            if n == 1 {
                if n2 > n1 {
                    is_growing = true;
                } else if n2 < n1 {
                    is_growing = false;
                } else {
                    is_unsafe = true;
                }
            }
            if is_growing && (n2 - n1 < 1 || n2 - n1 > 3) {
                is_unsafe = true;
            } else if !is_growing && (n1 - n2 < 1 || n1 - n2 > 3) {
                is_unsafe = true;
            }
            if n2 == n1 {
                is_unsafe = true;
            }
            if is_unsafe {
                vec_unsafe.push(level);
                continue 'outer;
            }
        }
        safe += 1;
    }
    (safe, vec_unsafe)
}

fn find_safe_special(input: Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    'outer: for level in input {
        let mut is_growing = true;
        'middle: for n in 0..level.len() {
            let mut new_vec = level.clone();
            new_vec.remove(n);
            for i in 1..new_vec.len() {
                let n1 = new_vec[i - 1];
                let n2 = new_vec[i];
                if i == 1 {
                    if n2 > n1 {
                        is_growing = true;
                    } else if n2 < n1 {
                        is_growing = false;
                    } else {
                        continue 'middle;
                    }
                }
                if is_growing && (n2 - n1 < 1 || n2 - n1 > 3) {
                    continue 'middle;
                } else if !is_growing && (n1 - n2 < 1 || n1 - n2 > 3) {
                    continue 'middle;
                }
                if n2 == n1 {
                    continue 'middle;
                }
            }
            safe += 1;
            continue 'outer;
        }
    }
    safe
}
