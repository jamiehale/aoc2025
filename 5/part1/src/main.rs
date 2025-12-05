use std::{
    io::{self, prelude::*},
    ops::RangeInclusive,
};

fn is_id_fresh(id: u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(&id))
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut got_ranges = false;
    for line in lines.iter() {
        if got_ranges {
            ids.push(line.parse().unwrap());
        } else if line.is_empty() {
            got_ranges = true;
        } else {
            let (first_s, last_s) = line.split_once('-').unwrap();
            ranges.push(RangeInclusive::new(
                first_s.parse().unwrap(),
                last_s.parse().unwrap(),
            ));
        }
    }

    let fresh: Vec<bool> = ids.iter().map(|id| is_id_fresh(*id, &ranges)).collect();
    let fresh_count = fresh.iter().filter(|f| **f).count();
    println!("{}", fresh_count);
}
