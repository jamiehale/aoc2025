use std::io::stdin;

#[derive(Debug)]
struct Shape {
    pattern: [[bool; 3]; 3],
    solid_count: usize,
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    shape_counts: [usize; 6],
    total_shapes: usize,
}

fn extract_shape(lines: &[String]) -> Shape {
    let pattern: [[bool; 3]; 3] = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[bool; 3]>>()
        .try_into()
        .unwrap();
    Shape {
        pattern,
        solid_count: pattern
            .iter()
            .fold(0_usize, |acc, row| acc + row.iter().filter(|b| **b).count()),
    }
}

fn to_region(line: &str) -> Region {
    let tokens = line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let (width, length) = tokens[0]
        .split_once(":")
        .map(|o| {
            o.0.split_once("x")
                .map(|bs| {
                    (
                        bs.0.parse::<usize>().unwrap(),
                        bs.1.parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unwrap();
    let shape_counts: [usize; 6] = tokens[1..]
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    Region {
        width,
        length,
        shape_counts,
        total_shapes: shape_counts.iter().sum(),
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let shapes = (0..6)
        .map(|i| extract_shape(&lines[(i * 5 + 1)..(i * 5 + 4)]))
        .collect::<Vec<Shape>>();
    // for shape in shapes.iter() {
    //     println!("{:?}", shape);
    // }
    let regions = lines
        .iter()
        .skip(6 * 5)
        .map(|l| to_region(l))
        .collect::<Vec<Region>>();
    // for region in regions.iter() {
    //     println!("{:?}", region);
    // }
    println!("Total regions: {}", regions.len());
    let regions = regions
        .iter()
        .filter(|r| (r.width / 3) * (r.length / 3) >= r.total_shapes)
        .collect::<Vec<&Region>>();
    println!("Oversized regions: {}", regions.len());
}
