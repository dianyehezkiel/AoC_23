use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents_len = contents.lines().count();

    let mut res_one: u32 = 0;

    let mut cards_count: Vec<u32> = vec![1; contents_len];
    let mut res_two: u32 = 0;

    for (line_idx, line) in contents.lines().enumerate() {
        let card = parse_card(line);
        let match_count = calculate_match(&card);
        if match_count > 0 {
            res_one += 2u32.pow(match_count - 1);
        }

        update_cards_count(&mut cards_count, &line_idx, &match_count);
    }

    res_two += cards_count.iter().sum::<u32>();

    println!("Cards is worth {} point(s)", res_one);
    println!("Total scratchcards is: {}", res_two);
}

#[derive(Debug)]
struct Card {
    w: Vec<u32>, // winning numbers
    a: Vec<u32>, // appeared numbers
}

fn parse_card(line: &str) -> Card {
    let mut card = Card {
        w: vec![],
        a: vec![],
    };

    let data = line.split(":").collect::<Vec<&str>>()[1];

    let w_and_a = data.split("|").collect::<Vec<&str>>();
    let w_str = w_and_a[0].trim();
    let a_str = w_and_a[1].trim();

    for w in w_str.split(" ") {
        if w.is_empty() {
            continue;
        }

        card.w.push(w.parse().unwrap());
    }

    for a in a_str.split(" ") {
        if a.is_empty() {
            continue;
        }

        card.a.push(a.parse().unwrap());
    }

    return card;
}

fn calculate_match(card: &Card) -> u32 {
    let mut w_vec = card.w.clone();
    let mut a_vec = card.a.clone();
    w_vec.sort();
    a_vec.sort();

    let mut match_count = 0u32;
    let mut w_idx = 0usize;
    let mut a_idx = 0usize;

    while a_idx < a_vec.len() {
        if w_idx == w_vec.len() {
            break;
        }

        if a_vec[a_idx] > w_vec[w_idx] {
            w_idx += 1;
            continue;
        }

        if a_vec[a_idx] == w_vec[w_idx] {
            match_count += 1;
        }

        a_idx += 1;
    }

    return match_count;
}

fn update_cards_count(cards_count: &mut Vec<u32>, line_idx: &usize, match_count: &u32) {
    let mc_usize: usize = match_count.clone().try_into().unwrap();
    let maybe_upper_idx = line_idx + 1usize + mc_usize;
    let upper_idx = if cards_count.len() <= maybe_upper_idx {
        cards_count.len()
    } else {
        maybe_upper_idx
    };
    let lower_idx = line_idx + 1;

    for i in lower_idx..upper_idx {
        cards_count[i] += cards_count[*line_idx];
    }
}
