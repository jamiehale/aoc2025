use std::{
    io::{self, prelude::*},
    ops::RangeInclusive,
};

fn merged(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|r| *r.start());

    let mut result: Vec<RangeInclusive<u64>> = vec![ranges[0].clone()];

    for range in ranges.into_iter().skip(1) {
        let last = result.last_mut().unwrap();
        if range.start() <= last.end() || *range.start() == last.end() + 1 {
            *last = *last.start()..=*last.end().max(range.end());
        } else {
            result.push(range);
        }
    }

    result
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        let (first_s, last_s) = line.split_once('-').unwrap();
        ranges.push(RangeInclusive::new(
            first_s.parse().unwrap(),
            last_s.parse().unwrap(),
        ));
    }

    let merged_ranges = merged(ranges);
    let fresh_count = merged_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>();

    println!("{}", fresh_count);
}
