use std::io::{self, BufRead};

fn seat_id(seat: &str) -> usize {
    let seat_id = seat
        .replace(|c| c == 'F' || c == 'L', "0")
        .replace(|c| c == 'B' || c == 'R', "1");
    usize::from_str_radix(&seat_id, 2).unwrap()
}

fn part2(seats: &Vec<usize>) -> Option<usize> {
    // Originally did a for loop with i[i], i[i+1] but windows() & find_map is nicer
    seats.windows(2).find_map(|x| if x[0] != x[1] - 1 {Some(x[0] + 1)} else {None})
}


fn main() {
    let mut seats: Vec<usize> = io::stdin().lock().lines()
        .map(|x| x.unwrap())
        .map(|x| seat_id(&x))
        .collect();
    seats.sort();
    
    println!("Part 1: {}", seats[seats.len() - 1]);

    println!("Part 2: {}", part2(&seats).unwrap());
}
