#![feature(slice_take)]
use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt").trim();
    let solution = Solution::new(data);
    solution.part_one();
    solution.part_two();
}

struct Solution<'a> {
    data: &'a str,
}

const VOWELS: &str = "aeiou";

impl<'a> Solution<'a> {
    fn new(data: &'a str) -> Self {
        Self { data }
    }

    fn part_one(&self) {
        let mut nice = 0;
        for line in self.data.lines() {
            match SantaDecisionPartOne::from_string(line) {
                SantaDecisionPartOne::Nice => nice += 1,
                SantaDecisionPartOne::Naughty => (),
          }
        }

        println!("PART ONE ANSWER: {nice}");
    }

    fn part_two(&self) {
        let mut nice = 0;
         for line in self.data.lines() {
            match SantaDecisionPartTwo::from_string(line) {
                SantaDecisionPartTwo::Nice => nice += 1,
                SantaDecisionPartTwo::Naughty => (),
          }
        }

        println!("PART TWO ANSWER: {nice}");
    }
}
enum SantaDecisionPartTwo {
    Nice,
    Naughty
}


impl SantaDecisionPartTwo {
    fn from_string(data: &str) -> SantaDecisionPartTwo {
        if !SantaDecisionPartTwo::contains_none_overlapping_pair(data) {
            return SantaDecisionPartTwo::Naughty
        }

       // if !SantaDecisionPartTwo::contains_repeated_letter_with_letter_between(data) {
      //      return SantaDecisionPartTwo::Naughty;
      //  }


        SantaDecisionPartTwo::Nice
    }

    fn contains_none_overlapping_pair(data: &str) -> bool {
        let mut pairs_seen = HashMap::new();
        let vev_chars = data.chars().collect::<Vec<char>>();
        let windowed = vev_chars.windows(3);
        let win_len = windowed.len();

        for mut w in windowed {
            let len = w.len();
            println!("Win Len: {}", win_len);
            if len >= 2 {
                let xy: &[char] = w.take(..2).unwrap();
                if len == 3 && w[0] == xy[1] {
                    pairs_seen.remove(&(xy[0], xy[1]));
                } else {
                    pairs_seen.entry((xy[0], xy[1])).and_modify(|entry| *entry += 1).or_insert(1);
                }
            }
        }


       


        println!("pairs_seen: {:?}", pairs_seen);

        pairs_seen.values().any(|v| *v >= 2)
    

    }

    fn contains_repeated_letter_with_letter_between(data: &str) -> bool {

        true
    }


}

enum SantaDecisionPartOne {
    Nice,
    Naughty,
}

impl SantaDecisionPartOne {
    fn from_string(data: &str) -> SantaDecisionPartOne {
        if !SantaDecisionPartOne::has_atleast_three_vowels(data) {
            return SantaDecisionPartOne::Naughty;
        }

        if !SantaDecisionPartOne::contains_double_letters(data) {
            return SantaDecisionPartOne::Naughty;
        }

        if SantaDecisionPartOne::contains_string(vec!["ab", "cd", "pq", "xy"], data) {
            return SantaDecisionPartOne::Naughty;
        }

        SantaDecisionPartOne::Nice
    }

    fn has_atleast_three_vowels(data: &str) -> bool {
        let mut vowel_count = 0;
        data.chars().for_each(|c| {
            if VOWELS.contains(c) {
                vowel_count += 1;
            }
        });

        vowel_count >= 3
    }

    fn contains_double_letters(data: &str) -> bool {
        data
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|wind| wind[0] == wind[1])
    }

    fn contains_string(to_test: Vec<&str>, data: &str) -> bool {
        to_test.into_iter().any(|value_to_test| data.contains(value_to_test))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_contains_none_overlapping_pair() {
       let solutuon = SantaDecisionPartTwo::contains_none_overlapping_pair("aabcdefgaa");

       println!("Solution: {}", solutuon);
       assert!(solutuon)
    }
}
