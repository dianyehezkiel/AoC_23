use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let mut res_one: u32 = 0;
    // let res_two: u32 = 0;

    let mut almanac = parse_almanac(&contents);
    sort_almanac_src(&mut almanac);
    let res_one = find_lowest_location_number(&almanac);
    let res_two = find_lowest_location_number_2(&almanac);

    println!("The lowest location number is: {}", res_one);
    println!("Sum of the power of the sets is: {}", res_two);
}

const KEY: [&str; 8] = [
    "seeds",
    "seed_to_soil",
    "soil_to_fertilizer",
    "fertilizer_to_water",
    "water_to_light",
    "light_to_temperature",
    "temperature_to_humidity",
    "humidity_to_location",
];

fn parse_number(line: &str) -> Vec<u32> {
    return line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let nums_str = line.split(":").collect::<Vec<&str>>()[1];

    return parse_number(nums_str);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Vec<u32>>,
    soil_to_fertilizer: Vec<Vec<u32>>,
    fertilizer_to_water: Vec<Vec<u32>>,
    water_to_light: Vec<Vec<u32>>,
    light_to_temperature: Vec<Vec<u32>>,
    temperature_to_humidity: Vec<Vec<u32>>,
    humidity_to_location: Vec<Vec<u32>>,
}

fn push_data(almanac: &mut Almanac, key: &str, new_value: Vec<u32>) {
    match key {
        "seeds" => almanac.seeds = new_value,
        "seed_to_soil" => almanac.seed_to_soil.push(new_value),
        "soil_to_fertilizer" => almanac.soil_to_fertilizer.push(new_value),
        "fertilizer_to_water" => almanac.fertilizer_to_water.push(new_value),
        "water_to_light" => almanac.water_to_light.push(new_value),
        "light_to_temperature" => almanac.light_to_temperature.push(new_value),
        "temperature_to_humidity" => almanac.temperature_to_humidity.push(new_value),
        "humidity_to_location" => almanac.humidity_to_location.push(new_value),
        _ => panic!("unknown field!"),
    }
}

fn parse_almanac(lines: &String) -> Almanac {
    let mut almanac = Almanac {
        seeds: vec![],
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let mut key_idx = 0usize;

    for (idx, line) in lines.lines().enumerate() {
        if line.is_empty() {
            key_idx += 1;
            continue;
        }

        if idx == 0 {
            push_data(&mut almanac, KEY[key_idx], parse_seeds(line))
        }

        if line.chars().nth(0).unwrap().is_digit(10) {
            push_data(&mut almanac, KEY[key_idx], parse_number(line))
        }
    }

    return almanac;
}

fn sort_almanac_src(almanac: &mut Almanac) {
    almanac.seeds.sort();
    almanac.seed_to_soil.sort_by(|a, b| a[1].cmp(&b[1]));
    almanac.soil_to_fertilizer.sort_by(|a, b| a[1].cmp(&b[1]));
    almanac.fertilizer_to_water.sort_by(|a, b| a[1].cmp(&b[1]));
    almanac.water_to_light.sort_by(|a, b| a[1].cmp(&b[1]));
    almanac.light_to_temperature.sort_by(|a, b| a[1].cmp(&b[1]));
    almanac
        .temperature_to_humidity
        .sort_by(|a, b| a[1].cmp(&b[1]));
    almanac.humidity_to_location.sort_by(|a, b| a[1].cmp(&b[1]));
}

fn map_src_des(src: &u32, map: &Vec<Vec<u32>>) -> u32 {
    let mut result = *src;

    for m in map {
        if m[1] > *src {
            break;
        };

        if (m[1] <= *src) && ((m[1] + (m[2] - 1)) >= *src) {
            result = m[0] + (src - m[1]);
        }
    }

    return result;
}

fn find_lowest_location_number(almanac: &Almanac) -> u32 {
    let mut result = u32::MAX;

    for seed in &almanac.seeds {
        let soil = map_src_des(seed, &almanac.seed_to_soil);
        let fertilizer = map_src_des(&soil, &almanac.soil_to_fertilizer);
        let water = map_src_des(&fertilizer, &almanac.fertilizer_to_water);
        let light = map_src_des(&water, &almanac.water_to_light);
        let temperature = map_src_des(&light, &almanac.light_to_temperature);
        let humidity = map_src_des(&temperature, &almanac.temperature_to_humidity);
        let location = map_src_des(&humidity, &almanac.humidity_to_location);

        if location < result {
            result = location;
        }
    }

    return result;
}

fn map_src_des_range(ranges: &Vec<Vec<u32>>, map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut ranges_mut = ranges.clone();
    let mut result = vec![];

    let mut m_idx = 0usize;
    let mut r_idx = 0usize;

    while m_idx < map.len() && r_idx < ranges_mut.len() {
        let lower_r = ranges[r_idx][0];
        let upper_r = ranges[r_idx][1];

        let lower_s = map[m_idx][1];
        let upper_s = map[m_idx][1] + (map[m_idx][2] - 1);

        let lower_d = map[m_idx][0];
        let upper_d = map[m_idx][0] + (map[m_idx][2] - 1);

        if lower_s <= lower_r {
            if upper_s >= upper_r {
                result.push(vec![lower_d + (lower_r - lower_s), upper_d - (upper_s - upper_r)]);
                r_idx += 1;
                m_idx += 1;
            } 
            if upper_s < upper_r {
                if (lower_r <= upper_s) {
                    result.push(vec![lower_d + (lower_r - lower_s), upper_d]);
                    ranges_mut[r_idx] = vec![upper_s + 1, upper_r];
                    m_idx += 1;
                }
            }

        }
        
        if lower_s > lower_r {
            if lower_s > upper_r {
                result.push(vec![lower_r, upper_r]);
                r_idx += 1;
            } else {
                result.push(vec![lower_r, lower_s - 1]);
                ranges_mut[r_idx] = vec![lower_s, upper_r];
            }
        }
    }

    return result;
}

fn find_lowest_location_number_2(almanac: &Almanac) -> u32 {
    let mut result = u32::MAX;

    let mut idx = 0usize;

    while idx < almanac.seeds.len() {
        for seed in almanac.seeds[idx]..(almanac.seeds[idx] + almanac.seeds[idx + 1]) {
            let soil = map_src_des(&seed, &almanac.seed_to_soil);
            let fertilizer = map_src_des(&soil, &almanac.soil_to_fertilizer);
            let water = map_src_des(&fertilizer, &almanac.fertilizer_to_water);
            let light = map_src_des(&water, &almanac.water_to_light);
            let temperature = map_src_des(&light, &almanac.light_to_temperature);
            let humidity = map_src_des(&temperature, &almanac.temperature_to_humidity);
            let location = map_src_des(&humidity, &almanac.humidity_to_location);

            if location < result {
                result = location;
            }

            println!("{location}");
        }

        println!("============================================={idx}=============================================");

        idx += 2;
    }

    return result;
}
