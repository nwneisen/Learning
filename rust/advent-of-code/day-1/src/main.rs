use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("something went wrong reading the file");
    let lines: Vec<_> = input.lines().collect();

    let mut prev = lines[0].parse::<i32>().unwrap();
    let mut count = 0;
    for (idx, line) in lines.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let curr = line.parse::<i32>().unwrap();
        if curr - prev > 0 {
            count += 1;
        }
        prev = curr;
    }

    println!("{}", count);
}
