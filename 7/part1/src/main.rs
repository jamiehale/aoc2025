use std::io::{self, BufRead};

fn dump_splitters(splitters: &[Vec<bool>]) {
    for row in splitters.iter() {
        for column in row.iter() {
            if *column {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let start_column = lines.first().unwrap().find("S").unwrap();
    let splitters: Vec<Vec<bool>> = lines
        .split_first()
        .unwrap()
        .1
        .iter()
        .map(|l| l.chars().map(|c| c == '^').collect())
        .collect();
    dump_splitters(&splitters);
    let mut current_row = 1;
    let mut positions: Vec<usize> = vec![start_column];
    let mut split_count = 0;
    loop {
        if current_row >= splitters.len() {
            break;
        }
        println!("Row {}", current_row);
        let mut next_positions: Vec<usize> = vec![];
        for p in positions.iter() {
            if splitters[current_row][*p] {
                println!("  Splitting at {}", *p);
                next_positions.push(p - 1);
                next_positions.push(p + 1);
                split_count += 1;
            } else {
                next_positions.push(*p);
            }
        }
        next_positions.sort();
        next_positions.dedup();
        positions = next_positions;
        current_row += 1;
    }
    println!("{}", split_count);
}
