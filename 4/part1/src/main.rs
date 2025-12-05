use std::io::{self, BufRead};

fn to_row(s: String) -> Vec<bool> {
    s.chars().map(|c| c == '@').collect()
}

fn count_neighbours(locations: &[Vec<bool>]) -> Vec<Vec<Option<u8>>> {
    let mut neighbour_counts: Vec<Vec<Option<u8>>> = locations
        .iter()
        .map(|row| row.iter().map(|_| None).collect())
        .collect();

    for r_i in 0..neighbour_counts.len() {
        let row = &locations[r_i];
        let r: i16 = r_i.try_into().unwrap();
        for c_i in 0..row.len() {
            if !locations[r_i][c_i] {
                continue;
            }

            let c: i16 = c_i.try_into().unwrap();
            let mut count = 0;
            for x_i in 0..3 {
                let x: i16 = x_i - 1;
                if c + x < 0 {
                    continue;
                }
                if c + x >= row.len().try_into().unwrap() {
                    continue;
                }

                for y_i in 0..3 {
                    let y: i16 = y_i - 1;
                    if r + y < 0 {
                        continue;
                    }
                    if r + y >= locations.len().try_into().unwrap() {
                        continue;
                    }
                    if x == 0 && y == 0 {
                        continue;
                    }

                    let ry_i: usize = (r + y).try_into().unwrap();
                    let cx_i: usize = (c + x).try_into().unwrap();
                    if locations[ry_i][cx_i] {
                        count += 1;
                    }
                }
            }
            neighbour_counts[r_i][c_i] = Some(count);
        }
    }
    neighbour_counts
}

fn calc_accessible(neighbour_counts: &[Vec<Option<u8>>]) -> Vec<Vec<char>> {
    neighbour_counts
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| {
                    if let Some(value) = *n {
                        if value < 4 {
                            'x'
                        } else {
                            '@'
                        }
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect()
}

fn dump_locations(locations: &[Vec<bool>], skip: usize, count: usize) {
    for row in locations.iter().skip(skip).take(count) {
        for b in row.iter() {
            print!("{}", if *b { "@" } else { "." });
        }
        println!();
    }
}

fn dump_neighbours(neighbour_counts: &[Vec<Option<u8>>], skip: usize, count: usize) {
    for row in neighbour_counts.iter().skip(skip).take(count) {
        for n in row.iter() {
            if let Some(value) = n {
                print!("{}", value);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dump_accessible(accessible: &[Vec<char>], skip: usize, count: usize) {
    for row in accessible.iter().skip(skip).take(count) {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn dump_augmented(accessible: &[Vec<char>], skip: usize, count: usize, counts: &[u16]) {
    for (r_i, row) in accessible.iter().skip(skip).take(count).enumerate() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!(" {}", counts[r_i]);
    }
}

fn marker() {
    println!("---");
}

fn main() {
    let locations: Vec<Vec<bool>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_row)
        .collect();
    let offset: usize = 0;
    let window: usize = 3;
    // let window: usize = locations.len();
    dump_locations(&locations, offset, window);
    marker();
    let neighbour_counts = count_neighbours(&locations);
    dump_neighbours(&neighbour_counts, offset, window);
    marker();
    let accessible: Vec<Vec<char>> = calc_accessible(&neighbour_counts);
    // dump_accessible(&accessible, 0, accessible.len());
    // marker();
    let counts: Vec<u16> = neighbour_counts
        .iter()
        .map(|row| {
            row.iter()
                .filter(|c| {
                    if let Some(value) = **c {
                        value < 4
                    } else {
                        false
                    }
                })
                .count()
                .try_into()
                .unwrap()
        })
        .collect();
    dump_augmented(&accessible, offset, window, &counts);
    marker();
    // println!("{:?}", counts.iter().take(2).collect());
    let count: u16 = counts.iter().sum();
    println!("{}", count);
}
