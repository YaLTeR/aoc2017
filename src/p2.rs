use std::cmp;
use std::str::FromStr;

fn difference(line: &str) -> u32 {
    let numbers = line.split_whitespace().map(|s| u32::from_str(s).unwrap());
    let (min, max) = numbers.fold((u32::max_value(), u32::min_value()), |(min, max), value| {
        (cmp::min(min, value), cmp::max(max, value))
    });
    max - min
}

fn division(line: &str) -> u32 {
    let numbers: Vec<_> = line.split_whitespace().map(|s| u32::from_str(s).unwrap())
                              .collect();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];

            let min = cmp::min(a, b);
            let max = cmp::max(a, b);

            if max % min == 0 {
                return max / min;
            }
        }
    }

    unreachable!();
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/2");

    let rows = input.lines();

    let result: u32 = rows.clone().map(difference).sum();
    println!("{}", result);

    let result: u32 = rows.map(division).sum();
    println!("{}", result);
}
