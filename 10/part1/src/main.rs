use itertools::Itertools;
use std::io::stdin;

struct Machine {
    target: u64,
    patterns: Vec<u64>,
    joltages: Vec<u64>,
}

impl Machine {
    fn enableable_with(&self, n: usize) -> bool {
        for combo in self.patterns.iter().combinations(n) {
            if combo.iter().fold(0, |acc, b| acc ^ *b) == self.target {
                return true;
            }
        }
        false
    }

    pub fn min_buttons(&self) -> usize {
        print!("Target = {:08b} trying ", self.target);
        for n in 1..=self.patterns.len() {
            print!("{} ", n);
            if self.enableable_with(n) {
                println!();
                return n;
            }
        }
        panic!("Not enableable?!");
    }

    pub fn dump(&self) {
        println!(
            "{:08b} ({})",
            self.target,
            self.patterns
                .iter()
                .map(|n| format!("{:08b}", n))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

fn to_target(s: &str) -> u64 {
    s.chars()
        .rev()
        .skip(1)
        .take(s.len() - 2)
        .fold(0, |acc, c| (acc << 1) + (if c == '#' { 1 } else { 0 }))
}

fn to_patterns(ss: &[&str]) -> Vec<u64> {
    ss.iter()
        .map(|s| {
            s[1..(s.len() - 1)]
                .split(",")
                .map(|n_s| n_s.parse::<u64>().unwrap())
                .fold(0, |acc, n| acc | (1 << n))
        })
        .collect()
}

fn to_joltages(s: &str) -> Vec<u64> {
    s[1..(s.len() - 1)]
        .split(",")
        .map(|n_s| n_s.parse::<u64>().unwrap())
        .collect()
}

fn to_machine(line: &str) -> Machine {
    let chunks = line.split(" ").collect::<Vec<&str>>();
    Machine {
        target: to_target(chunks.first().unwrap()),
        patterns: to_patterns(&chunks[1..(chunks.len() - 1)]),
        joltages: to_joltages(chunks.last().unwrap()),
    }
}

fn main() {
    let machines = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| to_machine(&l))
        .collect::<Vec<Machine>>();
    let result = machines.iter().map(|m| m.min_buttons()).sum::<usize>();
    println!("{}", result);
}
