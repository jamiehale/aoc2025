use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug)]
enum Alignment {
    Left,
    Right,
    Both,
}

#[derive(Debug)]
struct Column {
    operator: Operator,
    alignment: Option<Alignment>,
    numbers: Vec<u64>,
}

fn prime_columns(operator_line: &str) -> Vec<Column> {
    operator_line
        .split_whitespace()
        .map(|s| match s {
            "+" => Column {
                operator: Operator::Add,
                alignment: None,
                numbers: vec![],
            },
            _ => Column {
                operator: Operator::Multiply,
                alignment: None,
                numbers: vec![],
            },
        })
        .collect()
}

fn prime_numbers(columns: Vec<Column>, lines: &[String]) -> Vec<Column> {
    lines.iter().fold(columns, |acc, l| {
        let tokens = l
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        acc.into_iter()
            .enumerate()
            .map(|(i, c)| Column {
                operator: c.operator,
                alignment: c.alignment,
                numbers: c.numbers.into_iter().chain([tokens[i]]).collect::<Vec<_>>(),
            })
            .collect()
    })
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

fn find_alignment(rows: &[Vec<String>], i: usize) -> Alignment {
    rows.iter().fold(Alignment::Both, |acc, row| match acc {
        Alignment::Left => Alignment::Left,
        Alignment::Right => Alignment::Right,
        _ => {
            if row[i].starts_with(" ") {
                Alignment::Right
            } else if row[i].ends_with(" ") {
                Alignment::Left
            } else {
                Alignment::Both
            }
        }
    })
}

fn add_alignment(columns: Vec<Column>, lines: &[String]) -> Vec<Column> {
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
            alignment: Some(find_alignment(&all_strings, i)),
            numbers: c.numbers,
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
    let columns = prime_numbers(columns, lines.split_last().unwrap().1);
    // println!("{:?}", columns);
    let columns = add_alignment(columns, lines.split_last().unwrap().1);
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
