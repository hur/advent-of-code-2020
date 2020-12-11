use std::io::{self, BufRead};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor
}

type Seats = Vec<Vec<Seat>>;

fn part1(data: &Seats) {
    let mut previous_state = data.clone();
    let mut new_state = data.clone();
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for row in 0..previous_state.len()  {
            for col in 0..previous_state[row].len(){
                new_state[row][col] = match previous_state[row][col] {
                    Seat::Empty if count_adjacent_occupied(&previous_state, row, col) == 0 => {
                        has_changed = true;
                        Seat::Occupied
                        
                    },
                    Seat::Occupied if count_adjacent_occupied(&previous_state, row, col) >= 4 => {
                        has_changed = true;
                        Seat::Empty
                    },
                    x => x
                }
            }
        }
        std::mem::swap(&mut previous_state, &mut new_state); 
    }

    let result: usize = previous_state
            .iter()
            .map(|row| row.iter().filter(|x| **x == Seat::Occupied).count())
            .sum();  
    println!("Part 1: {}", result);
    
}

fn count_adjacent_occupied(seats: &Seats, row: usize, col: usize) -> usize {   
    DIRECTIONS
        .iter()
        .filter(|&(x, y)| is_occupied(seats, row as isize + *x, col as isize + *y))
        .count()
}

fn is_occupied(seats: &Seats, row: isize, col: isize) -> bool {
    if row < 0 || row >= seats.len() as isize || col < 0 || col >= seats[0].len() as isize {
        return false;
    }
    match seats[row as usize][col as usize] {
        Seat::Occupied => true,
        _ => false
    }
}

// It's late and I'm tired. I'll do DRY some other day!
fn part2(data: &Seats) {
    let mut previous_state = data.clone();
    let mut new_state = data.clone();
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for row in 0..previous_state.len()  {
            for col in 0..previous_state[row].len(){
                new_state[row][col] = match previous_state[row][col] {
                    Seat::Empty if count_visible_occupied(&previous_state, row, col) == 0 => {
                        has_changed = true;
                        Seat::Occupied
                        
                    },
                    Seat::Occupied if count_visible_occupied(&previous_state, row, col) >= 5 => {
                        has_changed = true;
                        Seat::Empty
                    },
                    x => x
                }
            }
        }
        std::mem::swap(&mut previous_state, &mut new_state); 
    }
    let result: usize = previous_state
            .iter()
            .map(|row| row.iter().filter(|x| **x == Seat::Occupied).count())
            .sum();  
    println!("Part 2: {}", result);  
}

fn count_visible_occupied(seats: &Seats, row: usize, col: usize) -> usize {   
    DIRECTIONS
        .iter()
        .filter(|&(x, y)| is_visible_occupied(seats, row as isize + *x, col as isize + *y, *x ,*y))
        .count()
}

fn is_visible_occupied(seats: &Seats, row: isize, col: isize, dx: isize, dy: isize) -> bool {
    if row < 0 || row >= seats.len() as isize || col < 0 || col >= seats[0].len() as isize {
        return false;
    }
    match seats[row as usize][col as usize] {
        Seat::Occupied => true,
        Seat::Floor =>is_visible_occupied(seats, row + dx, col + dy, dx, dy),
        _ => false
    }
}

fn main() {
    let seats: Seats = io::stdin().lock().lines()
        .filter_map(Result::ok)
        .map(|string| {
            string.chars()
                .map(|c| if c == 'L' {Seat::Empty} else {Seat::Floor})
                .collect()
        })
        .collect();
        part1(&seats);
        part2(&seats);
}

