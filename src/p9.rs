#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/9");

    let mut current_score = 1;
    let mut total_score = 0;
    let mut inside_garbage = false;
    let mut skip_next = false;
    let mut non_cancelled_garbage = 0;
    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if c == '!' {
            skip_next = true;
            continue;
        }

        if inside_garbage {
            if c == '>' {
                inside_garbage = false;
                continue;
            }

            non_cancelled_garbage += 1;
            continue;
        }

        if c == '<' {
            inside_garbage = true;
            continue;
        }

        if c == '{' {
            total_score += current_score;
            current_score += 1;
        } else if c == '}' {
            current_score -= 1;
        }
    }

    println!("{}", total_score);
    println!("{}", non_cancelled_garbage);
}
