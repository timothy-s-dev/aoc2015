use crate::solutions::Solution;
use anyhow::Result;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut floor = 0;
        for c in input.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }
        Ok(floor.to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_floor_empty() {
        assert_eq!(Day1.part1("").unwrap(), "0");
    }

    #[test]
    fn test_get_floor() {
        assert_eq!(Day1.part1("(())").unwrap(), "0");
        assert_eq!(Day1.part1("()()").unwrap(), "0");
        assert_eq!(Day1.part1("(((").unwrap(), "3");
        assert_eq!(Day1.part1("(()(()(").unwrap(), "3");
        assert_eq!(Day1.part1("))(((((").unwrap(), "3");
        assert_eq!(Day1.part1("())").unwrap(), "-1");
        assert_eq!(Day1.part1("))(").unwrap(), "-1");
        assert_eq!(Day1.part1(")))").unwrap(), "-3");
        assert_eq!(Day1.part1(")())())").unwrap(), "-3");
    }
}