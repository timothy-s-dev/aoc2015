use crate::solutions::Solution;
use anyhow::Result;

#[derive(Debug)]
struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

impl From<&str> for PresentBox {
    fn from(s: &str) -> Self {
        let dims: Vec<u32> = s
            .split('x')
            .map(|dim| dim.parse::<u32>().unwrap())
            .collect();
        PresentBox {
            length: dims[0],
            width: dims[1],
            height: dims[2],
        }
    }
}

impl PresentBox {
    fn surface_area(&self) -> u32 {
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }

    fn smallest_side_area(&self) -> u32 {
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.height * self.length;
        *[side1, side2, side3].iter().min().unwrap()
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn longest_side(&self) -> u32 {
        *[self.length, self.width, self.height].iter().max().unwrap()
    }

    fn smallest_perimeter(&self) -> u32 {
        2 * (self.length + self.width + self.height - self.longest_side())
    }
}

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut total_paper = 0;
        for line in input.lines() {
            let present_box = PresentBox::from(line);
            total_paper += present_box.surface_area() + present_box.smallest_side_area();
        }
        Ok(total_paper.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut total_ribbon = 0;
        for line in input.lines() {
            let present_box = PresentBox::from(line);
            total_ribbon += present_box.volume() + present_box.smallest_perimeter();
        }
        Ok(total_ribbon.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_surface_area() {
        let box1 = PresentBox::from("2x3x4");
        assert_eq!(box1.surface_area(), 52);

        let box2 = PresentBox::from("1x1x10");
        assert_eq!(box2.surface_area(), 42);
    }

    #[test]
    fn test_smallest_side() {
        let box1 = PresentBox::from("2x3x4");
        assert_eq!(box1.smallest_side_area(), 6);

        let box2 = PresentBox::from("1x1x10");
        assert_eq!(box2.smallest_side_area(), 1);
    }

    #[test]
    fn test_volume() {
        let box1 = PresentBox::from("2x3x4");
        assert_eq!(box1.volume(), 24);

        let box2 = PresentBox::from("1x1x10");
        assert_eq!(box2.volume(), 10);
    }

    #[test]
    fn test_smallest_perimeter() {
        let box1 = PresentBox::from("2x3x4");
        assert_eq!(box1.smallest_perimeter(), 10);

        let box2 = PresentBox::from("1x1x10");
        assert_eq!(box2.smallest_perimeter(), 4);
    }
}