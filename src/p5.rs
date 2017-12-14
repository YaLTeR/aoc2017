use std::str::FromStr;

fn solve(input: &mut [isize], part_two: bool) {
    let mut line = 0isize;
    let mut steps = 0;
    loop {
        steps += 1;

        {
            let jump = &mut input[line as usize];
            line += *jump;

            if part_two && *jump >= 3 {
                *jump -= 1;
            } else {
                *jump += 1;
            }
        }

        if line < 0 || line as usize >= input.len() {
            break;
        }
    }

    println!("{}", steps);
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/5");
    let mut input: Vec<_> = input.lines().map(|l| isize::from_str(l).unwrap()).collect();
    let mut input_2 = input.clone();

    solve(&mut input, false);
    solve(&mut input_2, true);
}
