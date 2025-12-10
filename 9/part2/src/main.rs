use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, BufWriter, Write, stdin};
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
struct Corner {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Red,
    Green,
    Other,
}

struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new((x, y): (usize, usize)) -> Self {
        Self {
            tiles: vec![vec![Tile::Other; x + 2]; y + 2],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, t: Tile) {
        self.tiles[y][x] = t;
    }

    pub fn set_corner(&mut self, c: &Corner, t: Tile) {
        self.set(c.x, c.y, t);
    }

    pub fn h_line(&mut self, y: usize, from_x: usize, to_x: usize, t: Tile) {
        for x in from_x..to_x {
            if self.tiles[y][x] != Tile::Red {
                self.set(x, y, t);
            }
        }
    }

    pub fn v_line(&mut self, x: usize, from_y: usize, to_y: usize, t: Tile) {
        for y in from_y..to_y {
            if self.tiles[y][x] != Tile::Red {
                self.set(x, y, t);
            }
        }
    }

    pub fn line(&mut self, c1: &Corner, c2: &Corner, t: Tile) {
        if is_vertical(c1, c2) {
            let x = c1.x;
            if v_length(c1, c2) > 2 {
                let from_y = c1.y.min(c2.y) + 1;
                let to_y = c1.y.max(c2.y);
                self.v_line(x, from_y, to_y, t);
            }
        } else {
            let y = c1.y;
            if h_length(c1, c2) > 2 {
                let from_x = c1.x.min(c2.x) + 1;
                let to_x = c1.x.max(c2.x);
                self.h_line(y, from_x, to_x, t);
            }
        }
    }

    pub fn fill(&mut self) {
        let mut lines: Vec<(usize, usize, usize)> = vec![];
        for (y, row) in self.tiles.iter().enumerate() {
            let mut fill_start: Option<usize> = None;
            for (x, tile) in row.iter().enumerate() {
                if let Some(from_x) = fill_start {
                    match tile {
                        Tile::Green => {
                            lines.push((y, from_x + 1, x));
                            fill_start = None;
                        }
                        Tile::Red => {}
                        _ => {}
                    }
                } else {
                    match tile {
                        Tile::Red | Tile::Green => {
                            fill_start = Some(x);
                        }
                        _ => {}
                    }
                }
            }
            if y.is_multiple_of(1000) {
                println!("{}/{}...", y, self.tiles.len());
            }
        }
        println!("Filling...");
        for (n, line) in lines.iter().enumerate() {
            self.h_line(line.0, line.1, line.2, Tile::Green);
            if n.is_multiple_of(1000) {
                println!("{}/{}...", n, lines.len());
            }
        }
    }

    pub fn contains_area(&self, c1: &Corner, c2: &Corner) -> bool {
        let start_x = c1.x.min(c2.x);
        let end_x = c1.x.max(c2.x);
        let start_y = c1.y.min(c2.y);
        let end_y = c1.y.max(c2.y);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                if self.tiles[y][x] == Tile::Other {
                    return false;
                }
            }
        }
        true
    }

    pub fn dump(&self) {
        for row in self.tiles.iter() {
            for column in row.iter() {
                print!(
                    "{}",
                    match column {
                        Tile::Red => "#",
                        Tile::Green => "X",
                        _ => ".",
                    }
                )
            }
            println!();
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let file = File::create(path).expect("Failed to create cache file");
        let mut writer = BufWriter::new(file);
        for (i, row) in self.tiles.iter().enumerate() {
            let s: String = row
                .iter()
                .map(|t| match t {
                    Tile::Red => '#',
                    Tile::Green => 'X',
                    Tile::Other => '.',
                })
                .collect();
            println!("Writing line {}...", i);
            writeln!(writer, "{}", s).unwrap();
        }
    }
}

fn load_from_file(path: &str) -> TileMap {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let tiles: Vec<Vec<Tile>> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            println!("Reading line {}...", i);
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Tile::Red,
                    'X' => Tile::Green,
                    _ => Tile::Other,
                })
                .collect()
        })
        .collect();
    TileMap { tiles }
}

fn to_corner(line: &str) -> Corner {
    line.split_once(",")
        .map(|(x_s, y_s)| (x_s.parse::<usize>().unwrap(), y_s.parse::<usize>().unwrap()))
        .map(|p| Corner { x: p.0, y: p.1 })
        .unwrap()
}

fn h_length(c1: &Corner, c2: &Corner) -> usize {
    c1.x.max(c2.x) - c1.x.min(c2.x) + 1
}

fn v_length(c1: &Corner, c2: &Corner) -> usize {
    c1.y.max(c2.y) - c1.y.min(c2.y) + 1
}

fn area_between(c1: &Corner, c2: &Corner) -> usize {
    h_length(c1, c2) * v_length(c1, c2)
}

fn is_vertical(c1: &Corner, c2: &Corner) -> bool {
    c1.x == c2.x
}

fn any_other_corner_inside(corners: &[Corner], c1: &Corner, c2: &Corner) -> bool {
    let min_x = c1.x.min(c2.x);
    let max_x = c1.x.max(c2.x);
    let min_y = c1.y.min(c2.y);
    let max_y = c1.y.max(c2.y);
    for corner in corners.iter() {
        if *corner == *c1 || *corner == *c2 {
            continue;
        }
        if corner.x > min_x && corner.x < max_x && corner.y > min_y && corner.y < max_y {
            return true;
        }
    }
    false
}

fn main() {
    let corners = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| to_corner(&s))
        .collect::<Vec<Corner>>();
    let bounds = corners
        .iter()
        .fold::<(usize, usize), _>((0, 0), |acc, c| (acc.0.max(c.x), acc.1.max(c.y)));
    let mut map = TileMap::new(bounds);
    println!("Drawing lines...");
    for pair in corners.windows(2) {
        let c1 = &pair[0];
        let c2 = &pair[1];
        map.set(c1.x, c1.y, Tile::Red);
        map.line(c1, c2, Tile::Green);
    }
    map.set_corner(corners.last().unwrap(), Tile::Red);
    map.line(
        corners.last().unwrap(),
        corners.first().unwrap(),
        Tile::Green,
    );
    println!("Lines complete");
    // map.dump();
    const CACHE_FILE: &str = "map_cache.txt";
    if Path::new(CACHE_FILE).exists() {
        println!("Loading from cache...");
        map = load_from_file(CACHE_FILE);
        println!("Loaded from cache");
    } else {
        map.fill();
        println!("Writing to cache...");
        map.save_to_file(CACHE_FILE);
    }
    // map.dump();
    let mut max_area: usize = 0;
    for i in 0..corners.len() {
        let c1 = &corners[i];
        for j in (i + 1)..corners.len() {
            print!("[{}/{}][{}] (max={})...", i, corners.len(), j, max_area);
            let c2 = &corners[j];
            if any_other_corner_inside(&corners, c1, c2) {
                println!(" skipping");
                continue;
            }
            println!(" testing");
            if map.contains_area(c1, c2) {
                max_area = max_area.max(area_between(c1, c2));
            }
        }
    }
    println!("{}", max_area);
}
