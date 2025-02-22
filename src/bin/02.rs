use advent_of_code::intcomp;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

advent_of_code::solution!(2);

static NOUN: AtomicUsize = AtomicUsize::new(12);
static VERB: AtomicUsize = AtomicUsize::new(2);
static DEBUG: AtomicBool = AtomicBool::new(false);

pub fn set_params(noun: usize, verb: usize, debug: bool) {
    // use static atomic variables with most strict thread ordering
    NOUN.store(noun, Ordering::SeqCst);
    VERB.store(verb, Ordering::SeqCst);
    DEBUG.store(debug, Ordering::SeqCst);
}


pub fn part_one(input: &str) -> Option<u64> {
    // split input into a vector of integers
    let mut data: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    // set the noun and verb values
    data[1] = NOUN.load(Ordering::SeqCst);
    data[2] = VERB.load(Ordering::SeqCst);
    let debug = DEBUG.load(Ordering::SeqCst);
    Some(intcomp(0, &mut data, debug) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    // set the noun and verb values
    let debug = DEBUG.load(Ordering::SeqCst);
    // iterate over all possible noun and verb values
    for noun in 0..100 {
        for verb in 0..100 {
            let mut data = data.clone();
            data[1] = noun;
            data[2] = verb;
            if intcomp(0, &mut data, debug) == 19690720 {
                return Some((100 * noun + verb) as u64);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        set_params(9,10,false);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3500));
    }

    #[test]
    fn test_part_two() {
        set_params(9,10,false);
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
