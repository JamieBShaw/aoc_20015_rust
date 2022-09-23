#!/bin/bash

day=${1##+(0)}
project=$(printf "day%02d" $1)
session="$AOC_SESSION"

cargo new ${project}

cd ${project}

curl -s "https://adventofcode.com/2015/day/${day}/input" --cookie "session=${session}" -o input.txt

echo -n 'fn main() {
    let data = include_str!("../input.txt").trim();
}

struct Solution {
    data: String,
}

impl Solution {
    fn new(data: String) -> Self {
        Self {
            data,
        }
    }

    fn part_one(&self) {

        println!("PART ONE ANSWER: {}", "ANSWER_HERE");
    }

    fn part_two(&self) {

        println!("PART TWO ANSWER: {}", "ANSWER_HERE");

    }
}

' > src/main.rs
