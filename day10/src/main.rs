use core::panic;
use std::fs;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    AddX(isize),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    program_counter: usize,
    cycle_counter: usize,
    value_x: isize,

    cycles_waiting: isize,
    add_when_finished: isize,
}

fn read_program_from_string(input: &str) -> Vec<Instruction> {
    return input.split("\n")
        .map(|line| {
            if line == "noop" {
                return Instruction::Noop;
            } else if line.starts_with("addx ") {
                let amount = line["addx ".len()..].parse::<isize>().unwrap();
                return Instruction::AddX(amount);
            } else {
                panic!("Unknown program line");
            }
        })
        .collect();
}

fn create_program_from_instructions(instructions: &Vec<Instruction>) -> Program {
    return Program { 
        instructions: instructions.clone(), 
        program_counter: 0, 
        cycle_counter: 0, 
        value_x: 1, 
        cycles_waiting: 0, 
        add_when_finished: 0,
     };
}

fn run_cycle(program: &mut Program) {
    let instruction = &program.instructions[program.program_counter];

    program.cycle_counter += 1;

    if program.cycles_waiting > 0 {
        program.cycles_waiting -= 1;
        if program.cycles_waiting > 0 {
            return;
        }
    }

    program.value_x += program.add_when_finished;
    program.add_when_finished = 0;

    if let Instruction::AddX(amount) = instruction {
        program.add_when_finished = amount.clone();
        program.cycles_waiting = 2;
    }

    program.program_counter = (program.program_counter + 1) % program.instructions.len();
}

fn run_x_cycles(program: &mut Program, cycles: usize) {
    for _ in 0..cycles {
        run_cycle(program);
    }    
}

fn main() {
    let input = fs::read_to_string("assets/problem.input").unwrap();
    let instructions = read_program_from_string(&input);

    // Part 1
    let mut program = create_program_from_instructions(&instructions);

    let mut sum_of_signals = 0;
    for cycles in [20, 40, 40, 40, 40, 40] {
        run_x_cycles(&mut program, cycles);
        sum_of_signals += (program.cycle_counter as isize) * program.value_x;
    }

    println!("Sum of signal strenghts is {}", sum_of_signals);

    // Part 2
    let mut program = create_program_from_instructions(&instructions);

    for _ in 0..6 {
        for column in 0..40 {
            run_cycle(&mut program);

            if column >= program.value_x - 1 && column <= program.value_x + 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }    
}
