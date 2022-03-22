use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("something went wrong reading the file");
    let lines: Vec<_> = input.lines().collect();

    let mut prev = 0;
    let mut count = 0;
    let mut sum = 0;

    for (idx, line) in lines.iter().enumerate() {
        if idx <= 2 {
            sum += line.parse::<i32>().unwrap();
            prev = sum;
            continue;
        }

        sum += line.parse::<i32>().unwrap();
        sum -= lines[idx - 3].parse::<i32>().unwrap();

        if sum - prev > 0 {
            count += 1;
        }
        prev = sum;
    }

    println!("{}", count);
}
