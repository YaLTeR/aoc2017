use std::cmp;

fn steps(x: isize, y: isize) -> usize {
    // Magic
    let x_steps = x.abs() / 2;
    let new_y = cmp::max(0, y.abs() - x.abs() / 2);
    (x_steps + (new_y / 2) + (new_y % 2)) as usize
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/11");
    // let input = "ne,nw,ne,nw,n,n";

    let (mut x, mut y) = (0isize, 0isize);
    let mut most_steps = 0;

    for dir in input.trim().split(',') {
        let (dx, dy) = match dir {
            "n" => (0, 2),
            "ne" => (2, 1),
            "se" => (2, -1),
            "s" => (0, -2),
            "sw" => (-2, -1),
            "nw" => (-2, 1),
            _ => unreachable!()
        };

        x += dx;
        y += dy;

        most_steps = cmp::max(most_steps, steps(x, y));
    }

    println!("{}", steps(x, y));
    println!("{}", most_steps);
}
