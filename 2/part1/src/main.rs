use std::io::{self, Read};

struct Range {
    from: u64,
    to: u64,
}

impl From<String> for Range {
    fn from(s: String) -> Range {
        let (from_s, to_s) = s.split_once('-').unwrap();
        Range {
            from: from_s.parse().unwrap(),
            to: to_s.parse().unwrap(),
        }
    }
}

fn is_invalid(n: u64) -> bool {
    let s = n.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let (first_s, second_s) = s.split_at(s.len() / 2);
    first_s == second_s
}

fn main() {
    let reader = io::stdin().lock();
    let mut ranges: Vec<Range> = vec![];
    let mut s = String::new();
    for byte_result in reader.bytes() {
        let byte = byte_result.unwrap();
        let c = byte as char;
        if c == '\n' {
            continue;
        }
        if c == ',' {
            ranges.push(s.into());
            s = String::new();
        } else {
            s.push(c);
        }
    }
    ranges.push(s.into());
    let mut invalid_ids: Vec<u64> = vec![];
    for range in ranges.iter() {
        let mut n = range.from;
        while n <= range.to {
            if is_invalid(n) {
                invalid_ids.push(n);
            }
            n += 1;
        }
    }
    let sum: u64 = invalid_ids.iter().sum();
    println!("{}", sum);
}
