use std::{env, fs, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_vec = contents.lines().collect::<Vec<&str>>();
    let width = contents_vec[0].len() - 1;
    let height = contents_vec.len() - 1;

    let mut res_one: u32 = 0;
    let mut res_two: u32 = 0;
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();

    for (line_idx, line) in contents_vec.iter().enumerate() {
        let maybe_part_numbers = check_line(*line);

        for maybe_part_number in maybe_part_numbers {
            let coordinates = create_box(&maybe_part_number.left_idx, &maybe_part_number.right_idx, &line_idx, &width, &height);

            for coordinate in coordinates {
                let char = contents_vec[coordinate.y].as_bytes()[coordinate.x] as char;
                let dot = '.';

                if is_symbol(&char) {
                    if &char == &'*' {
                        gears.entry(String::from("x".to_string() + &coordinate.x.to_string() + "y" + &coordinate.y.to_string()))
                            .and_modify(|g| g.push(maybe_part_number.number))
                            .or_insert(vec![maybe_part_number.number]);
                    }
                    res_one += maybe_part_number.number;
                    assert_ne!(char, dot);
                    break;
                } else {
                    assert_eq!(char, dot);
                }
            }
        }
    }

    for (_key, gear) in gears {
        if gear.len() != 2 { continue; }

        res_two += gear[0] * gear[1];
    }

    println!("Sum of all of the part numbers is: {}", res_one);
    println!("Sum of all of the gear ratios is: {}", res_two);
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn is_symbol(char: &char) -> bool {
    if (char == &'.') || (char.is_digit(10)) {
        return false;
    };

    return true;
}

fn create_box(
    left_idx: &usize,
    right_idx: &usize,
    line_idx: &usize,
    width: &usize,
    height: &usize,
) -> Vec<Coordinate> {
    let mut res: Vec<Coordinate> = vec![];

    let is_first_row = line_idx == &0usize;
    let is_last_row = line_idx == height;
    let is_first_col = left_idx == &0usize;
    let is_last_col = right_idx == width;

    if !is_first_row {
        for i in *left_idx..=*right_idx {
            res.push(Coordinate {
                x: i,
                y: line_idx - 1,
            });
        }
    }

    if !is_last_row {
        for i in *left_idx..=*right_idx {
            res.push(Coordinate {
                x: i,
                y: line_idx + 1,
            });
        }
    }

    if !is_first_col {
        if !is_first_row {
            res.push(Coordinate {
                x: left_idx - 1,
                y: line_idx - 1,
            });
        }

        if !is_last_row {
            res.push(Coordinate {
                x: left_idx - 1,
                y: line_idx + 1,
            });
        }

        res.push(Coordinate {
            x: left_idx - 1,
            y: *line_idx,
        });
    }

    if !is_last_col {
        if !is_first_row {
            res.push(Coordinate {
                x: right_idx + 1,
                y: line_idx - 1,
            });
        }

        if !is_last_row {
            res.push(Coordinate {
                x: right_idx + 1,
                y: line_idx + 1,
            });
        }

        res.push(Coordinate {
            x: right_idx + 1,
            y: *line_idx,
        });
    }

    return res;
}

#[derive(Debug)]
struct MaybePartNumber {
    number: u32,
    left_idx: usize,
    right_idx: usize,
}

fn check_line(line: &str) -> Vec<MaybePartNumber> {
    let mut numbers: Vec<MaybePartNumber> = vec![];

    let mut temp_str_number = String::from("");
    let mut temp_left_index = 0usize;
    let w = line.len() - 1;

    for (idx, char) in line.char_indices() {
        if char.is_digit(10) {
            if temp_str_number.is_empty() {
                temp_left_index = idx;
            }

            temp_str_number.push(char);

            if idx == w {
                numbers.push(MaybePartNumber {
                    number: temp_str_number.parse().unwrap(),
                    left_idx: temp_left_index,
                    right_idx: idx,
                });
                temp_str_number = String::from("");
            }
        } else {
            if !temp_str_number.is_empty() {
                numbers.push(MaybePartNumber {
                    number: temp_str_number.parse().unwrap(),
                    left_idx: temp_left_index,
                    right_idx: idx - 1,
                });
                temp_str_number = String::from("");
            }
        }
    }

    return numbers;
}
