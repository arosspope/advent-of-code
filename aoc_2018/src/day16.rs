//Day 16: Chronal Classification
//
use crate::day16::Opcode::*;
use core::{slice::Iter};
use std::collections::{HashMap};

#[derive(Debug)]
pub struct Sample {
    instruction: Vec<usize>,
    before: Vec<usize>,
    after: Vec<usize>
}

#[derive(Hash, PartialEq, Eq, Debug)]
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
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn op(&self, i: &[usize], r: &[usize]) -> Option<Vec<usize>> {
        let mut o = r.clone().to_vec();
        match self {
            Opcode::Addr => o[i[3]] = r.get(i[1])? + r.get(i[2])?,
            Opcode::Addi => o[i[3]] = r.get(i[1])? + i[2],
            Opcode::Mulr => o[i[3]] = r.get(i[1])? * r.get(i[2])?,
            Opcode::Muli => o[i[3]] = r.get(i[1])? * i[2],
            Opcode::Banr => o[i[3]] = r.get(i[1])? & r.get(i[2])?,
            Opcode::Bani => o[i[3]] = r.get(i[1])? & i[2],
            Opcode::Borr => o[i[3]] = r.get(i[1])? | r.get(i[2])?,
            Opcode::Bori => o[i[3]] = r.get(i[1])? | i[2],
            Opcode::Setr => o[i[3]] = *r.get(i[1])?,
            Opcode::Seti => o[i[3]] = i[1],
            Opcode::Gtir => o[i[3]] = (i[1] > *r.get(i[2])?) as usize,
            Opcode::Gtri => o[i[3]] = (r.get(i[1])? > &i[2]) as usize,
            Opcode::Gtrr => o[i[3]] = (r.get(i[1])? > r.get(i[2])?) as usize,
            Opcode::Eqir => o[i[3]] = (i[1] == *r.get(i[2])?) as usize,
            Opcode::Eqri => o[i[3]] = (r.get(i[1])? == &i[2]) as usize,
            Opcode::Eqrr => o[i[3]] = (r.get(i[1])? == r.get(i[2])?) as usize,
        }

        Some(o)
    }

    pub fn opcodes() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir,
            Gtri, Gtrr, Eqir, Eqri, Eqrr,
        ];
        OPCODES.into_iter()
    }
}

pub fn parse_test_program(input: &str) -> Vec<Vec<usize>> {
    let mut test_program: Vec<Vec<usize>> = Vec::new();
    
    let program_start = input.find("\n\n\n").unwrap();
    let start = &input[(program_start + 4)..];
    
    for line in start.lines() {
        let instructions = line
            .trim()
            .split(' ')
            .flat_map(str::parse::<usize>)
            .collect();
            
        test_program.push(instructions);
    }

    test_program
}

