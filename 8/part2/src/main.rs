use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn from(coords: Vec<i64>) -> Self {
        if coords.len() != 3 {
            panic!("Invalid input");
        }
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    pub fn distance_squared_to(&self, other: &Point) -> i64 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }
}

impl PartialEq for Point {
    fn eq(&self, p: &Point) -> bool {
        p.x == self.x && p.y == self.y && p.z == self.z
    }
}

struct Distance {
    p1_i: usize,
    p2_i: usize,
    distance_squared: i64,
}

struct Circuit {
    id: usize,
    points: Vec<Point>,
}

impl Circuit {
    pub fn new(id: usize, p: &Point) -> Self {
        Self {
            id,
            points: vec![*p],
        }
    }

    pub fn contains(&self, p: &Point) -> bool {
        self.points.contains(p)
    }

    pub fn merge(self, other: &mut Circuit) -> Circuit {
        let mut new_points = self.points;
        new_points.append(&mut other.points);
        Circuit {
            id: self.id,
            points: new_points,
        }
    }
}

fn calc_sorted_distances(points: &[Point]) -> Vec<Distance> {
    let mut distances: Vec<Distance> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            distances.push(Distance {
                p1_i: i,
                p2_i: j,
                distance_squared: p1.distance_squared_to(p2),
            });
        }
    }
    distances.sort_by_key(|d| d.distance_squared);
    distances
}

fn init_circuits(points: &[Point]) -> Vec<Circuit> {
    points
        .iter()
        .enumerate()
        .map(|(i, p)| Circuit::new(i, p))
        .collect()
}

fn circuit_containing(circuits: &[Circuit], p: &Point) -> usize {
    for circuit in circuits.iter() {
        if circuit.contains(p) {
            return circuit.id;
        }
    }
    panic!("Nope")
}

fn merge_circuits(circuits: &mut Vec<Circuit>, c1_id: usize, c2_id: usize) {
    let c1 = circuits.remove(circuits.iter().position(|c| c.id == c1_id).unwrap());
    let mut c2 = circuits.remove(circuits.iter().position(|c| c.id == c2_id).unwrap());
    circuits.push(c1.merge(&mut c2));
}

fn main() {
    let points: Vec<Point> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            Point::from(
                l.unwrap()
                    .split(",")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();
    let distances = calc_sorted_distances(&points);
    let mut circuits = init_circuits(&points);
    for distance in distances.iter() {
        let c1_id = circuit_containing(&circuits, &points[distance.p1_i]);
        let c2_id = circuit_containing(&circuits, &points[distance.p2_i]);
        if c1_id == c2_id {
            // same circuit already - ignore
        } else {
            merge_circuits(&mut circuits, c1_id, c2_id);
            if circuits.len() == 1 {
                println!("{}", points[distance.p1_i].x * points[distance.p2_i].x);
                break;
            }
        }
    }
}
