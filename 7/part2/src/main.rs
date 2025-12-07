use std::{
    collections::HashMap,
    io::{self, BufRead},
};

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

fn cache_or_calc(
    column: usize,
    splitters: &[Vec<bool>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(splitters.len(), column)) {
        *count
    } else {
        let count = count_splits_from(column, splitters, cache);
        cache.insert((splitters.len(), column), count);
        count
    }
}

fn count_splits_from(
    column: usize,
    splitters: &[Vec<bool>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if splitters.is_empty() {
        return 1;
    }
    let remaining_lines = splitters.split_first().unwrap().1;
    if splitters.first().unwrap()[column] {
        let left_branch = cache_or_calc(column - 1, remaining_lines, cache);
        let right_branch = cache_or_calc(column + 1, remaining_lines, cache);
        left_branch + right_branch
    } else {
        count_splits_from(column, remaining_lines, cache)
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
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let split_count =
        count_splits_from(start_column, splitters.split_first().unwrap().1, &mut cache);
    println!("{}", split_count);
}
