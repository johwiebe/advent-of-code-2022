use lazy_static::lazy_static;
use regex::Regex;

type Stack = Vec<char>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
}

pub fn solve() {
    let mut stack_created: bool = false;
    let mut stack: Vec<Stack> = vec![Vec::new(); 10];

    if let Ok(lines) = crate::helpers::read_lines("./data/5.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    stack_created = true;
                } else if stack_created {
                    update_stack(&mut stack, ip)
                } else {
                    create_stack(&mut stack, ip)
                }
            }
        }
    }
    let score: String = get_score(&stack);
    println!("Score: {}", score);
}

fn create_stack(stack: &mut Vec<Stack>, line: String) {
    for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
        if chunk[0] == '[' {
            stack[i + 1].insert(0, chunk[1]);
        }
    }
}

fn update_stack(stack: &mut Vec<Stack>, line: String) {
    let caps = RE.captures(&line).unwrap();
    let num: u8 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
    let from: u8 = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
    let to: u8 = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();
    for _i in 1..=num {
        let item: char = stack[from as usize].pop().unwrap();
        stack[to as usize].push(item);
    }
}

fn get_score(stack: &Vec<Stack>) -> String {
    let mut score: String = String::new();
    for s in stack {
        if let Some(c) = s.last().copied() {
            score.push(c)
        }
    }
    score
}
