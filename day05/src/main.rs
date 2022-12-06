use std::fs;

fn main() {
    let filepath = "inputfile";
    let file_content: String = fs::read_to_string(filepath).expect("Unable to read file");

    let sum: i32 = file_content.split(|c| c == '\n').filter(|&x| !x.is_empty()).map(|s| {
        0
    }).sum();
}
