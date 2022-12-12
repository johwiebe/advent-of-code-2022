use std::collections::HashSet;

pub fn solve() {

    let mut start_of_packet: usize = 0;

    if let Ok(lines) = crate::helpers::read_lines("./data/6.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                start_of_packet = find_start_of_packet(&ip).unwrap();
            }
        }
    }
    println!("Start of packet: {}", start_of_packet);
}

fn find_start_of_packet(line: &str) -> Option<usize> {
    let message_length: usize = 14;
    for i in 0..line.len() - message_length + 1 {
        let range: &str = &line[i..i+message_length];
        if unique(range) {
            return Some(i + message_length);
        }
    }
    return None;
}

fn unique(range: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::new();
    for c in range.chars() {
        if seen.contains(&c) {
            return false;
        } else {
            seen.insert(c);
        }
    }
    return true;
}
