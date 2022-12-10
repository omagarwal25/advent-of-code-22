use std::{ops::Index, str::Chars};

use itertools::Itertools;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl From<String> for Move {
    fn from(s: String) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        let amount = parts[1];
        let from = parts[3];
        let to = parts[5];

        let amount = amount.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        Move { from, to, amount }
    }
}

pub fn part1(inp: String) {
    let lines: std::str::Lines = inp.lines();

    let starting = lines
        .clone()
        .take_while(|f| *f != "")
        .collect::<Vec<&str>>();

    let columns = starting
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let rest = lines
        .skip_while(|f| *f != "")
        .skip(1)
        .collect::<Vec<&str>>()
        .clone()
        .iter()
        .map(|f| f.to_string())
        .map(Move::from)
        .collect::<Vec<Move>>();

    let stacks: Vec<Vec<char>> = starting
        .iter()
        .take(starting.len() - 1)
        .map(|line| {
            line.chars()
                .clone()
                .chunks(4)
                .into_iter()
                .map(|f| f.collect::<String>().chars().nth(1).unwrap())
                .collect::<Vec<char>>()
        })
        .collect();

    // rotate stacks 90 degrees funtionally
    let mut rotated = vec![vec![' '; stacks.len()]; stacks[0].len()];
    for i in 0..stacks.len() {
        for j in 0..stacks[i].len() {
            rotated[j][i] = stacks[i][j];
        }
    }

    // turn each stack into a LIFO stack of chars with index 0 being the top
    let mut stacks: Vec<Vec<char>> = rotated
        .iter()
        .map(|f| f.iter().rev().cloned().collect::<Vec<char>>())
        .collect();

    let mut stacks = stacks
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter(|f| f != &&' ')
                .map(|f| f.clone())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    for instruction in rest {
        let from = instruction.from;
        let to = instruction.to;
        let amount = instruction.amount;

        for _ in 0..amount {
            let top = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(top);
        }
    }

    let tops = stacks.iter().map(|f| f.last().unwrap()).collect::<String>();

    println!("{}", tops)
}

pub fn part2(inp: String) {
    let lines: std::str::Lines = inp.lines();

    let starting = lines
        .clone()
        .take_while(|f| *f != "")
        .collect::<Vec<&str>>();

    let columns = starting
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let rest = lines
        .skip_while(|f| *f != "")
        .skip(1)
        .collect::<Vec<&str>>()
        .clone()
        .iter()
        .map(|f| f.to_string())
        .map(Move::from)
        .collect::<Vec<Move>>();

    let stacks: Vec<Vec<char>> = starting
        .iter()
        .take(starting.len() - 1)
        .map(|line| {
            line.chars()
                .clone()
                .chunks(4)
                .into_iter()
                .map(|f| f.collect::<String>().chars().nth(1).unwrap())
                .collect::<Vec<char>>()
        })
        .collect();

    // rotate stacks 90 degrees funtionally
    let mut rotated = vec![vec![' '; stacks.len()]; stacks[0].len()];
    for i in 0..stacks.len() {
        for j in 0..stacks[i].len() {
            rotated[j][i] = stacks[i][j];
        }
    }

    // turn each stack into a LIFO stack of chars with index 0 being the top
    let mut stacks: Vec<Vec<char>> = rotated
        .iter()
        .map(|f| f.iter().rev().cloned().collect::<Vec<char>>())
        .collect();

    let mut stacks = stacks
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter(|f| f != &&' ')
                .map(|f| f.clone())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    for instruction in rest {
        let from = instruction.from;
        let to = instruction.to;
        let amount = instruction.amount;

        let length = stacks[from - 1].len();

        // pop amount off stacks from
        let elements = stacks[from - 1]
            .drain(length - amount..)
            .collect::<Vec<char>>();
        stacks[to - 1].extend(elements);
    }

    let tops = stacks.iter().map(|f| f.last().unwrap()).collect::<String>();

    println!("{}", tops)
}
