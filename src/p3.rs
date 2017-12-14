/// Each spiral is always around a square.
/// n = 1, 2, 3, ...
/// Side: 3, 5, 7, ...
/// Number of squares: side * 4 - 4
/// First square in a spiral is right above the bottom right corner.
/// Coordinates (offset from center, x points to the right, y upwards): (n, -(n - 1)).

fn get_coords(input: isize) -> (isize, isize) {
    let side = |n| n * 2 + 1;
    let squares = |n| side(n) * 4 - 4;
    let first_square = (1..).scan(2, |state, n| {
        *state += squares(n - 1);
        Some(*state)
    });

    let (our_ring, first) = first_square.take_while(|v| *v <= input)
                                        .enumerate()
                                        .last()
                                        .map(|(r, s)| (r as isize + 1, s))
                                        .unwrap();

    let squares_per_side = squares(our_ring) / 4;
    let (square, initial, direction) = match (input - first) / squares_per_side {
        0 => {
            println!("Right side");
            (first, (our_ring, -(our_ring - 1)), (0, 1))
        }

        1 => {
            println!("Top side");
            (first + squares_per_side, (our_ring - 1, our_ring), (-1, 0))
        }

        2 => {
            println!("Left side");
            (first + 2 * squares_per_side, (-our_ring, our_ring - 1), (0, -1))
        }

        3 => {
            println!("Bottom side");
            (first + 3 * squares_per_side, (-(our_ring - 1), -our_ring), (1, 0))
        }

        _ => unreachable!(),
    };

    let (x, y) = (square..).take_while(|s| *s != input)
                           .scan(initial, |state, _| {
                               *state = (state.0 + direction.0, state.1 + direction.1);
                               Some(*state)
                           })
                           .last()
                           .unwrap_or(initial);

    (x, y)
}

#[allow(unused)]
pub fn run() {
    let input = 368078;
    let (x, y) = get_coords(input);
    println!("{}", x.abs() + y.abs());

    // Part 2
    const MEM_SIZE: usize = 20;
    let mut memory = Vec::with_capacity(MEM_SIZE * MEM_SIZE);
    memory.resize(MEM_SIZE * MEM_SIZE, 0);

    const CENTER: (usize, usize) = (5, 5);
    let mem = |x: isize, y: isize| (CENTER.1 + y as usize) * MEM_SIZE + CENTER.0 + x as usize;
    memory[mem(0, 0)] = 1;

    let mut count = 2;
    let mut max_value = 1;
    loop {
        let (mut x, mut y) = get_coords(count);
        max_value = iproduct!(-1..2, -1..2).filter(|&(dx, dy)| dx != 0 || dy != 0)
                                           .map(|(dx, dy)| memory[mem(x + dx, y + dy)])
                                           .sum();

        if max_value > input {
            break;
        }

        memory[mem(x, y)] = max_value;
        count += 1;
    }

    println!("{}", max_value);
}
