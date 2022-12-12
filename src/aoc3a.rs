use std::collections::HashSet;
use std::collections::HashMap;


pub fn solve() {
    let mut score: u32 = 0;
    let map: HashMap<_, _> = ('a'..='z')
        .zip(1..27)
        .into_iter()
        .chain(
            ('A'..='Z')
            .zip(27..53).
            into_iter()
        )
        .collect();

    if let Ok(lines) = crate::helpers::read_lines("./data/3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let vec_line: Vec<char> = ip.chars().collect();
                let left: HashSet<_> = vec_line[0..vec_line.len() / 2].into_iter().collect();
                let right: HashSet<_> = vec_line[vec_line.len() / 2..].into_iter().collect();
                let intersection = left.intersection(&right);
                for item in intersection {
                    let priority: u32 = match map.get(&item) {
                        Some(&priority) => priority,
                        _ => panic!()
                    };
                    score = score + priority
                };
            }
        }
    }
    println!("Score: {}", score);
}
