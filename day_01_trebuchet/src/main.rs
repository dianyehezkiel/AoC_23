use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut res_one: u32 = 0;
    let mut res_two: u32 = 0;

    for line in contents.lines() {
        res_one += get_digits(line);
        let new_str = replace_str_to_digit(line);
        let n = get_digits(&new_str);
        res_two += n;
    }

    println!(
        "The sum of calibration values for first solution is {}",
        &res_one
    );
    println!(
        "The sum of calibration values for second solution is {}",
        &res_two
    );
}

fn get_digits(line: &str) -> u32 {
    if line.is_empty() {
        return 0;
    };

    let mut res: u32 = 0;

    for c in line.chars() {
        if c.is_ascii_digit() {
            res += c.to_digit(10).unwrap_or(0) * 10;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            res += c.to_digit(10).unwrap_or(0);
            break;
        }
    }

    return res;
}

fn replace_str_to_digit(line: &str) -> String {
    if line.is_empty() {
        return "".to_string();
    };

    let number_str: HashMap<&str, &str> = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    let mut res: String = line.to_string();

    for (from, to) in number_str {
        res = res.replace(from, to);
    }

    return res;
}
