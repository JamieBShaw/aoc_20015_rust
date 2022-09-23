fn main() {
    let data = include_str!("../input.txt");

    println!(
        "PART ONE ANSWER: {}", 
        data.len() - 2*(data.matches(')').count())
    );


    let mut current_floor = 0;
    for (i, val) in data.chars().enumerate() {

        if current_floor < 0 {
            println!("PART TWO ANSWER: {}", i);
            break;
       }

        match val {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }
    };

   
}
