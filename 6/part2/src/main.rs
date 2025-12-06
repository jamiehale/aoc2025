use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Column {
    operator: Operator,
    width: Option<usize>,
    strings: Vec<String>,
    numbers: Vec<u64>,
}

fn prime_columns(operator_line: &str) -> Vec<Column> {
    operator_line
        .split_whitespace()
        .map(|s| match s {
            "+" => Column {
                operator: Operator::Add,
                width: None,
                strings: vec![],
                numbers: vec![],
            },
            _ => Column {
                operator: Operator::Multiply,
                width: None,
                strings: vec![],
                numbers: vec![],
            },
        })
        .collect()
}

fn split_line(line: &str, widths: &[usize]) -> Vec<String> {
    let mut offset = 0;
    let mut strings: Vec<String> = vec![];
    for width in widths.iter() {
        strings.push(line[offset..(offset + width)].to_string());
        offset += width + 1;
    }
    strings
}

fn add_widths(columns: Vec<Column>, lines: &[String]) -> Vec<Column> {
    let mut widths: Vec<usize> = vec![0; lines.first().unwrap().split_whitespace().count()];
    for l in lines.iter() {
        let these_widths = l
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.len())
            .collect::<Vec<usize>>();
        for (i, width) in these_widths.iter().enumerate() {
            widths[i] = widths[i].max(*width);
        }
    }
    let all_strings = lines
        .iter()
        .map(|l| split_line(l, &widths))
        .collect::<Vec<Vec<String>>>();
    columns
        .into_iter()
        .enumerate()
        .map(|(i, c)| Column {
            operator: c.operator,
            width: Some(widths[i]),
            strings: all_strings.iter().fold(vec![], |acc, row| {
                acc.into_iter().chain([row[i].clone()]).collect::<Vec<_>>()
            }),
            numbers: c.numbers,
        })
        .collect()
}

fn extract_numbers(strings: Vec<String>, width: usize) -> Vec<u64> {
    (0..width)
        .map(|i| {
            strings.iter().fold(0_u64, |acc, s| {
                let c = s.chars().nth(i).unwrap();
                if c == ' ' {
                    acc
                } else {
                    acc * 10 + s[i..(i + 1)].parse::<u64>().unwrap()
                }
            })
        })
        .rev()
        .collect()
}

fn add_numbers(columns: Vec<Column>) -> Vec<Column> {
    columns
        .into_iter()
        .map(|c| Column {
            operator: c.operator,
            width: c.width,
            strings: vec![],
            numbers: extract_numbers(c.strings, c.width.unwrap()),
        })
        .collect()
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let columns = prime_columns(lines.iter().last().unwrap());
    // println!("{:?}", columns);
    let columns = add_widths(columns, lines.split_last().unwrap().1);
    // println!("{:?}", columns);
    let columns = add_numbers(columns);
    // println!("{:?}", columns);

    let operator_results: Vec<u64> = columns
        .iter()
        .map(|c| match c.operator {
            Operator::Add => c.numbers.iter().sum(),
            _ => c.numbers.iter().product(),
        })
        .collect();
    // println!("{:?}", operator_results);
    let sum: u64 = operator_results.iter().sum();
    println!("{}", sum);
}
