use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let input_string = include_str!("../../inputs/day02.txt");
    println!("{input_string}");
    let answer_part_1 = 0;
    let answer_part_2 = 0;
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}
