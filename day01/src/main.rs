use std::fs;

fn main() {
    let filepath = "inputfile";

    let file_content: String = fs::read_to_string(filepath)
            .expect("Unable to read file");

    let all_elves_calories: Vec<&str> = file_content.split("\n\n").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

    let max: i32 = all_elves_calories.iter().map(|s| {
        let single_elve_calories = s.split("\n").collect::<Vec<&str>>();

        single_elve_calories.iter().filter(|&x| !x.is_empty()).map(|&cal| { 
            cal.parse::<i32>().unwrap() 
        }).sum::<i32>()
    }).max().unwrap();

    println!("Max: {}", max);
}
