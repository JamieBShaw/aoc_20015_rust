fn main() {
    let data = include_str!("../input.txt");

    let mut total_area = 0;

    for line in data.lines() {
        let mut dims = line
            .split('x')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>();

        dims.sort();

        let (length, width, height) = (dims[0], dims[1], dims[2]);

        let paper_needed = surface_area(width, height, length);

        total_area += paper_needed;
    }

    println!("ANSWER ONE: {}", total_area);


    let mut total_feet = 0;
    for line in data.lines() {
        let mut dims = line 
            .split('x')
            .map(|d| d.parse().unwrap())
            .collect::<Vec<u32>>();


        dims.sort();

        let (length, width, height) = (dims[0], dims[1], dims[2]);

        let feet_of_ribben = length * 2 + width * 2;
        let feet_for_bow = length * width * height;
        
        total_feet += feet_of_ribben + feet_for_bow;
    }

    println!("ANSWER TWO: {}", total_feet);

}


fn surface_area(width: u32, height: u32, length: u32) -> u32 {
    (3 * length * width) + (2 * width * height) + (2 * height * length) 
}
