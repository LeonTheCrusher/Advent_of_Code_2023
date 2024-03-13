use regex::Regex;
use std::convert::TryInto;
use std::fs::read_to_string;

struct Schematic {
    data: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Numbers {
    number: i32,
    start_x: i32,
    end_x: i32,
    y: i32,
    used: bool,
}

struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let content: String = read_to_string("schematic.txt").unwrap();

    let schematics: Schematic = Schematic {
        data: schematic_to_array(&content),
    };

    let mut numbers: Vec<Numbers> = Vec::new();

    let symbol_coords: Vec<(i32, i32)> = extract_coords(&content);

    let mut number_coords: Vec<(i32, i32)> = Vec::new();

    for i in 0..symbol_coords.len() {
        for j in -1..=1 {
            for k in -1..=1 {
                if j == 0 && k == 0 {
                    continue;
                }
                if schematics.data[(symbol_coords[i].0 + j) as usize]
                    [(symbol_coords[i].1 + k) as usize]
                    .is_ascii_digit()
                {
                    number_coords.push((symbol_coords[i].0 + j, symbol_coords[i].1 + k));
                }
            }
        }
    }

    for (col, line) in content.lines().enumerate() {
        let mut current_number = String::new();
        for (row, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                current_number.push(ch);
            } else if !current_number.is_empty() {
                let number: Numbers = Numbers {
                    number: current_number.parse().unwrap(),
                    start_x: (row as i32) - (current_number.len() as i32),
                    end_x: (row as i32) - 1,
                    y: col as i32,
                    used: false,
                };
                numbers.push(number);
                current_number.clear();
            }
        }

        if !current_number.is_empty() {
            let number: Numbers = Numbers {
                number: current_number.parse().unwrap(),
                start_x: (line.len() as i32) - (current_number.len() as i32),
                end_x: (line.len() as i32) - 1,
                y: col as i32,
                used: false,
            };
            numbers.push(number);
            current_number.clear();
        }
    }

    let mut sum = 0;

    for i in 0..numbers.len() {
        for j in 0..number_coords.len() {
            if j < number_coords.len() && i < numbers.len() {
                if number_coords[j].0 == numbers[i].y
                    && numbers[i].start_x <= number_coords[j].1
                    && numbers[i].end_x >= number_coords[j].1
                    && numbers[i].used == false
                {
                    sum += numbers[i].number;
                    numbers[i].used = true;
                    break;
                }
            }
        }
    }

    println!("{sum}");
}

fn schematic_to_array(input: &String) -> Vec<Vec<char>> {
    let mut schematics: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut current_row: Vec<char> = Vec::new();

        for symbol in line.chars() {
            current_row.push(symbol);
        }

        schematics.push(current_row);
    }

    return schematics;
}

fn extract_coords(input: &String) -> Vec<(i32, i32)> {
    let check_symbol = Regex::new(r"[^.\d]").unwrap();

    let mut symbol_coords: Vec<(i32, i32)> = Vec::new();

    for (col_idx, line) in input.lines().enumerate() {
        for (row_idx, ch) in line.chars().enumerate() {
            if check_symbol.is_match(&ch.to_string()) {
                symbol_coords.push((col_idx.try_into().unwrap(), row_idx.try_into().unwrap()));
            }
        }
    }

    return symbol_coords;
}
