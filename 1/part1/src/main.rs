use std::io::{self, BufRead};

fn to_rotation(s: String) -> i32 {
    let (prefix, num_str) = s.split_at(1);
    let value: u32 = num_str.parse().unwrap();

    if prefix == "R" {
        return value.try_into().unwrap();
    }
    return -1 * i32::try_from(value).unwrap();
}

fn main() {
    let rotations: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_rotation)
        .collect();
    let (_, count) = rotations.iter().fold((50, 0), |(current, count), r| {
        let new_current = (current + r) % 100;
        (
            new_current,
            if new_current == 0 { count + 1 } else { count },
        )
    });
    println!("Count = {}", count);
}
