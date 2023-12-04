use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let max_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut res_one: u32 = 0;
    let mut res_two: u32 = 0;

    for line in contents.lines() {
        let (id, cubes_vec) = split_id_and_data(line);

        if is_possible(&cubes_vec, &max_cubes) {
            res_one += id;
        }

        let least_cubes = least_possible_cubes(&cubes_vec);

        res_two += least_cubes.power();
    }

    println!("Sum of the IDs of the possible games is: {}", res_one);
    println!("Sum of the power of the sets is: {}", res_two);
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Cubes {
    fn default() -> Cubes {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Cubes {
    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

fn split_id_and_data(line: &str) -> (u32, Vec<Cubes>) {
    let id_and_data_str = line.split(": ").collect::<Vec<&str>>();

    let id = id_and_data_str[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let mut cubes_vec: Vec<Cubes> = Vec::new();

    let data = id_and_data_str[1].split("; ");

    for cubes_data in data {
        let cubes = cubes_data.split(", ");

        let mut c = Cubes {
            ..Default::default()
        };

        for cube in cubes {
            if cube.ends_with("red") {
                let n = cube.split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap();

                c.red = n;
            }

            if cube.ends_with("green") {
                let n = cube.split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap();

                c.green = n;
            }

            if cube.ends_with("blue") {
                let n = cube.split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap();

                c.blue = n;
            }
        }

        cubes_vec.push(c);
    }

    return (id, cubes_vec);
}

fn is_possible(cubes_vec: &Vec<Cubes>, max_cubes: &Cubes) -> bool {
    for cubes in cubes_vec {
        if cubes.red > max_cubes.red || cubes.green > max_cubes.green || cubes.blue > max_cubes.blue {
            return false;
        }
    }

    return true;
}

fn least_possible_cubes(cubes_vec: &Vec<Cubes>) -> Cubes {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for cubes in cubes_vec {
        if cubes.red > max_red { max_red = cubes.red };
        if cubes.green > max_green { max_green = cubes.green };
        if cubes.blue > max_blue { max_blue = cubes.blue };
    }

    return Cubes {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}
