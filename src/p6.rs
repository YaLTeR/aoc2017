use std::collections::HashSet;
use std::str::FromStr;

fn reallocate(banks: &mut [usize]) {
    let (i, mut count) = {
        let (i, largest) = banks.iter_mut()
                                .enumerate()
                                .max_by(|&(i1, &mut v1), &(i2, &mut v2)| {
                                            if v1 == v2 {
                                                i2.cmp(&i1)
                                            } else {
                                                v1.cmp(&v2)
                                            }
                                        })
                                .unwrap();
        let count = *largest;
        *largest = 0;

        (i, count)
    };

    for j in (0..banks.len()).cycle().skip(i + 1) {
        if count == 0 {
            break;
        }
        count -= 1;

        banks[j] += 1;
    }
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/6");
    //    let input = "0 2 7 0";
    let mut banks: Vec<_> = input.split_whitespace()
                                 .map(|s| usize::from_str(s).unwrap())
                                 .collect();

    let mut part = 1;

    loop {
        let mut configurations = HashSet::new();
        configurations.insert(banks.clone());

        let mut step = 1;
        loop {
            reallocate(&mut banks);

            if !configurations.insert(banks.clone()) {
                break;
            }

            step += 1;
        }

        println!("{}", step);

        part += 1;
        if part > 2 {
            break;
        }
    }
}
