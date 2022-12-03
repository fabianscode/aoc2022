use std::fs;

fn get_priority(c: char) -> i32{
    let as_ascii_int: i32 = c.to_string().as_bytes()[0].into();

    if (97..123).contains(&as_ascii_int) {
        return as_ascii_int - 96;
    }

    if (65..91).contains(&as_ascii_int) {
        return as_ascii_int - 38;
    }

    panic!("help");
}

fn get_common_item(common_char_strings: Vec<&str>) -> char {
    let str0: &str = common_char_strings[0];

    for c in str0.chars() {
        if common_char_strings[1..].iter().all(|s| s.contains(c)) {
            return c;
        }
    }

    panic!("help");
}

fn main() {
    let filepath = "inputfile";
    let file_content: String = fs::read_to_string(filepath).expect("Unable to read file");
    let rucksacks: Vec<&str> = file_content.split("\n").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

    let sum: i32 = rucksacks.iter().map(|s| {
        let (compartment1, compartment2) = (s).split_at(s.len() / 2);
        let common_item: char = get_common_item(vec![compartment1, compartment2]);

        return get_priority(common_item);
    }).sum();

    println!("Sum: {}", sum);

    //////////// part 2 ////////////

    let sum2: i32 = rucksacks.chunks(3).map(|chunk| {
        get_priority(get_common_item(chunk.to_vec()))
    }).sum();

    println!("Sum2: {}", sum2);
}
