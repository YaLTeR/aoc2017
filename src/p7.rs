use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Tower<'a> {
    pub weight: usize,
    pub subtowers: Vec<&'a str>,
}

impl<'a> Tower<'a> {
    fn new(weight: usize, subtowers: Vec<&'a str>) -> Self {
        Tower { weight, subtowers }
    }

    fn weight(&self, programs: &HashMap<&'a str, Tower<'a>>) -> usize {
        self.weight
        + self.subtowers.iter()
              .map(|x| programs[x].weight(programs))
              .sum::<usize>()
    }
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/7");

    // Part 1
    let mut all_programs = HashSet::new();
    let mut non_base_programs = HashSet::new();

    for line in input.lines() {
        all_programs.insert(line.split_whitespace().next().unwrap());

        if let Some(non_base) = line.splitn(2, "-> ").nth(1) {
            for program in non_base.split(", ") {
                non_base_programs.insert(program);
            }
        }
    }

    let base = all_programs.difference(&non_base_programs).next().unwrap();
    println!("{}", base);

    // Part 2
    let mut programs = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let name = iter.next().unwrap();
        let weight_str = iter.next().unwrap();
        let weight = usize::from_str(&weight_str[1..weight_str.len() - 1]).unwrap();

        let subprograms = iter.skip(1).map(|s| s.split(',').next().unwrap()).collect();

        programs.insert(name, Tower::new(weight, subprograms));
    }

    let mut current = base;
    let mut result = 0;
    loop {
        // This is probably overcomplicated; not sure how to do it cleanly.
        let weights_map: HashMap<_, _> =
            programs[current].subtowers
                             .iter()
                             .map(|x| (x, programs[x].weight(&programs)))
                             .collect();

        // println!("{:#?}", weights_map);

        let mut weight_counts = HashMap::new();
        for weight in weights_map.iter().map(|(_, &w)| w) {
            *weight_counts.entry(weight).or_insert(0) += 1;
        }

        if weight_counts.iter().count() == 1 {
            break;
        }

        let weights: Vec<_> = weight_counts.iter().collect();
        // println!("{:?}", weights);

        let n = if *weights[0].1 == 1 { 0 } else { 1 };
        let k = 1 - n;

        current = weights_map.iter()
                             .filter(|&(_, w)| w == weights[n].0)
                             .map(|(k, _)| k)
                             .next()
                             .unwrap();

        // println!("current: {} {}", current, programs[current].weight);

        result = (*weights[k].0 as isize - *weights[n].0 as isize
                  + programs[current].weight as isize) as usize;
    }

    println!("{}", result);
}
