use color_eyre::*;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        left_vec.push(parts.next().map(|s| s.parse::<i32>().unwrap()).unwrap());
        right_vec.push(parts.next().map(|s| s.parse::<i32>().unwrap()).unwrap());
    }

    let mut sum = 0;
    while let Some(left) = left_vec.pop() {
        let count = right_vec.iter().filter(|&val| *val == left).count();
        println!("{count}");
        sum += left * count as i32;
    }

    println!("sum: {sum}");

    Ok(())
}
