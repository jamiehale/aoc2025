use std::{
    cmp::max,
    io::{self, BufRead},
};

fn to_rotation(s: String) -> i32 {
    let (prefix, num_str) = s.split_at(1);
    let value: u32 = num_str.parse().unwrap();

    if prefix == "R" {
        return value.try_into().unwrap();
    }
    return -1 * i32::try_from(value).unwrap();
}

fn brute_force((current, count): (i32, i32), r: &i32) -> (i32, i32) {
    let mut zeros = 0;
    let mut remaining = r.abs();
    let mut new_current = current;
    while remaining > 0 {
        remaining -= 1;
        new_current += if *r > 0 { 1 } else { -1 };
        if new_current == -1 {
            new_current = 99;
        }
        if new_current == 100 {
            new_current = 0;
        }
        if new_current == 0 {
            zeros += 1;
        }
    }
    println!(
        "start = {} r = {} new = {} zeros = {}",
        current, r, new_current, zeros
    );
    (new_current, count + zeros)
}

fn rotate_cw(current: i32, count: i32, r: i32) -> (i32, i32) {
    let mut zeros = 0;
    let to_first_zero = 100 - current;
    if r >= to_first_zero {
        zeros = 1 + max(0, r - to_first_zero) / 100;
    }
    let new_current = (current + r) % 100;
    println!(
        "start = {} r = {} new = {} zeros = {}",
        current, r, new_current, zeros
    );
    (new_current, count + zeros)
}

fn normalize(mut n: i32) -> i32 {
    while n < 0 {
        n += 100;
    }
    n
}

fn rotate_ccw(current: i32, count: i32, r: i32) -> (i32, i32) {
    let mut zeros = 0;
    let to_first_zero = if current == 0 { 100 } else { current };
    if r == to_first_zero {
        zeros = 1;
    }
    if r > to_first_zero {
        zeros = 1 + max(0, r - to_first_zero) / 100;
    }
    let new_current = normalize(current - r);
    println!(
        "start = {} r = {} new = {} zeros = {}",
        current, -r, new_current, zeros
    );
    (new_current, count + zeros)
}

fn better((current, count): (i32, i32), r: &i32) -> (i32, i32) {
    if *r > 0 {
        rotate_cw(current, count, *r)
    } else {
        rotate_ccw(current, count, -1 * *r)
    }
}

fn main() {
    let rotations: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_rotation)
        .collect();
    let (_, count) = rotations.iter().fold((50, 0), better);
    println!("Count = {}", count);
}
