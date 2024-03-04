use std::convert::TryInto;
use std::fs::read_to_string;

// TODO: add a struct for each number in the sequence, save the number i32, and the start and end coordinates as (i32,i32)

fn main() {
    let content: String = read_to_string("test.txt").unwrap();

    let mut schematics: Vec<Vec<char>> = Vec::new();

    let symbol_coords = extract_coords(&content);

    let mut number_coords: Vec<(i32, i32)> = Vec::new();

    for line in content.lines() {
        let mut current_row: Vec<char> = Vec::new();

        for symbol in line.chars() {
            current_row.push(symbol);
        }

        schematics.push(current_row);
    }

    // println!("{:?}", schematics[0][1]);

    for i in 0..symbol_coords.len() {
        for j in -1..=1 {
            for k in -1..=1 {
                if j == 0 && k == 0 {
                    continue;
                }
                if schematics[(symbol_coords[i].0 + j) as usize][(symbol_coords[i].1 + k) as usize].is_ascii_digit() {
                    number_coords.push((symbol_coords[i].0 + j, symbol_coords[i].1 + k));
                }
            }
        }
    }    
    
}

fn extract_coords(input: &String) -> Vec<(i32, i32)> {
    let mut symbol_coords: Vec<(i32, i32)> = Vec::new();


    for (col_idx, line) in input.lines().enumerate() {
        for (row_idx, ch) in line.chars().enumerate() {
            if !ch.is_numeric() && ch != '.' {
                symbol_coords.push((col_idx.try_into().unwrap(), row_idx.try_into().unwrap()));
            }
        }
    }

    return symbol_coords;
}