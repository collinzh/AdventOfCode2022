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
