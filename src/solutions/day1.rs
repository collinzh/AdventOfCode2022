use crate::utils::strings::read_lines;

#[allow(dead_code)]
pub fn day1() {
    let lines = read_lines("day1.txt");
    let mut nums: Vec<Vec<i32>> = vec![];
    let mut elf: Vec<i32> = vec![];

    for line in lines.into_iter() {
        let str = line.unwrap();
        if str == "" {
            nums.push(elf.clone());
            elf.clear();
        } else {
            let n = str.parse::<i32>().unwrap();
            elf.push(n)
        }
    }

    let mut max: Vec<i32> = nums
        .into_iter()
        .map(|n| n.into_iter().reduce(|accum, item| accum + item).unwrap())
        .collect();

    max.sort_by(|a, b| b.cmp(a));

    println!(
        "Day\t1: Top 3 are {}, {}, {}, sum {}",
        max[0],
        max[1],
        max[2],
        max[0] + max[1] + max[2]
    )
}
