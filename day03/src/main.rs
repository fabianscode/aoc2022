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

fn get_common_item(rucksack: &str) -> char {
    let (compartment1, compartment2) = (rucksack).split_at(rucksack.len() / 2);

    for c in compartment1.chars() {
        if compartment2.contains(c) {
            return c;
        }
    }

    panic!("help");
}

fn main() {
    let filepath = "inputfile";

    let file_content: String = fs::read_to_string(filepath)
            .expect("Unable to read file");

    let rucksacks: Vec<&str> = file_content.split("\n").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

    let sum: i32 = rucksacks.iter().map(|s| {
        let common_item: char = get_common_item(s);
        return get_priority(common_item);
    }).sum();

    println!("Sum: {}", sum);

    //////////// part 2 ////////////
    
    let mut sum2: i32 = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let r1 = rucksacks[i];
        let r2 = rucksacks[i + 1];
        let r3 = rucksacks[i + 2];

        for c in r1.chars() {
            if r2.contains(c) && r3.contains(c) {
                sum2 += get_priority(c);
                break;
            }
        }
    }

    println!("Sum2: {}", sum2);
}
