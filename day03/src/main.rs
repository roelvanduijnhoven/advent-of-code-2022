use std::fs;

struct Rucksack {
    front: Vec<char>,
    back: Vec<char>,
}

fn rucksack_includes(rucksack: &Rucksack, search: &char) -> bool {
    for item in all_rucksack_items(&rucksack) {
        if item == search {
            return true;
        }
    }

    return false;
}

fn all_rucksack_items(rucksack: &Rucksack) -> impl Iterator<Item = &char> {
    rucksack.front.iter().chain(rucksack.back.iter())
}

fn get_priority_of_item(item: &char) -> u32 {
    let char = item.clone() as u32;
    let char_a = 'a' as u32;
    let char_capitcal_a = 'A' as u32;

    if char >= char_a {
        return char - char_a + 1;
    } else {
        return char - char_capitcal_a + 27;
    }
}

fn main() {
    let contents = fs::read_to_string("input/part-1.input").unwrap();
    let lines = contents.split("\n");

    let rucksacks: Vec<Rucksack> = lines
        .map(|line| create_rucksack_from_string(line))
        .collect();

    // Part 1
    let sum: u32 = rucksacks
        .iter()
        .map(|rucksack: &Rucksack| {
            for char in &rucksack.front {
                if rucksack.back.contains(&char) {
                    return char;
                }
            }
            panic!("Could not parse");
        })
        .map(|item| get_priority_of_item(item))
        .sum();

    println!("The sum of the priorities that are packed unnecessary is {:?}.", sum);

    // Part 2
    let sum: u32 = rucksacks
        .chunks(3)
        .map(|chunk| {
            let a = &chunk[0];
            let b = &chunk[1];
            let c = &chunk[2];

            for item in all_rucksack_items(&a) {
                if rucksack_includes(b, &item) && rucksack_includes(c, &item) {
                    return item;
                }
            }

            panic!("Could not find!");
        })
        .map(|item| get_priority_of_item(&item))
        .sum();

    println!("The sum of the priorities that represent the groups is {:?}.", sum);
}

fn create_rucksack_from_string(input: &str) -> Rucksack {
    assert!(input.len() % 2 == 0);
    let half = input.len() / 2;
    Rucksack {
        front: input[0..half].chars().collect(),
        back: input[half..].chars().collect(),
    }
}