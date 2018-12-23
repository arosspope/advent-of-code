//Day 16: Chronal Classification
//
use crate::day16::Opcode::*;
use core::slice::Iter;

pub struct Sample {
    instruction: Vec<usize>,
    before: Vec<usize>,
    after: Vec<usize>,
}

pub trait Opperation {
    fn op(input: [usize; 4]) -> [usize; 4];
}

pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
}

impl Opcode {
    fn op(&self, i: [usize; 4], r: [usize; 4]) -> [usize; 4] {
        let mut o = r.clone();
        match self {
            Opcode::Addr => o[i[3]] = r[i[1]] + r[i[2]],
            Opcode::Addi => o[i[3]] = r[i[1]] + i[2],
            Opcode::Mulr => o[i[3]] = r[i[1]] * r[i[2]],
            Opcode::Muli => o[i[3]] = r[i[1]] * i[2],
            Opcode::Banr => o[i[3]] = r[i[1]] & r[i[2]],
            Opcode::Bani => o[i[3]] = r[i[1]] & i[2],
            Opcode::Borr => o[i[3]] = r[i[1]] | r[i[2]],
            Opcode::Bori => o[i[3]] = r[i[1]] | i[2],
            Opcode::Setr => o[i[3]] = r[i[1]],
            Opcode::Seti => o[i[3]] = i[1],
            Opcode::Gtir => o[i[3]] = (r[i[1]] > r[i[2]]) as usize,
        }
        
        o
    }
    
    pub fn opcodes() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode;  11] = [Addr,
        Addi,
        Mulr,
        Muli,
        Banr,
        Bani,
        Borr,
        Bori,
        Setr,
        Seti,
        Gtir];
        OPCODES.into_iter()
    }
}

#[aoc_generator(day16)]
pub fn input_samples(input: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut lines = input.lines();

    loop {
        if let Some(line) = lines.next() {
            if line.contains("Before") {
                let before: Vec<usize> = line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|n| n as usize)
                    .collect();
                let instruction: Vec<usize> = lines
                    .next()
                    .unwrap()
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|n| n as usize)
                    .collect();
                let after: Vec<usize> = lines
                    .next()
                    .unwrap()
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|n| n as usize)
                    .collect();

                samples.push(Sample {
                    instruction,
                    before,
                    after,
                });
            }
        } else {
            break; //We've consumed the entire string
        }
    }

    samples
}

#[aoc(day16, part1)]
pub fn part1(input: &[Sample]) -> usize {
    // let frequency: HashMap<Opcode, usize>;
    for oc in Opcode::opcodes() {
        if oc.op(input[0].instruction, input[0].before) == input[0].after {
            return 1;
        }
    }
    0
}

#[aoc(day16, part2)]
pub fn part2(input: &[Sample]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Before: [3, 2, 1, 1]\n\
                             9 2 1 2\n\
                             After:  [3, 2, 2, 1]\n\n\n1 3 2 1\n2 9 8 1";

    #[test]
    fn grok_input() {
        let samples = input_samples(TEST_STR);
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].before, vec![3, 2, 1, 1]);
        assert_eq!(samples[0].instruction, vec![9, 2, 1, 2]);
        assert_eq!(samples[0].after, vec![3, 2, 2, 1]);
    }
    
    #[test]
    fn sample1() {
        assert_eq!(part1(&input_samples(TEST_STR)), 1);
    }
}
