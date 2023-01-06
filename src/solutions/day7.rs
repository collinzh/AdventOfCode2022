use crate::utils::strings;
use crate::utils::strings::is_numeric;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Clone, Debug)]
struct Node {
    is_dir: bool,
    size: u32,
    children: Vec<String>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_dir {
            write!(f, "Dir size {}, children: {:?}", self.size, self.children)
        } else {
            write!(f, "File size {}", self.size)
        }
    }
}

type DirStructure = HashMap<String, Node>;

#[allow(dead_code)]
pub fn day7() {
    let lines = strings::read_lines_unwrapped("day7.txt");
    let mut structures = parse_dir_structures(&lines);
    sum_dir_sizes(&mut structures);
    let p1 = part1(&structures);
    let p2 = part2(&structures);

    println!("Day\t7: Part 1: {}, Part 2: {}", p1, p2);
}

fn parse_dir_structures(lines: &Vec<String>) -> DirStructure {
    let mut structures: DirStructure = HashMap::new();

    let mut cwd: Vec<String> = vec![];

    structures.insert(
        "/".to_string(),
        Node {
            is_dir: true,
            children: vec![],
            size: 0,
        },
    );

    let mut line_it = lines.iter().peekable();
    loop {
        let n = line_it.next();
        if n.is_none() {
            break;
        }

        let next_line = n.unwrap().to_owned();

        let mut parts = next_line.split_whitespace();
        let part1 = parts.next().unwrap();
        // The main loop should only have to handle commands
        if part1 != "$" {
            println!("Unrecognized line {}", part1);
            break;
        }

        let part2 = parts.next().unwrap();
        // Handle cd command
        if part2 == "cd" {
            let name = parts.next().unwrap().to_string();
            if name == ".." {
                cwd.pop();
            } else if name == "/" {
                cwd.push("".to_string());
            } else {
                cwd.push(name);
            }
        } else if part2 == "ls" {
            parse_ls_results(&join_path(&cwd), &mut line_it, &mut structures);
        }
    }

    // for (f, n) in &structures {
    //     println!("File {0}, Node {1}", f, n);
    // }

    structures
}

fn sum_dir_sizes(structures: &mut DirStructure) {
    let mut to_visit: HashSet<String> = HashSet::new();
    to_visit.insert("/".to_string());

    while !to_visit.is_empty() {
        let visiting: Vec<&String> = Vec::from_iter(to_visit.iter());
        to_visit = scan_all_pending(structures, &visiting);
    }
}

/// performs a BFS search
fn scan_all_pending(structures: &mut DirStructure, to_visit: &Vec<&String>) -> HashSet<String> {
    // directories with missing dependencies
    let mut new_to_visit: HashSet<String> = HashSet::new();

    // for each node to be scanned this iteration
    for next in to_visit {
        if let Some(node) = structures.get(*next) {
            if !node.is_dir {
                continue;
            }

            let mut total = 0u32;
            let mut ready = true;

            for child in node.children.iter() {
                if let Some(c) = structures.get(child) {
                    if c.size != 0 {
                        total += c.size;
                    } else if c.is_dir {
                        // scan the child directory next round
                        new_to_visit.insert(child.to_string());
                        ready = false;
                    }
                } else {
                    println!("Should not be possible, but file {} is missing", child);
                }
            }

            if ready {
                // update size of current directory if all dependencies are met
                let mut n = node.clone();
                n.size = total;
                structures.insert(next.to_string(), n);
            } else {
                // retry this dir later
                new_to_visit.insert(next.to_string());
            }
        }
    }

    new_to_visit
}

/// Parse ls command output
fn parse_ls_results(
    cwd: &String,
    line_iter: &mut Peekable<Iter<String>>,
    structures: &mut DirStructure,
) {
    let mut dir = Node {
        is_dir: true,
        children: vec![],
        size: 0,
    };

    'dir: loop {
        // Peek and make sure we have not reached the end of command output
        match line_iter.peek() {
            Some(&s) => {
                if s.starts_with("$") {
                    break 'dir;
                }
            }
            None => break 'dir,
        }

        let next = line_iter.next();

        let mut parts = next.unwrap().split_whitespace();
        let p1 = parts.next().unwrap();
        let mut name = cwd.to_owned();
        if name != "/" {
            name.push_str("/");
        }
        name.push_str(parts.next().unwrap());

        // Entries for files start with their size
        if is_numeric(p1) {
            // Add a new file to the directory
            let size = p1.parse::<u32>().unwrap();
            structures.insert(
                name.clone(),
                Node {
                    is_dir: false,
                    size,
                    children: Vec::with_capacity(0),
                },
            );
            dir.children.push(name);
        } else if p1 == "dir" {
            // Entries for directories start with "dir"
            // Just push an empty name for directories
            dir.children.push(name);
        }
    }

    let mut current = cwd.to_owned();
    if cwd == "" {
        current = "/".to_string();
    }

    // Only insert listed directories to file nodes
    structures.insert(current, dir);
}

fn join_path(cwd: &Vec<String>) -> String {
    cwd.join("/")
}

fn part1(structures: &DirStructure) -> u32 {
    let mut total = 0;
    for (_, node) in structures {
        if node.is_dir && node.size <= 100000 {
            total += node.size;
        }
    }

    total
}

fn part2(structures: &DirStructure) -> u32 {
    let mut smallest = 2147483627_u32;

    let root = structures.get("/");
    let remaining = 70000000 - root.unwrap().size;
    let required = 30000000_u32;

    for (_, node) in structures {
        if node.is_dir && remaining + node.size >= required {
            if node.size < smallest {
                smallest = node.size;
            }
        }
    }

    smallest
}
