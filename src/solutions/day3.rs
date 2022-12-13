use crate::utils::strings::read_lines;

#[allow(dead_code)]
pub fn day3() {
    let lines = read_lines("day3.txt");
    let mut total: u32 = 0;
    let mut groups: Vec<Vec<String>> = vec![];
    let mut group: Vec<String> = vec![];

    for line in lines {
        let rucksack = line.unwrap();

        // Build groups for part 2
        group.push(rucksack.clone());
        if group.len() == 3 {
            groups.push(group.clone());
            group.clear();
        }

        // Split a rucksack into 2 compartments
        let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
        // then find the common element
        let common = find_common_char(c1, c2).unwrap();
        total += item_priority(common)
    }

    let p2_total = part2(&groups);

    println!("Day\t3: Sum of priorities: {}, part 2: {}", total, p2_total);
}

fn item_priority(item: u8) -> u32 {
    // upper case letters have priorities 27 to 52
    if item >= 0x41 && item <= 0x5A {
        return (item - 0x40) as u32 + 26_u32;
    } else if item >= 0x61 && item <= 0x7A {
        // lower case letters have priorities 1 to 26
        return (item - 0x60) as u32;
    } else {
        println!("Common is not a letter: {}", item)
    }

    0
}

fn part2(groups: &Vec<Vec<String>>) -> u32 {
    let mut total: u32 = 0;
    for group in groups {
        let e1 = group.get(0).unwrap().as_str();
        let e2 = group.get(1).unwrap().as_str();
        let e3 = group.get(2).unwrap().as_str();

        // Find the common item from 3 rucksacks
        let common = find_common_char3(e1, e2, e3).unwrap();
        total += item_priority(common);
    }

    total
}

fn find_common_char3(e1: &str, e2: &str, e3: &str) -> Option<u8> {
    let e1b = e1.as_bytes();
    let e2b = e2.as_bytes();
    let e3b = e3.as_bytes();

    for b in e1b {
        if e2b.contains(b) && e3b.contains(b) {
            return Some(*b);
        }
    }

    println!("Warning, no common elements found");
    None
}

fn find_common_char(s1: &str, s2: &str) -> Option<u8> {
    let c1_bytes = s1.as_bytes();
    let c2_bytes = s2.as_bytes();
    for b in c1_bytes {
        if c2_bytes.contains(b) {
            return Some(*b);
        }
    }

    println!("Warning, no common elements found");
    None
}
