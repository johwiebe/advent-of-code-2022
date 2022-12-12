use std::collections::HashSet;


pub fn solve() {
    let mut score: u32 = 0;

    if let Ok(lines) = crate::helpers::read_lines("./data/4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if overlap(&ip) {
                    score = score + 1
                }
            }
        }
    }
    println!("Score: {}", score);
}

fn overlap(line: &String) -> bool {
    let (left, right): (&str, &str) = line.split_once(',').unwrap();
    let (ids_left, ids_right): (HashSet<u8>, HashSet<u8>) = (get_ids(&left), get_ids(&right));
    !ids_left.is_disjoint(&ids_right)
}

fn get_ids(string: &str) -> HashSet<u8> {
    let (left, right): (&str, &str) = string.split_once('-').unwrap();
    let (left_int, right_int): (u8, u8) = (left.parse().unwrap(), right.parse().unwrap());
    (left_int..=right_int).collect()
}
