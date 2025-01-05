use color_eyre::*;
use std::collections::BinaryHeap;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        left_heap.push(parts.next().map(|s| s.parse::<i32>().unwrap()).unwrap());
        right_heap.push(parts.next().map(|s| s.parse::<i32>().unwrap()).unwrap());
    }

    let mut sum = 0;
    while let Some(left) = left_heap.pop() {
        let right = right_heap.pop().unwrap();

        sum += left.abs_diff(right);
    }

    println!("sum: {sum}");

    Ok(())
}
