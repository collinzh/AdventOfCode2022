use crate::utils::strings::read_lines;

struct Elf {
    low: u32,
    high: u32,
}

impl Elf {
    pub fn new(range: &str) -> Elf {
        let v: Vec<&str> = range.split("-").collect();
        if v.len() != 2 {
            println!("Unrecognized elf: {}", range)
        }
        let low: u32 = v.get(0).unwrap().to_string().parse::<u32>().unwrap();
        let high: u32 = v.get(1).unwrap().to_string().parse::<u32>().unwrap();

        Elf { low, high }
    }

    /// Determines if this elf full contains the other elf. Please make sure this elf has larger range
    /// than the other elf
    pub fn fully_contains(&self, other: &Elf) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    /// Determines if this elf has overlapping range with other elf
    pub fn overlaps_with(&self, other: &Elf) -> bool {
        (self.low <= other.low && self.high >= other.low)
            || (self.high >= other.high && self.low <= other.high)
    }

    /// Total number of assignments
    pub fn total(&self) -> u32 {
        self.high - self.low + 1
    }
}

#[allow(dead_code)]
pub fn day4() {
    let lines = read_lines("day4.txt");
    let mut p1_count = 0;
    let mut p2_count = 0;
    for line in lines {
        let pairs = line.unwrap();
        let p: Vec<&str> = pairs.split(",").collect();
        let e1 = Elf::new(p.get(0).unwrap());
        let e2 = Elf::new(p.get(1).unwrap());

        // Check fully contains
        if e1.total() > e2.total() {
            if e1.fully_contains(&e2) {
                p1_count += 1;
                continue;
            }
        } else {
            if e2.fully_contains(&e1) {
                p1_count += 1;
                continue;
            }
        }

        // Check partial overlaps, fully contain cases not included
        if e1.overlaps_with(&e2) {
            p2_count += 1;
        }
    }

    // Full contain cases should be included
    p2_count += p1_count;

    println!("Day\t4: Part 1: {}, Part 2: {}", p1_count, p2_count);
}
