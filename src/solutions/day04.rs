use crate::solutions::Solution;
use anyhow::Result;

pub struct Day4;

impl Solution for Day4 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut number = 0;
        loop {
            let input = format!("{}{}", input, number);
            let hash = format!("{:x}", md5::compute(input));
            if hash.starts_with("00000") {
                break Ok(number.to_string());
            }
            number += 1;
            if number > 100_000_000 { break Err(anyhow::anyhow!("Not found by 100M"))}
        }
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut number = 0;
        loop {
            let input = format!("{}{}", input, number);
            let hash = format!("{:x}", md5::compute(input));
            if hash.starts_with("000000") {
                break Ok(number.to_string());
            }
            number += 1;
            if number > 100_000_000 { break Err(anyhow::anyhow!("Not found by 100M"))}
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day4.part1("abcdef").unwrap(), "609043");
        assert_eq!(Day4.part1("pqrstuv").unwrap(), "1048970");
    }
}