use std::io::stdin;

#[derive(Clone, Copy)]
struct Corner {
    x: u64,
    y: u64,
}

fn to_corner(line: &str) -> Corner {
    line.split_once(",")
        .map(|(x_s, y_s)| (x_s.parse::<u64>().unwrap(), y_s.parse::<u64>().unwrap()))
        .map(|p| Corner { x: p.0, y: p.1 })
        .unwrap()
}

fn area_between(p1: &Corner, p2: &Corner) -> u64 {
    (p1.x.max(p2.x) - p1.x.min(p2.x) + 1) * (p1.y.max(p2.y) - p1.y.min(p2.y) + 1)
}

fn main() {
    let corners = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| to_corner(&s))
        .collect::<Vec<Corner>>();
    let mut result: Option<(u64, Corner, Corner)> = None;
    for i in 0..corners.len() {
        for j in i..corners.len() {
            let from = &corners[i];
            let to = &corners[j];
            let area = area_between(from, to);
            if let Some((max_area, _, _)) = result {
                if area > max_area {
                    result = Some((area, *from, *to));
                }
            } else {
                result = Some((area, *from, *to));
            }
        }
    }
    println!("{}", result.unwrap().0);
}
