use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../input.txt");
    let solution = Solution::new(data.to_string());
    solution.part_one();

    solution.part_two();
}

struct Solution {
    data: String,
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Unable to from char: {}", c),
        }
    }
}

impl Solution {
    fn new(data: String) -> Self {
        Solution { data }
    }

    fn part_one(&self) {
        let mut houses = HashSet::new();
        let mut pos = (0, 0);
        houses.insert(pos);

        for c in self.data.chars() {
            self.update_pos(c, &mut pos);
            houses.insert(pos);
        }

        println!("ANSWER PART ONE: {}", houses.len());
    }

    fn part_two(&self) {
        let mut santa_pos = (0, 0);
        let mut robot_pos = (0, 0);
        let mut current: &mut (i32, i32);
        let mut houses = HashMap::new();
        houses.insert(santa_pos, 2);

        for (i, c) in self.data.chars().enumerate() {
            current = if i % 2 == 0 {
                &mut santa_pos
            } else {
                &mut robot_pos
            };
            self.update_pos(c, current);
            houses.entry(*current).and_modify(|h| *h += 1).or_insert(1);
        }

        println!("ANSWER TWO ONE: {}", houses.len());
    }

    fn update_pos(&self, c: char, pos: &mut (i32, i32)) {
        match Move::from_char(c) {
            Move::Up => pos.1 += 1,
            Move::Down => pos.1 -= 1,
            Move::Left => pos.0 -= 1,
            Move::Right => pos.0 += 1,
        }
    }
}
