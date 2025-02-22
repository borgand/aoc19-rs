advent_of_code::solution!(1);

pub fn fuel(mass: u64) -> u64 {
    let mut f = (mass / 3).saturating_sub(2); // subsract respecting bounds
    if f > 0 {
        f + fuel(f)
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // split input by lines and parse u64 numbers into variable modules
    let modules: Vec<u64> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // reduce modules vector into sum of fuel requirements
    let fuel: u64 = modules.iter().map(|&m| m / 3 - 2).sum();
    Some(fuel)
}

pub fn part_two(input: &str) -> Option<u64> {
   let modules: Vec<u64> = input
       .lines()
       .map(|line| line.parse().unwrap())
       .collect();
    let fuel: u64 = modules.iter().map(|&m| fuel(m)).sum();
    Some(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(part_one("12\n14\n1969\n100756\n"), Some(2+2+654+33583));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("14\n1969\n100756\n"), Some(2+966+50346));
    }
}
