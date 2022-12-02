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
    // Parse input
    let contents = fs::read_to_string("input/part-1.input").unwrap();
    let baggages = contents
        .split("\n\n")
        .map(|input| create_from_lines(input))
        .collect::<Vec<Baggage>>();

    {
        // Part 1:
        let mut highest: Option<usize> = None;
        for baggage in &baggages {        
            let sum_of_calories = sum_of(baggage);
            highest = match highest {
                None => Some(sum_of_calories),
                Some(highest_value) if sum_of_calories > highest_value => Some(sum_of_calories),
                Some(x) => Some(x),
            };
        }

        println!("The top elf carries {:?} of calories.", highest.unwrap());
    }

    {
        // Part 2:
        let mut top = baggages
            .iter()
            .map(|baggage| sum_of(&baggage))
            .collect::<Vec<usize>>();
            
        top.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let sum_of_top_3: usize = top.iter().take(3).sum();

        println!("The top 3 elfs together carry {:?} of calories with them.", sum_of_top_3);
    }   
}
