use std::ops::BitXor;

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/10");
    // let input = "3,4,1,5";
    let input: Vec<usize> = input.trim()
                                 .split(',')
                                 .map(str::parse)
                                 .map(Result::unwrap)
                                 .collect();

    let mut ring: Vec<_> = (0..256).collect();
    // let mut ring: Vec<_> = (0..5).collect();
    let mut current_pos = 0;
    let mut skip = 0;

    for &length in &input {
        // println!("{:?}", ring);
        ring = {
            let iter = ring.iter().chain(ring.iter()).skip(current_pos);

            let chain: Vec<_> = iter.clone().take(length).collect();
            let rev = chain.into_iter().rev();

            let rest = iter.skip(length).take(ring.len() - length);

            let new = rev.chain(rest);
            let new_clone = new.clone();
            let new_ring = new.chain(new_clone).skip(ring.len() - current_pos)
                              .take(ring.len())
                              .cloned()
                              .collect();

            current_pos += length + skip;
            current_pos %= ring.len();
            skip += 1;

            new_ring
        };
    }
    // println!("{:?}", ring);

    println!("{}", ring[0] * ring[1]);

    // Part 2
    let input = include_str!("../input/10");
    let input: Vec<_> = input.trim()
                             .bytes()
                             .map(usize::from)
                             .chain([17, 31, 73, 47, 23].iter().cloned())
                             .collect();

    let mut ring: Vec<_> = (0..256).collect();
    let mut current_pos = 0;
    let mut skip = 0;

    for _round in 0..64 {
        for &length in &input {
            ring = {
                let iter = ring.iter().chain(ring.iter()).skip(current_pos);

                let chain: Vec<_> = iter.clone().take(length).collect();
                let rev = chain.into_iter().rev();

                let rest = iter.skip(length).take(ring.len() - length);

                let new = rev.chain(rest);
                let new_clone = new.clone();
                let new_ring = new.chain(new_clone).skip(ring.len() - current_pos)
                                  .take(ring.len())
                                  .cloned()
                                  .collect();

                current_pos += length + skip;
                current_pos %= ring.len();
                skip += 1;

                new_ring
            };
        }
    }

    let hash = ring.chunks(16).map(|c| c.iter().fold(0, usize::bitxor));
    for value in hash {
        print!("{:02x}", value);
    }

    println!("");
}
