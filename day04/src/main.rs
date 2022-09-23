use std::i32;
use std::io::{Error, ErrorKind};

fn main() {
    let data = include_str!("../input.txt").trim();
    let solution = Solution::new(data.to_string());

    solution.part_one();
    solution.part_two();
}

struct Solution {
    data: String,
}

impl Solution {
    fn new(data: String) -> Self {
        Self { data }
    }

    fn part_one(&self) {
        if let Ok(i) = self.compute("00000") {
            println!("PART ONE ANSWER: {}", i);
        }
    }

    fn part_two(&self) {
        if let Ok(i) = self.compute("000000") {
            println!("PART TWO ANSWER: {}", i);
        }
    }

    fn compute(&self, num_of_zeros: &str) -> Result<i32, Error> {
        for i in 1.. {
            let digest = md5::compute(format!("{0}{i}", self.data));
            let str_digest = format!("{:x}", digest);
            if str_digest.starts_with(num_of_zeros) {
                return Ok(i);
            }

            if i == i32::MAX {
                break;
            }
        }

        Err(Error::new(ErrorKind::Other, "No solution for given input"))
    }
}
