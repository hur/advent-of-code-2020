// Welcome to Enum hell.
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(Direction),
    Turn(Rotation),
}

impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            'N' | 'E' | 'S' | 'W' | 'F' => Command::Move(Direction::from(c)),
            'L' | 'R' => Command::Turn(Rotation::from(c)),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    AbsoluteDirection(AbsoluteDirection),
    RelativeDirection(RelativeDirection),
}

impl From<char> for Direction {
    fn from(dir: char) -> Self {
        match dir {
            'N' => Direction::AbsoluteDirection(AbsoluteDirection::North),
            'E' => Direction::AbsoluteDirection(AbsoluteDirection::East),
            'S' => Direction::AbsoluteDirection(AbsoluteDirection::South),
            'W' => Direction::AbsoluteDirection(AbsoluteDirection::West),
            'F' => Direction::RelativeDirection(RelativeDirection::Forward),
            other=> panic!("Can't make direction from {}", other)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AbsoluteDirection {
    North,
    East,
    South,
    West,
}

impl AbsoluteDirection {
    fn turn_left(&mut self) {
        match self {
            AbsoluteDirection::North => *self = AbsoluteDirection::West,
            AbsoluteDirection::East  => *self = AbsoluteDirection::North,
            AbsoluteDirection::South => *self = AbsoluteDirection::East,
            AbsoluteDirection::West  => *self = AbsoluteDirection::South
        }
    } 
    fn turn_right(&mut self) {
        match self {
            AbsoluteDirection::North => *self = AbsoluteDirection::East,
            AbsoluteDirection::East  => *self = AbsoluteDirection::South,
            AbsoluteDirection::South => *self = AbsoluteDirection::West,
            AbsoluteDirection::West  => *self = AbsoluteDirection::North
        }
    } 
}
#[derive(Debug, Clone, Copy)]
enum RelativeDirection {
    Forward,
}

// Rotations come in multiples of 90 (meaning direction is always N/E/S/W)
#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

impl From<char> for Rotation {
    fn from(dir: char) -> Self {
        match dir {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _   => unreachable!()
        }
    }
}
#[derive(Debug)]
struct Waypoint {
    x: isize,
    y: isize
}

impl Waypoint {
    fn new(x: isize, y: isize) -> Self {
        Waypoint { x,y }
    }

    fn move_absolute(&mut self, direction: AbsoluteDirection, amount: isize) {
        match direction {
            AbsoluteDirection::North  => self.y += amount,
            AbsoluteDirection::East   => self.x += amount,
            AbsoluteDirection::South  => self.y -= amount,
            AbsoluteDirection::West   => self.x -= amount,
        }
    }

    fn rotate(&mut self, way: Rotation, amount: isize) {
        let times_to_rotate = amount / 90;
        match way {
            Rotation::Left  => (0..times_to_rotate).for_each(|_| {
                std::mem::swap(&mut self.x, &mut self.y);
                self.x = -self.x;
            }), 
            Rotation::Right => (0..times_to_rotate).for_each(|_| {
                std::mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y;
            }),
        }
    }
}
#[derive(Debug)]
struct Ferry {
    x: isize,
    y: isize,
    direction: AbsoluteDirection,
    waypoint: Waypoint
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            x: 0,
            y: 0,
            direction: AbsoluteDirection::East,
            waypoint: Waypoint::new(10, 1),
        }
    }

    fn move_in_direction(&mut self, direction: Direction, amount: isize) {
        match direction {
            Direction::AbsoluteDirection(d) => self.move_absolute(d, amount),
            Direction::RelativeDirection(RelativeDirection::Forward) => self.move_absolute(self.direction, amount), 
        }
    }
    
    fn move_absolute(&mut self, direction: AbsoluteDirection, amount: isize) {
        match direction {
            AbsoluteDirection::North  => self.y += amount,
            AbsoluteDirection::East   => self.x += amount,
            AbsoluteDirection::South  => self.y -= amount,
            AbsoluteDirection::West   => self.x -= amount,
        }
    }

    fn move_towards_waypoint(&mut self, amount: isize) {
        self.x += amount * self.waypoint.x;
        self.y += amount * self.waypoint.y;
    }

    fn rotate(&mut self, way: Rotation, amount: isize) {
        let times_to_rotate = amount / 90;
        match way {
            Rotation::Left  => (0..times_to_rotate).for_each(|_| self.direction.turn_left()),
            Rotation::Right => (0..times_to_rotate).for_each(|_| self.direction.turn_right()),
        }
    }

    fn handle_command_part1(&mut self, command: Command, amount: isize) {
        match command {
            Command::Move(d) => self.move_in_direction(d, amount),
            Command::Turn(t)  => self.rotate(t, amount),
        }
    }

    fn handle_command_part2(&mut self, command: Command, amount: isize) {
        match command {
            Command::Move(Direction::AbsoluteDirection(d)) => self.waypoint.move_absolute(d, amount),
            Command::Move(Direction::RelativeDirection(Forward)) => self.move_towards_waypoint(amount),
            Command::Turn(t)=> self.waypoint.rotate(t, amount),
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn part1(lines : &Vec<(char, isize)>) {
    let mut ferry = Ferry::new();
    lines.iter().for_each(|(command, amount)| {
        let command = Command::from(*command);
        ferry.handle_command_part1(command, *amount);
    });
    println!("Part 1: Manhattan distance is {}", ferry.manhattan_distance());
}

fn part2(lines: &Vec<(char, isize)>) {
     let mut ferry = Ferry::new();
     lines.iter().for_each(|(command, amount)| {
         let command = Command::from(*command);
         ferry.handle_command_part2(command, *amount);
     });
     println!("Part 2: Manhattan distance is {}", ferry.manhattan_distance())
}

fn main() {
    let lines: Vec<(char, isize)> = io::stdin().lock().lines()
        .filter_map(Result::ok)
        .map(|line| {
            // May panic on bad input ~ there won't be any on the AoC input
            let result = line.split_at(1); 
            (result.0.chars().nth(0).unwrap(), result.1.parse::<isize>().unwrap())
        })
        .collect();
     part1(&lines);
     part2(&lines);
}
