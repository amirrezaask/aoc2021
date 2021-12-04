use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Command {
    action: String,
    arg: i32,
}

fn solve_one(path: &str) {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd).lines();
    let commands: Vec<Command> = reader
        .filter_map(|line| {
            if let Ok(s) = line {
                let splitted = s.split(" ").collect::<Vec<&str>>();
                Some(Command {
                    action: splitted[0].to_string(),
                    arg: splitted[1].to_string().parse::<i32>().unwrap(),
                })
            } else {
                None
            }
        })
        .collect();
    let mut h = 0;
    let mut d = 0;
    for command in commands {
        if command.action == "forward" {
            h += command.arg;
        } else if command.action == "down" {
            d += command.arg;
        } else if command.action == "up" {
            d -= command.arg
        }
    }
    println!("answer to day 2 part 2 is {}", d * h);
}
fn solve_two(path: &str) {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd).lines();
    let commands: Vec<Command> = reader
        .filter_map(|line| {
            if let Ok(s) = line {
                let splitted = s.split(" ").collect::<Vec<&str>>();
                Some(Command {
                    action: splitted[0].to_string(),
                    arg: splitted[1].to_string().parse::<i32>().unwrap(),
                })
            } else {
                None
            }
        })
        .collect();
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
    for command in commands {
        if command.action == "forward" {
            h += command.arg;
            d += a * command.arg;
        } else if command.action == "down" {
            a += command.arg;
        } else if command.action == "up" {
            a -= command.arg
        }
    }

    println!("answer to day 2 part 2 is {}", d * h);
}

pub fn solve(path: &str) {
    solve_one(path);
    solve_two(path);
}
