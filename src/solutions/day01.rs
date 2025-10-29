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

    fn part2(&self, input: &str) -> Result<String> {
        let mut floor = 0;
        for (index, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor < 0 {
                return Ok((index + 1).to_string());
            }
        }
        Ok("Never reached the basement".to_string())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1_empty() {
        assert_eq!(Day1.part1("").unwrap(), "0");
    }

    #[test]
    fn test_part1() {
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

    #[test]
    fn test_part2() {
        assert_eq!(Day1.part2(")").unwrap(), "1");
        assert_eq!(Day1.part2("()())").unwrap(), "5");
    }

    #[test]
    fn test_part2_never_basement() {
        assert_eq!(Day1.part2("(((").unwrap(), "Never reached the basement");
    }
}