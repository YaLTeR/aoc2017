#[allow(unused)]
pub fn run() {
    let input = include_bytes!("../input/1");

    let c = |c: &&u8| **c >= '0' as u8 && **c <= '9' as u8;
    let c2 = |c| c - '0' as u8;
    let digits = input.iter().filter(&c).map(&c2);

    let mut sum = 0;
    for (a, b) in digits.clone().zip(digits.clone().cycle().skip(1)) {
        if a == b {
            sum += a as usize;
        }
    }

    println!("{}", sum);

    let mut sum = 0;
    for (a, b) in digits.clone()
                        .zip(digits.clone().cycle().skip(digits.count() / 2))
    {
        if a == b {
            sum += a as usize;
        }
    }

    println!("{}", sum);
}
