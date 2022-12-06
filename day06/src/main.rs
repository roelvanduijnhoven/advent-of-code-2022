use std::fs;

fn find_start_of_packet(packet: &str) -> Option<usize> {
    const LOOK_AT_LAST: usize = 4;
    let chars = packet.chars().collect::<Vec<_>>();
    let windows = chars.windows(LOOK_AT_LAST);

    for (position, window) in windows.enumerate() {        
        let mut sorted_window = window.to_vec();
        sorted_window.sort();

        let increasing = sorted_window
            .windows(2)
            .filter(|ab| ab[0] < ab[1])
            .count();

        if increasing == LOOK_AT_LAST - 1 {
            return Some(position + LOOK_AT_LAST);
        }
    }

    return None;
}

fn main() {
    let contents = fs::read_to_string("assets/part-1.input").unwrap();

    let result = find_start_of_packet(&contents);
    match result {
        None => println!("Could not find start of packet"),
        Some(position) => println!("The first packet starts at position {:?}", position),
    }
}
