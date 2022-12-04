use std::fs;

struct Assignment {
    from: usize,
    up_to: usize,
}

fn create_assignment_from(input: &str) -> Assignment {
    let parts: Vec<_> = input.split("-").collect();
    Assignment { 
        from: parts[0].parse::<usize>().unwrap(), 
        up_to: parts[1].parse::<usize>().unwrap(),
    }
}

fn assignment_contained_in(input: &Assignment, target: &Assignment) -> bool {
    return input.from >= target.from && input.up_to <= target.up_to;
}

fn assignment_overlaps(a: &Assignment, b: &Assignment) -> bool {
    return // a:     ( 3 4 5 )
           // b: ( 1 2 3 4 )
           (a.up_to >= b.from && a.from <= b.up_to)
           
           // a: ( 1 2 3 4 )           
           // b:     ( 3 4 5 )
        || (b.up_to >= a.from && b.from <= a.up_to);
}

fn main() {
    let contents = fs::read_to_string("assets/part-1.input").unwrap();

    let pairs: Vec<_> = contents
        .split("\n")
        .map(|line| {
            let parts: Vec<_> = line.split(",").collect();
            (create_assignment_from(parts[0]), create_assignment_from(parts[1]))
        })
        .collect();

    // Part 1
    let results = pairs
        .iter()
        .filter(|(a, b)| assignment_contained_in(&a, &b) || assignment_contained_in(&b, &a))
        .count();

    println!("There are {:?} pairs where one fully overlaps the other.", results);

    // Part 2
    let results = pairs
        .iter()
        .filter(|(a, b)| assignment_overlaps(&a, &b))
        .count();

    println!("There are {:?} pairs where one overlaps the other.", results);    
}
