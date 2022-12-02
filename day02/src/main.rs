use std::fs;

fn get_shape_index(m: char) -> i32 {
    match m {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("I dont know what to do, panic!"),
    }
}

fn main() {
    // A, X rock 
    // B, Y paper
    // C, Z scissors

    let filepath = "inputfile";
    let file_content: String = fs::read_to_string(filepath)
            .expect("Unable to read file");

    let all_battles: Vec<&str> = file_content.split("\n").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

    let sum: i32 = all_battles.iter().map(|s| {
        let other_shape_score = get_shape_index(s.chars().nth(0).unwrap());
        let my_shape_score = get_shape_index(s.chars().nth(2).unwrap());

        if other_shape_score == my_shape_score {
            return 3 + my_shape_score;
        }
        
        let match_score = match (other_shape_score, my_shape_score) {
            (1, 2) => 6,
            (1, 3) => 0,
            (2, 1) => 0,
            (2, 3) => 6,
            (3, 1) => 6,
            (3, 2) => 0,
            _ => panic!("Help pls")
        };

        return match_score + my_shape_score;
    }).sum();

    println!("Score: {}", sum);


    // A rock 
    // B paper
    // C scissors

    // X lose, Y draw, Z win

    let sum2: i32 = all_battles.iter().map(|s| {
        let other_shape_score = get_shape_index(s.chars().nth(0).unwrap());
        let match_outcome = get_shape_index(s.chars().nth(2).unwrap());

        if match_outcome == 2 {
            return 3 + other_shape_score;
        }
        
        let match_score = match (other_shape_score, match_outcome) {
            (1, 1) => 3,
            (1, 3) => 8,
            (2, 1) => 1,
            (2, 3) => 9,
            (3, 1) => 2,
            (3, 3) => 7,
            _ => panic!("Help pls")
        };

        return match_score;
    }).sum();

    println!("Score: {}", sum2);
}
