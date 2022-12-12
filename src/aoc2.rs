pub fn solve() {
    let mut score: u32 = 0;
    if let Ok(lines) = crate::helpers::read_lines("./data/2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let vec_line: Vec<char> = ip.chars().collect();
                let opponent = match vec_line[0] {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => panic!()
                };
                let me = match vec_line[2] {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => panic!()
                };
                let outcome = match (opponent, me) {
                    (1, 1) => 3,
                    (1, 2) => 6,
                    (1, 3) => 0,
                    (2, 1) => 0,
                    (2, 2) => 3,
                    (2, 3) => 6,
                    (3, 1) => 6,
                    (3, 2) => 0,
                    (3, 3) => 3,
                    (_, _) => panic!()
                };
                score = score + me + outcome
            }
        }
    }
    println!("Score: {}", score);
}
