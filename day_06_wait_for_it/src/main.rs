use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut res_one: u32 = 0;

    let mut times: Vec<u32> = vec![];
    let mut distances: Vec<u32> = vec![];
    let mut time = 0;
    let mut distance = 0;

    for (idx, line) in contents.lines().enumerate() {
        if idx == 0 {
            times = parse_vec(line);
            time = parse_num(line);
        }

        if idx == 1 {
            distances = parse_vec(line);
            distance = parse_num(line);
        }
    }

    for idx in 0..times.len() {
        let n_win = calc_n_win(&times[idx], &0u32, &distances[idx]);

        if idx == 0 {
            res_one = n_win;
            continue;
        }

        res_one *= n_win;
    }

    let res_two = calc_n_win_loop(&time, &distance);

    println!("Multiply of number of ways record beaten is: {}", res_one);
    println!("Number of way record beaten is: {}", res_two);
}

fn parse_vec(line: &str)  -> Vec<u32> {
    let str_vec = line.split(":").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();

    let mut data: Vec<u32> = vec![];

    for s in str_vec {
        if s.is_empty() { continue; };

        data.push(s.parse().unwrap());
    }

    return data;
}

fn parse_num(line: &str) -> u64 {
    let str_vec = line.split(":").collect::<Vec<&str>>()[1];

    let mut num_str = String::from("");

    for ch in str_vec.chars() {
        if !ch.is_digit(10) {
            continue;
        }

        num_str.push(ch);
    }

    return num_str.parse().unwrap();
}

fn calc_n_win(time_move: &u32, time_charges: &u32, min_distance: &u32) -> u32 {
    if time_move == &0u32 {
        return 0;
    }

    let distance = time_move * time_charges;

    if &distance <= min_distance {
        return 0 + calc_n_win(&(time_move - 1), &(time_charges + 1), min_distance);
    }

    return 1 + calc_n_win(&(time_move - 1), &(time_charges + 1), min_distance);
}

fn calc_n_win_loop(time: &u64, distance: &u64) -> u32 {
    let mut n_win = 0;

    for t in 1..*time {
        let d = t * (time - t);

        if d > *distance {
            n_win += 1;
        }
    }

    return n_win;
}
