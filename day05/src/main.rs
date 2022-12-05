use std::fs;

#[derive(Debug)]
struct Storage {
    stacks: Vec<Vec<char>>
}

// Example:
//
// ``` 
//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 
// ```
fn create_storage_from_string(input: &str) -> Storage {
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut lines_in_reverse = lines.iter().rev();

    // Consume first line, with the digits. And use it to determine how many stacks there are.
    let header = lines_in_reverse.next();
    let number_of_stacks = (header.unwrap().chars().count() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..number_of_stacks {
        stacks.push(vec![]);
    }
    
    // Now read stack from bottom to top.
    for line in lines_in_reverse {
        for i in 0..number_of_stacks {
            let start_of_char = i * 4 + 1;
            let container_letter = line.chars().nth(start_of_char).unwrap();
            if container_letter == ' ' {
                continue;
            }

            stacks[i].push(container_letter);
        }
    }

    return Storage {
        stacks: stacks,
    };
}

fn apply_move(storage: &mut Storage, instruction: &Instruction) {
    for _ in 0..instruction.amount {
        let take = storage.stacks[instruction.from].pop().unwrap();
        storage.stacks[instruction.to].push(take);
    }
}

fn apply_move_retain_order(storage: &mut Storage, instruction: &Instruction) {
    for i in (0..instruction.amount).rev() {
        let size = &storage.stacks[instruction.from].len();
        let take = storage.stacks[instruction.from].remove(size - 1 - i);
        storage.stacks[instruction.to].push(take);
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

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
    for instruction in instructions {
        apply_move(&mut storage, &instruction);
    }

    let result: String = (0..storage.stacks.len())
        .map(|index| storage.stacks[index].last().unwrap())
        .collect();

    println!("If we use the CrateMover 9000, the top container of each stack formatted as a string is {:?}", result);


    // Part 2
    let mut storage = create_storage_from_string(parts[0]);
    let instructions: Vec<_> = parts[1]
        .split("\n")
        .map(|line| create_instruction_from_string(line))
        .collect();

    for instruction in instructions {
        apply_move_retain_order(&mut storage, &instruction);
    }

    let result: String = (0..storage.stacks.len())
        .map(|index| storage.stacks[index].last().unwrap())
        .collect();        

    println!("If we use the CrateMover 9001, the top container of each stack formatted as a string is {:?}", result);
}
