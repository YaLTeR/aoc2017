use itertools::Itertools;

fn is_valid(passphrase: &&str) -> bool {
    let words = passphrase.split_whitespace();
    let unique_words = words.clone().unique();
    words.count() == unique_words.count()
}

fn count_letters(word: &str) -> [usize; 28] {
    let mut rv = [0; 28];

    for c in word.chars() {
        rv[c as usize - 'a' as usize] += 1;
    }

    rv
}

fn is_valid_p2(passphrase: &&str) -> bool {
    let words = passphrase.split_whitespace();
    let counts = words.map(count_letters);
    let unique = counts.clone().unique();
    counts.count() == unique.count()
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/4");
    println!("{}", input.lines().filter(is_valid).count());
    println!("{}", input.lines().filter(is_valid_p2).count());
}
