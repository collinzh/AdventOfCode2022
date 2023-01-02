use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("unable to open file");
    io::BufReader::new(file).lines()
}

pub fn read_lines_unwrapped<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    lines
        .into_iter()
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .collect()
}

/// Returns true if supplied string is a number
pub fn is_numeric(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

#[allow(dead_code)]
pub fn read_ints_file<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    lines
        .into_iter()
        .map(|l| l.unwrap().parse::<i32>())
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect()
}
