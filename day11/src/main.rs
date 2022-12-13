use std::{fs, ops::{Div, Rem}};

#[derive(Debug)]
struct Monkey {
    id: usize,
    divisable_test: usize,
    true_branch: usize,
    false_branch: usize,
    items: Vec<isize>,
    inspected_items: usize,
}

// TODO Learn about lifetimes, so that I know what I am doing.
fn get_substring(input: &str, prefix: &str, postfix: &str) -> String {
    return input[prefix.len()..(input.len() - postfix.len())].to_string();
}

fn get_usize_after_prefix(input: &str, prefix: &str, postfix: &str) -> usize {
    return get_substring(input, prefix, postfix).parse::<usize>().unwrap();
}

fn create_monkey_from(block: &str) -> Monkey {
    let parts: Vec<_> = block.split("\n").collect();
    Monkey {
        inspected_items: 0,
        id: get_usize_after_prefix(parts[0], "Monkey ", ":"),
        divisable_test: get_usize_after_prefix(parts[3], "  Test: divisible by ", ""),
        true_branch: get_usize_after_prefix(parts[4], "    If true: throw to monkey ", ""),
        false_branch: get_usize_after_prefix(parts[5], "    If false: throw to monkey ", ""),
        items: get_substring(parts[1], "  Starting items: ", "")
            .split(",")
            .map(|part| part.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>()
     }
}

fn create_monkeys_from(input: &str) -> Vec<Monkey> {
    return input
        .split("\n\n")
        .map(|block| create_monkey_from(&block))
        .collect();
}

fn pop_item_from_monkey(monkey: &mut Monkey) -> isize {
    monkey.inspected_items += 1;
    return monkey.items.remove(0);
}

// TODO Hardcoded. Could obviously be improved.
fn do_monkey_operation(input: isize, monkey: &Monkey) -> isize {
    // The Hardcoded part of the example problem.
    // return match monkey.id {
    //     0 => input * 19,
    //     1 => input + 6,
    //     2 => input * input,
    //     3 => input + 3,
    //     _ => panic!("Unhandled monkey operations"),
    // };

    match monkey.id {
        0 => input * 13,
        1 => input + 2,
        2 => input + 1,
        3 => input + 8,
        4 => input * input,
        5 => input + 4,
        6 => input * 17,
        7 => input + 5,
        _ => panic!("Unhandled monkey operations"),
    }
}

fn monkey_turn(monkeys: &mut Vec<Monkey>, monkey_id: usize, divide_by_three: bool) {
    let gcd: isize = monkeys
        .iter()
        .map(|monkey| monkey.divisable_test as isize)
        .product();

    while monkeys[monkey_id].items.len() > 0 {
        let current_item = pop_item_from_monkey(monkeys.get_mut(monkey_id).unwrap());

        let monkey = monkeys.get(monkey_id).unwrap();
        let mut new_item = do_monkey_operation(current_item, &monkey);

        if divide_by_three {
            new_item = new_item.div(3);
        }

        // Make sure the integers don't grow out of bounds.
        new_item = new_item % gcd;

        let is_divisible = new_item.rem(monkey.divisable_test as isize) == 0;

        let monkey_recipient = if is_divisible {
            monkey.true_branch
        } else {
            monkey.false_branch
        };

        monkeys.get_mut(monkey_recipient).unwrap().items.push(new_item);
    }
}

fn do_monkey_round(monkeys: &mut Vec<Monkey>, divide_by_three: bool) {
    for i in 0..monkeys.len() {
        monkey_turn(monkeys, i, divide_by_three);
    }
}

fn get_monkey_score(monkeys: &Vec<Monkey>) -> usize {
    let mut result: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .collect();
    result.sort_by(|a, b| b.partial_cmp(&a).unwrap());
    result[0] * result[1]
}

fn main() {
    let input = fs::read_to_string("assets/problem.input").unwrap();

    // Part 1
    let mut monkeys = create_monkeys_from(&input);
    for _ in 0..20 {
        do_monkey_round(&mut monkeys, true);
    }

    println!("After 20 rounds we have a monkey score of {}", get_monkey_score(&monkeys));
    
    // Part 2
    let mut monkeys = create_monkeys_from(&input);
    for _ in 0..10_000 {
        do_monkey_round(&mut monkeys, false);
    }

    println!("After 10.000 rounds we have a monkey score of {}", get_monkey_score(&monkeys));
}
