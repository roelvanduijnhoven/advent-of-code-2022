use std::fs;

struct Rucksack {
    front: Vec<char>,
    back: Vec<char>,
}

fn main() {
    let contents = fs::read_to_string("input/part-1.input").unwrap();
    let lines = contents.split("\n");

    let rucksacks = lines
        .map(|line| create_rucksack_from_string(line));

    let sum: u32 = rucksacks
        .map(|rucksack: Rucksack| {
            for char in rucksack.front {
                if rucksack.back.contains(&char) {
                    return char;
                }
            }
            panic!("Could not parse");
        })
        .map(|char| {
            let char = char as u32;
            let char_a = 'a' as u32;
            let char_capitcal_a = 'A' as u32;

            if char >= char_a {
                return char - char_a + 1;
            } else {
                return char - char_capitcal_a + 27;
            }
        })
        .sum();

    println!("{:?}", sum);
}

fn create_rucksack_from_string(input: &str) -> Rucksack {
    assert!(input.len() % 2 == 0);
    let half = input.len() / 2;
    Rucksack {
        front: input[0..half].chars().collect(),
        back: input[half..].chars().collect(),
    }
}