use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use std::str;

pub fn solve(path: &str) {
    solve_one(path);
    solve_two(path);
}

fn solve_one(path: &str) {
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd).lines();
    let numbers: Vec<String> = reader.filter_map(|line| 
        match line {
            Ok(l) => Some(l),
            Err(_) => None,
        } 
    ).collect();
    let number_len = numbers[0].len();
    // map of pos => (0's count, 1's count)
    let mut db: HashMap<i32, (i32, i32)> = HashMap::new();
    for number in numbers {
        for i in 0..number_len as i32 {
            let n = number.chars().nth(i as usize).unwrap();
            match db.get(&i) {
                Some(v) => {
                    if n == '0' {
                        db.insert(i, (v.0+1, v.1));
                    } else {
                        db.insert(i, (v.0, v.1+1));
                    }
                },
                None => {
                    if n == '0' {
                        db.insert(i, (1, 0));
                    } else {
                        db.insert(i, (0, 1));
                    }
                }
            } 
        } 
    }
    let mut gamma_rate_str = String::new();
    let mut epsilon_rate_str = String::new();
    for i in 0..number_len {
        let v = db.get(&(i as i32)).unwrap();
        if v.0 > v.1 {
            gamma_rate_str.push('0');
            epsilon_rate_str.push('1');
        } else if v.1 > v.0 {
            epsilon_rate_str.push('0');
            gamma_rate_str.push('1');
        }
    }
    let gamma_rate =  isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_str, 2).unwrap();
    println!("answer to day 3 first part is {}", gamma_rate*epsilon_rate)
    
}

fn solve_two(path: &str) {

}
