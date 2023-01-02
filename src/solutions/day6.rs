use crate::utils::strings;

#[allow(dead_code)]
pub fn day6() {
    let line = strings::read_lines_unwrapped("day6.txt")
        .get(0)
        .unwrap()
        .to_owned();

    let p1 = look_ahead_search(&line, 4);
    let p2 = look_ahead_search(&line, 14);

    println!("Day\t6: Part 1: {}, Part 2: {}", p1, p2);
}

fn look_ahead_search(line: &String, max_ahead: usize) -> usize {
    let mut base_ptr: usize = 0;
    let data = line.to_owned().into_bytes();

    'main: loop {
        // look ahead buffer
        let mut buffer: Vec<u8> = vec![];

        // For the next 4/14 characters....
        for ahead in 0..max_ahead {
            match data.get(base_ptr + ahead) {
                // Determine if the next character is in look ahead buffer
                Some(char) => match index_of(&buffer, char) {
                    Some(idx) => {
                        // forward base pointer to one after the duplicated char
                        base_ptr += idx + 1;
                        continue 'main;
                    }
                    None => {
                        // Add new char to buffer
                        buffer.push(*char);
                    }
                },
                None => {
                    println!("No more data");
                    break 'main;
                }
            }
        }

        break;
    }

    // println!(
    //     "Base pointer is {}, first data at {}",
    //     base_ptr,
    //     base_ptr + max_ahead
    // );

    base_ptr + max_ahead
}

/// Find the index of a character in a vector of characters
fn index_of(v: &Vec<u8>, char: &u8) -> Option<usize> {
    v.iter().position(|&c| c == *char)
}
