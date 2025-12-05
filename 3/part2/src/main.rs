use std::{
    cmp::max,
    collections::HashMap,
    io::{self, BufRead},
};

fn to_joltages(line: String) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect()
}

fn max_in_slice(
    starting_at: usize,
    digits: u32,
    js: &[u64],
    cache: &mut HashMap<(usize, u32), u64>,
) -> u64 {
    // println!(
    //     "starting_at {} digits = {} ({:?})",
    //     starting_at,
    //     digits,
    //     &js[starting_at..]
    // );

    if let Some(precalced) = cache.get(&(starting_at, digits)) {
        // println!("  *> found {}", precalced);
        return *precalced;
    }

    if digits == 1 {
        // println!("  *> single digits...");
        let local_max = *js[starting_at..].iter().max().unwrap();
        cache.insert((starting_at, digits), local_max);
        return local_max;
    }

    let mut maxes: Vec<u64> = vec![];
    for i in starting_at..(js.len() - digits as usize + 1) {
        let first = js[i];
        let m = max_in_slice(i + 1, digits - 1, js, cache);
        let local_max = first * 10_u64.pow(digits - 1) + m;
        maxes.push(local_max);
    }
    let max = *maxes.iter().max().unwrap();
    cache.insert((starting_at, digits), max);
    max
}

fn to_max_joltage(digits: u32, js: &[u64]) -> u64 {
    let mut m = 0;
    let mut cache: HashMap<(usize, u32), u64> = HashMap::new();
    for i in 0..(js.len() - digits as usize + 1) {
        m = max(m, max_in_slice(i, digits, js, &mut cache));
    }
    m
}

fn main() {
    let joltages: Vec<Vec<u64>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_joltages)
        .collect();
    let maxes: Vec<u64> = joltages.iter().map(|js| to_max_joltage(12, js)).collect();
    let sum: u64 = maxes.iter().sum();
    println!("{}", sum);
}
