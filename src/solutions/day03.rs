use crate::solutions::Solution;
use anyhow::Result;

pub struct Day3;

impl Solution for Day3 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut x = 0;
        let mut y = 0;
        let mut visited = std::collections::HashMap::new();
        visited.insert((x, y), 1);
        for c in input.chars() {
            match c {
                '>' => x += 1,
                'v' => y += 1,
                '<' => x -= 1,
                '^' => y -= 1,
                _ => (),
            }
            let entry = visited.entry((x, y)).or_insert(0);
            *entry += 1;
        }
        Ok(visited.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut agents = [
            (0, 0),
            (0, 0)
        ];
        let mut agent_id = 0;
        let mut visited = std::collections::HashMap::new();
        visited.insert((0, 0), 1);
        for c in input.chars() {
            match c {
                '>' => agents[agent_id] = (agents[agent_id].0 + 1, agents[agent_id].1),
                'v' => agents[agent_id] = (agents[agent_id].0, agents[agent_id].1 + 1),
                '<' => agents[agent_id] = (agents[agent_id].0 - 1, agents[agent_id].1),
                '^' => agents[agent_id] = (agents[agent_id].0, agents[agent_id].1 - 1),
                _ => (),
            }
            let entry = visited.entry(agents[agent_id]).or_insert(0);
            *entry += 1;
            agent_id += 1;
            if agent_id >= agents.len() { agent_id = 0; }
        }
        Ok(visited.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1_empty() {
        assert_eq!(Day3.part1("").unwrap(), "1");
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day3.part1(">").unwrap(), "2");
        assert_eq!(Day3.part1("^>v<").unwrap(), "4");
        assert_eq!(Day3.part1("^v^v^v^v^v").unwrap(), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day3.part2("^v").unwrap(), "3");
        assert_eq!(Day3.part2("^>v<").unwrap(), "3");
        assert_eq!(Day3.part2("^v^v^v^v^v").unwrap(), "11");
    }
}