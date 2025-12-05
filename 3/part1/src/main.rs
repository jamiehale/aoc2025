use std::{
    cmp::max,
    io::{self, BufRead},
};

fn to_joltages(line: String) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect()
}

fn max_in_slice(digits: u32, s: &[u64]) -> u64 {
    // println!("max_in_slice({}, {:?}) = ", digits, s);
    if digits == 1 {
        return *s.iter().max().unwrap();
    }
    let mut maxes: Vec<u64> = vec![];
    for i in 0..(s.len() - digits as usize + 1) {
        let first = s[i];
        let m = max_in_slice(digits - 1, &s[i + 1..]);
        maxes.push(first * 10_u64.pow(digits - 1) + m);
    }
    let max = *maxes.iter().max().unwrap();
    // println!("=> {}", max);
    max
}

fn to_max_joltage(digits: u32, js: &[u64]) -> u64 {
    let mut m = 0;
    for i in 0..(js.len() - digits as usize + 1) {
        m = max(m, max_in_slice(digits, &js[i..]));
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
