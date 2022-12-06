use std::fs;

fn find_start_of_packet(packet: &str, look_at_last: usize) -> Option<usize> {
    let chars = packet.chars().collect::<Vec<_>>();
    let windows = chars.windows(look_at_last);

    for (position, window) in windows.enumerate() {        
        let mut sorted_window = window.to_vec();
        sorted_window.sort();

        let increasing = sorted_window
            .windows(2)
            .filter(|ab| ab[0] < ab[1])
            .count();

        if increasing == look_at_last - 1 {
            return Some(position + look_at_last);
        }
    }

    return None;
}

fn main() {
    let contents = fs::read_to_string("assets/part-1.input").unwrap();

    let result = find_start_of_packet(&contents, 4);
    match result {
        None => println!("Could not find start of packet"),
        Some(position) => println!("The first packet starts at position {:?}", position),
    }

    let result = find_start_of_packet(&contents, 14);
    match result {
        None => println!("Could not find start of message"),
        Some(position) => println!("The message starts at position {:?}", position),
    }
}
