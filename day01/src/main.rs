use std::fs;

struct Baggage {
    items: Vec<usize>,
}

fn create_from_lines(input: &str) -> Baggage {
    Baggage {
        items: input
            .split("\n")
            .map(|item| item.parse::<usize>().unwrap())
            .collect(),
    }
}

fn sum_of(baggage: &Baggage) -> usize {
    baggage.items.iter().sum()
}

fn main() {
    let contents = fs::read_to_string("input/part-1.input").unwrap();

    let baggages = contents
        .split("\n\n")
        .map(|input| create_from_lines(input))
        .collect::<Vec<Baggage>>();

    let mut highest: Option<Baggage> = None;
    for baggage in baggages {        
        highest = match highest {
            None => Some(baggage),
            Some(highest_value) if sum_of(&baggage) > sum_of(&highest_value) => Some(baggage),
            Some(x) => Some(x),
        };
    }

    print!("Top result has sum {:?}", sum_of(&highest.unwrap()));
}
