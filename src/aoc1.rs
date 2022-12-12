pub fn solve() {
    let mut calories = Vec::new();
    let mut current_calories = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = crate::helpers::read_lines("./data/1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    calories.push(current_calories);
                    current_calories = 0;
                } else {
                    let meal: u32 = ip.parse().unwrap();
                    current_calories = current_calories + meal;
                }
            }
        }
        calories.sort();
        calories.reverse();
        let sum: u32 = calories[1..=3].iter().sum();
        println!("Sum of calories: {}", sum);
    }
}