#[aoc_generator(day16, part1)]
pub fn input_samples(input: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut lines = input.lines();

    loop {
        if let Some(line) = lines.next() {
            if line.contains("Before") {
                let before: Vec<usize> = line[7..]
                    .replace(&['[', ']'][..], "")
                    .split(',')
                    .map(|s| s.trim())
                    .flat_map(str::parse::<usize>)
                    .collect();

                let instruction: Vec<usize> = lines
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .flat_map(str::parse::<usize>)
                    .collect();

                let after_line = &lines.next().unwrap()[6..];
                let after: Vec<usize> = after_line
                    .replace(&['[', ']'][..], "")
                    .split(',')
                    .map(|s| s.trim())
                    .flat_map(str::parse::<usize>)
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
pub fn part1(samples: &[Sample]) -> usize {
    let mut three_or_more = 0;

    //Iterate over each sample and test each opcode against it
    for s in samples.iter() {
        let mut i = 0;
        for oc in Opcode::opcodes() {
            if let Some(result) = oc.op(&s.instruction, &s.before) {
                if result == s.after {
                    i += 1;
                }
            }
        }

        if i >= 3 {
            three_or_more += 1;
        }
    }

    three_or_more
}

fn is_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let samples = input_samples(input);
    let test_program = parse_test_program(input);
    let mut opcode_guesses: HashMap<&Opcode, Vec<usize>> = HashMap::new();
    
    // Parse the samples, and record the guesses
    for s in samples.iter() {
        for oc in Opcode::opcodes() {
            if let Some(result) = oc.op(&s.instruction, &s.before) {
                if result == s.after {
                    // Push the instruction code into the guess hashmap for this opcode
                    opcode_guesses.entry(oc).or_insert(Vec::new()).push(s.instruction[0]);
                }
            }
        }
    }
    
    // Using the above guesses, resolve the opcode id to an opcode operation
    //
    let mut opcode_lookup: HashMap<usize, &Opcode> = HashMap::new();
    loop {
        if opcode_lookup.keys().len() == Opcode::opcodes().len() {
            // Once we've resolved all the opcodes stop guessing
            break;
        }
        
        let mut to_remove: Option<(&Opcode, usize)> = None;
        for (op, guesses) in opcode_guesses.iter() {
            if is_all_same(guesses) {
                // Add guess to lookup
                opcode_lookup.insert(guesses[0], op);
                    
                // Remove guess from all other entries  
                to_remove = Some((op, guesses[0]));    
                break;
            }
        }

        if let Some((op, remove_guess)) = to_remove {
            opcode_guesses.remove(op); // Remove opcode from the list to guess
            
            // And remove the opcode id from all the other guesses
            opcode_guesses = opcode_guesses.iter()
                .map(|(&k, v)| (k, v
                    .iter()
                    .filter(|&&g| g != remove_guess)
                    .map(|&g| g)
                    .collect::<Vec<usize>>()
                ))
                .collect();
        } else {
            panic!("Out of guesses");
        }
    }
    
    // Using the new knowledge of the opcode ids, lets
    // evaluate the test program
    //
    let mut registers = vec![0, 0, 0, 0];
    for instruction in test_program {
        let oc = opcode_lookup.get(&instruction[0]).unwrap();
        if let Some(result) = oc.op(&instruction, &registers){
            registers = result;
        }
    }
    
    registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Before: [3, 2, 1, 1]\n\
                             9 2 1 2\n\
                             After:  [3, 2, 2, 1]\n\n\
                             Before: [3, 2, 1, 1]\n\
                             15 2 1 2\n\
                             After:  [3, 2, 2, 1]\n\n\n1 3 2 1\n2 9 8 1";

    #[test]
    fn grok_input() {
        let samples = input_samples(TEST_STR);
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].before, vec![3, 2, 1, 1]);
        assert_eq!(samples[0].instruction, vec![9, 2, 1, 2]);
        assert_eq!(samples[0].after, vec![3, 2, 2, 1]);

        assert_eq!(samples[1].before, vec![3, 2, 1, 1]);
        assert_eq!(samples[1].instruction, vec![15, 2, 1, 2]);
        assert_eq!(samples[1].after, vec![3, 2, 2, 1]);
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_samples(TEST_STR)), 2);
    }
    
    #[test]
    fn unique_vectors() {
        assert!(is_all_same(&vec![1, 1, 1, 1, 1]));
        assert!(!is_all_same(&vec![1, 1, 1, 1, 2]));
    }

    #[test]
    fn operations(){
        let base = vec![3, 2, 2, 1];
        let ins = vec![9, 2, 1, 0];

        assert_eq!(Opcode::Addi.op(&ins, &base).unwrap(), vec![3, 2, 2, 1]);
        assert_eq!(Opcode::Addr.op(&ins, &base).unwrap(), vec![4, 2, 2, 1]);

        assert_eq!(Opcode::Mulr.op(&ins, &base).unwrap(), vec![4, 2, 2, 1]);
        assert_eq!(Opcode::Muli.op(&ins, &base).unwrap(), vec![2, 2, 2, 1]);

        assert_eq!(Opcode::Banr.op(&ins, &base).unwrap(), vec![2, 2, 2, 1]);
        assert_eq!(Opcode::Bani.op(&ins, &base).unwrap(), vec![0, 2, 2, 1]);

        assert_eq!(Opcode::Borr.op(&ins, &base).unwrap(), vec![2, 2, 2, 1]);
        assert_eq!(Opcode::Bori.op(&ins, &base).unwrap(), vec![3, 2, 2, 1]);

        assert_eq!(Opcode::Setr.op(&ins, &base).unwrap(), vec![2, 2, 2, 1]);
        assert_eq!(Opcode::Seti.op(&ins, &base).unwrap(), vec![2, 2, 2, 1]);

        assert_eq!(Opcode::Gtir.op(&ins, &base).unwrap(), vec![0, 2, 2, 1]);
        assert_eq!(Opcode::Gtri.op(&ins, &base).unwrap(), vec![1, 2, 2, 1]);
        assert_eq!(Opcode::Gtrr.op(&ins, &base).unwrap(), vec![0, 2, 2, 1]);

        assert_eq!(Opcode::Eqir.op(&ins, &base).unwrap(), vec![1, 2, 2, 1]);
        assert_eq!(Opcode::Eqri.op(&ins, &base).unwrap(), vec![0, 2, 2, 1]);
        assert_eq!(Opcode::Eqrr.op(&ins, &base).unwrap(), vec![1, 2, 2, 1]);
    }
}
