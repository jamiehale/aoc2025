use std::io::stdin;

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, microlp, variable,
};

struct Button {
    affected_levels: Vec<usize>,
}

impl Button {
    pub fn to_source(&self, width: usize) -> Vec<i32> {
        let mut source = vec![0_i32; width];
        for i in self.affected_levels.iter() {
            source[*i] = 1;
        }
        source
    }
}

struct Machine {
    buttons: Vec<Button>,
    joltages: Vec<i32>,
}

impl Machine {
    fn solve(&self, sources: &[Vec<i32>]) -> Option<Vec<u64>> {
        let num_sources = sources.len();
        let num_vars = self.joltages.len();

        let mut problem = ProblemVariables::new();

        let coeffs: Vec<Variable> = (0..num_sources)
            .map(|_| problem.add(variable().integer().min(0)))
            .collect();

        let objective: Expression = coeffs.iter().sum();

        let mut model = problem.minimise(objective).using(microlp);

        for var_idx in 0..num_vars {
            let lhs: Expression = coeffs
                .iter()
                .zip(sources.iter())
                .map(|(coeff, source)| source[var_idx] * *coeff)
                .sum();
            model = model.with(constraint!(lhs == self.joltages[var_idx]));
        }

        match model.solve() {
            Ok(solution) => Some(coeffs.iter().map(|c| solution.value(*c) as u64).collect()),
            Err(_) => None,
        }
    }

    pub fn calc_lowest_activation(&self) -> u64 {
        let sources = self
            .buttons
            .iter()
            .map(|b| b.to_source(self.joltages.len()))
            .collect::<Vec<Vec<i32>>>();
        println!("sources = {:?}", sources);
        println!("targets = {:?}", self.joltages);
        if let Some(result) = self.solve(&sources) {
            println!("solution = {:?}", result);
            return result.iter().sum();
        }
        panic!();
    }
}

fn to_buttons(ss: &[&str]) -> Vec<Button> {
    ss.iter()
        .map(|s| Button {
            affected_levels: s[1..(s.len() - 1)]
                .split(",")
                .map(|n_s| n_s.parse::<usize>().unwrap())
                .collect(),
        })
        .collect()
}

fn to_joltages(s: &str) -> Vec<i32> {
    s[1..(s.len() - 1)]
        .split(",")
        .map(|n_s| n_s.parse::<i32>().unwrap())
        .collect()
}

fn to_machine(line: &str) -> Machine {
    let chunks = line.split(" ").collect::<Vec<&str>>();
    Machine {
        buttons: to_buttons(&chunks[1..(chunks.len() - 1)]),
        joltages: to_joltages(chunks.last().unwrap()),
    }
}

fn main() {
    let machines = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| to_machine(&l))
        .collect::<Vec<Machine>>();
    let min_buttons = machines
        .iter()
        .map(|m| m.calc_lowest_activation())
        .collect::<Vec<u64>>();
    let result = min_buttons.iter().sum::<u64>();
    println!("{}", result);
}
