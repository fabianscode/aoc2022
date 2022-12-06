use std::fs;

fn main() {
    let filepath = "inputfile";
    let file_content: String = fs::read_to_string(filepath).expect("Unable to read file");

    let sum: i32 = file_content.split(|c| c == '\n').filter(|&x| !x.is_empty()).map(|s| {
        let (first_range, second_range) = s.split_once(',').unwrap();

        let (from_str0, to_str_0) = first_range.split_once('-').unwrap();
        let (from_str1, to_str_1) = second_range.split_once('-').unwrap();

        let from0 = from_str0.parse::<i32>().unwrap();
        let to0 = to_str_0.parse::<i32>().unwrap();

        let from1 = from_str1.parse::<i32>().unwrap();
        let to1 = to_str_1.parse::<i32>().unwrap();

        return if (from0 <= from1 && to0 >= to1) || (from_str1.parse::<i32>().unwrap() <= from0 && to1 >= to0) { 1 } else { 0 };
    }).sum();

    println!("Sum: {}", sum);

    let sum2: i32 = file_content.split(|c| c == '\n').filter(|&x| !x.is_empty()).map(|s| {
        let (first_range, second_range) = s.split_once(',').unwrap();

        let (from_str0, to_str_0) = first_range.split_once('-').unwrap();
        let (from_str1, to_str_1) = second_range.split_once('-').unwrap();

        let from0 = from_str0.parse::<i32>().unwrap();
        let to0 = to_str_0.parse::<i32>().unwrap();

        let from1 = from_str1.parse::<i32>().unwrap();
        let to1 = to_str_1.parse::<i32>().unwrap();

        return if (from1 <= to0 && to1 >= from0) || (from0 <= to1 && to0 >= from1) { 1 } else { 0 };
    }).sum();

    println!("Sum: {}", sum2);
}
