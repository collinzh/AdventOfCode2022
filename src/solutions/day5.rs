use crate::utils;
use crate::utils::strings::read_lines_unwrapped;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

#[allow(dead_code)]
pub fn day5() {
    let lines: Vec<String> = read_lines_unwrapped("day5.txt");
    let mut iter = lines.iter();
    let (header, row) = parse_stack(&mut iter);

    let pivoted_stack: HashMap<u32, Vec<String>> = get_pivoted_stack(&header, row);
    let moves = parse_moves(&mut iter);

    let p1_final = do_moves(&pivoted_stack, &moves, crane_p1);
    let p1 = read_top(&p1_final, &header);

    let p2_final = do_moves(&pivoted_stack, &moves, crane_p2);
    let p2 = read_top(&p2_final, &header);

    println!("Day\t5: Part 1: {}, Part 2: {}", p1, p2);
}

fn do_moves(
    stack: &HashMap<u32, Vec<String>>,
    moves: &Vec<Move>,
    crane: fn(&Vec<String>, &Vec<String>, u32) -> (Vec<String>, Vec<String>),
) -> HashMap<u32, Vec<String>> {
    let mut pivoted_stack = stack.to_owned();
    for m in moves {
        let from = pivoted_stack.get(&m.from).unwrap();
        let to = pivoted_stack.get(&m.to).unwrap();
        let (new_from, new_to) = crane(from, to, m.amount);
        pivoted_stack.insert(m.from, new_from);
        pivoted_stack.insert(m.to, new_to);
    }

    pivoted_stack
}

fn read_top(stack: &HashMap<u32, Vec<String>>, header: &Vec<u32>) -> String {
    let mut result = String::new();
    for i in header {
        let stack = stack.get(&i).unwrap();
        match stack.last() {
            Some(s) => result.push_str(s.as_str()),
            None => println!("Stack {} is empty", i),
        };
    }

    result
}

fn crane_p1(from: &Vec<String>, to: &Vec<String>, amount: u32) -> (Vec<String>, Vec<String>) {
    let start = from.len() - amount as usize;
    let new_from = from[..start].to_owned();
    let mut new_to = to.to_owned();
    for i in 0..amount as usize {
        let next = from.get(from.len() - i - 1).unwrap().to_string();
        new_to.push(next);
    }

    (new_from, new_to)
}

fn crane_p2(from: &Vec<String>, to: &Vec<String>, amount: u32) -> (Vec<String>, Vec<String>) {
    let start = from.len() - amount as usize;
    let new_from = from[..start].to_owned();
    let mut new_to = to.to_owned();
    for i in start..from.len() as usize {
        let next = from.get(i).unwrap().to_string();
        new_to.push(next);
    }

    (new_from, new_to)
}

fn parse_moves(iter: &mut Iter<String>) -> Vec<Move> {
    lazy_static! {
        static ref MOVE_PATTERN: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let mut moves: Vec<Move> = vec![];
    loop {
        let next = iter.next();
        if next.is_none() {
            break;
        }

        let line = next.unwrap().to_owned();
        let captures = MOVE_PATTERN.captures(line.as_str());
        if captures.is_some() {
            let captured = captures.unwrap();
            let amount = captured[1].parse::<u32>().unwrap();
            let from = captured[2].parse::<u32>().unwrap();
            let to = captured[3].parse::<u32>().unwrap();
            moves.push(Move { amount, from, to })
        }
    }

    moves
}

/// Pivot rows into columns in reverse order
fn get_pivoted_stack(header: &Vec<u32>, stack: Vec<Vec<String>>) -> HashMap<u32, Vec<String>> {
    let mut pivoted: HashMap<u32, Vec<String>> = HashMap::new();
    let height = stack.len();

    header.iter().for_each(|head| {
        let mut column: Vec<String> = vec![];

        'f: for h in 1..height + 1 {
            let row = height - h;
            let cell = stack.get(row).unwrap().get((head - 1) as usize);
            match cell {
                Some(s) => {
                    if *s != "" {
                        column.push(s.to_string());
                    } else {
                        break 'f;
                    }
                }
                None => break 'f,
            }
        }
        pivoted.insert(*head, column);
    });

    pivoted
}

fn parse_stack(iter: &mut Iter<String>) -> (Vec<u32>, Vec<Vec<String>>) {
    let mut rows: Vec<Vec<String>> = vec![];
    let mut headers: Vec<u32> = vec![];

    loop {
        let line = iter.next().unwrap();

        let mut idx = 0;
        let mut row: Vec<String> = vec![];

        if line.trim() == "" {
            break;
        }

        while idx < line.len() {
            let mut sub = line.get(idx..idx + 3);
            if sub.is_none() && line.len() - idx > 0 {
                sub = line.get(idx..);
            }

            if sub.is_some() {
                let has_sub = sub.unwrap();
                let char = has_sub.get(1..2).unwrap();

                if char == " " {
                    row.push(String::from(""));
                } else if utils::strings::is_numeric(char) {
                    headers.push(char.parse::<u32>().unwrap())
                } else {
                    row.push(String::from(char));
                }
            }
            idx += 4;
        }
        if !headers.is_empty() {
            break;
        }
        rows.push(row);
    }

    (headers, rows)
}
