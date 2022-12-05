use std::fs;

type Storage = Vec<Vec<char>>;

// Example:
//
// ``` 
//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 
// ```
fn create_storage_from_string(input: &str) -> Storage {
    let mut storage: Storage = vec![];
    
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut lines_in_reverse = lines.iter().rev();

    // Consume first line, with the digits. And use it to determine how many stacks there are.
    let header = lines_in_reverse.next();
    let number_of_stacks = (header.unwrap().chars().count() + 1) / 4;
    for _ in 0..number_of_stacks {
        storage.push(vec![]);
    }
    
    // Now read stack from bottom to top.
    for line in lines_in_reverse {
        for i in 0..number_of_stacks {
            let start_of_char = i * 4 + 1;
            let container_letter = line.chars().nth(start_of_char).unwrap();
            if container_letter == ' ' {
                continue;
            }

            storage[i].push(container_letter);
        }
    }

    return storage;
}

fn apply_move(storage: &mut Storage, instruction: &Instruction) {
    for _ in 0..instruction.amount {
        let at_top = storage[instruction.from].pop().unwrap();
        storage[instruction.to].push(at_top);
    }
}

fn apply_move_retain_order(storage: &mut Storage, instruction: &Instruction) {
    for i in (0..instruction.amount).rev() {
        let size = &storage[instruction.from].len();
        let at_top = storage[instruction.from].remove(size - 1 - i);
        storage[instruction.to].push(at_top);
    }
}

fn get_containers_at_top(storage: &Storage) -> String {
    return (0..storage.len())
        .map(|index| storage[index].last().unwrap_or(&' '))
        .collect();
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

// Example would be `move 3 from 7 to 4`.
fn create_instruction_from_string(line: &str) -> Instruction {
    let parts: Vec<_> = line.split(' ').collect();
    Instruction {
        amount: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap() - 1,
        to: parts[5].parse::<usize>().unwrap() - 1,
    }
}

fn main() {
    let contents = fs::read_to_string("assets/part-1.input").unwrap();
    let parts: Vec<_> = contents.split("\n\n").collect();    

    let instructions: Vec<_> = parts[1]
        .split("\n")
        .map(|line| create_instruction_from_string(line))
        .collect();

    // Part 1
    let mut storage = create_storage_from_string(parts[0]);
    for instruction in &instructions {
        apply_move(&mut storage, instruction);
    }

    println!("If we use the CrateMover 9000, the top container of each stack formatted as a string is {:?}", get_containers_at_top(&storage));

    // Part 2
    let mut storage = create_storage_from_string(parts[0]);
    for instruction in &instructions {
        apply_move_retain_order(&mut storage, instruction);
    }

    println!("If we use the CrateMover 9001, the top container of each stack formatted as a string is {:?}", get_containers_at_top(&storage));
}
