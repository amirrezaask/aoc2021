use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(path: &str) {
    solve_part_one(path);
    solve_part_two(path);
}

fn solve_part_one(path: &str) {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd).lines();
    let numbers: Vec<i32> = reader
        .filter_map(|line| {
            if let Ok(s) = line {
                if let Ok(i) = s.parse::<i32>() {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut last = &numbers[0];
    let mut increased = 0;

    for number in numbers.iter().skip(1) {
        if number > last {
            increased += 1;
        }
        last = number
    }
    println!("answer to day 1 part 1 is {}", increased)
}

fn solve_part_two(path: &str) {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd).lines();
    let numbers: Vec<i32> = reader
        .filter_map(|line| {
            if let Ok(s) = line {
                if let Ok(i) = s.parse::<i32>() {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let mut last = 0;
    let mut increased = 0;

    for i in 0..numbers.len() {
        if i + 2 >= numbers.len() {
            break;
        }
        let x = &numbers[i];
        let y = &numbers[i + 1];
        let z = &numbers[i + 2];
        if i == 0 {
            last = x + y + z;
            continue;
        }
        if last < x + y + z {
            increased += 1;
        }
        last = x + y + z;
    }

    println!(
        "answer to day 1 part 2 is {}",
        increased
    );
}
