use std::collections::HashSet;
use std::collections::HashMap;


pub fn solve() {
    let mut score: u32 = 0;
    let mut elf: u8 = 0;
    let map: HashMap<_, _> = ('a'..='z')
        .zip(1..27)
        .into_iter()
        .chain(
            ('A'..='Z')
            .zip(27..53).
            into_iter()
        )
        .collect();

    let mut intersection: HashSet<char> = HashSet::new();
    if let Ok(lines) = crate::helpers::read_lines("./data/3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let line: HashSet<char> = ip.chars().collect();
                if elf == 0 {
                    for item in &intersection {
                        let priority: u32 = match map.get(&item) {
                            Some(&priority) => priority,
                            _ => panic!()
                        };
                        score = score + priority
                    }
                    intersection = line;
                    elf = 2
                }
                else {
                    intersection = intersection.intersection(&line).cloned().collect();
                    elf = elf - 1;
                }
            }
        }
    }
    for item in &intersection {
        let priority: u32 = match map.get(&item) {
            Some(&priority) => priority,
            _ => panic!()
        };
        score = score + priority;
    }
    println!("Score: {}", score);
}
