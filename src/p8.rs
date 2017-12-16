use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Op {
    Increase,
    Decrease,
}

impl Op {
    fn apply<T: AddAssign + SubAssign + Debug>(self, a: &mut T, value: T) {
        // println!("{:?} {:?} {:?}", a, self, value);

        match self {
            Op::Increase => *a += value,
            Op::Decrease => *a -= value,
        };
    }
}

#[derive(Debug)]
struct ParseOpError;

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "inc" {
            Ok(Op::Increase)
        } else if s == "dec" {
            Ok(Op::Decrease)
        } else {
            Err(ParseOpError)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cmp {
    Equal,
    NotEqual,
    GT,
    LT,
    GE,
    LE,
}

impl Cmp {
    fn apply<T: Ord + Debug>(self, a: T, b: T) -> bool {
        // println!("{:?} {:?} {:?}", a, self, b);

        match self {
            Cmp::Equal => a == b,
            Cmp::NotEqual => a != b,
            Cmp::GT => a > b,
            Cmp::LT => a < b,
            Cmp::GE => a >= b,
            Cmp::LE => a <= b,
        }
    }
}

#[derive(Debug)]
struct ParseCmpError;

impl FromStr for Cmp {
    type Err = ParseCmpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "==" {
            Ok(Cmp::Equal)
        } else if s == "!=" {
            Ok(Cmp::NotEqual)
        } else if s == ">" {
            Ok(Cmp::GT)
        } else if s == "<" {
            Ok(Cmp::LT)
        } else if s == ">=" {
            Ok(Cmp::GE)
        } else if s == "<=" {
            Ok(Cmp::LE)
        } else {
            Err(ParseCmpError)
        }
    }
}

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/8");

    let mut registers = HashMap::new();
    let highest =
        |registers: &HashMap<&str, isize>| *registers.iter().map(|(_, v)| v).max().unwrap();

    let mut overall_highest = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let register_name = iter.next().unwrap();
        let op: Op = iter.next().unwrap().parse().unwrap();
        let value = iter.next().unwrap().parse().unwrap();

        let mut iter = iter.skip(1);
        let cond_register = *registers.entry(iter.next().unwrap()).or_insert(0);
        let cmp: Cmp = iter.next().unwrap().parse().unwrap();
        let cmp_value = iter.next().unwrap().parse().unwrap();

        if cmp.apply(cond_register, cmp_value) {
            op.apply(registers.entry(register_name).or_insert(0), value);
        }

        overall_highest = cmp::max(overall_highest, highest(&registers));
    }

    println!("{}", highest(&registers));
    println!("{}", overall_highest);
}
