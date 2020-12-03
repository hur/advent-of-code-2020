struct Ride<'a> {
    lines: &'a Vec<Vec<char>>,
    pos: (usize, usize),
    slope: (usize, usize)
}

impl<'a> Ride<'a> {
    fn new(lines: &'a Vec<Vec<char>>, slope: (usize, usize)) -> Self {
        Ride {
            lines,
            pos: (0,0),
            slope
        }
    }

    fn tree(&self) -> bool {
        self.lines[self.pos.1][self.pos.0] == '#'
    }

    fn has_next(&self) -> bool {
        self.pos.1 + self.slope.1 < self.lines.len()
    }

    fn next(&mut self) {
        let new_x = (self.pos.0 + self.slope.0) % self.lines[0].len();
        let new_y = self.pos.1 + self.slope.1;
        self.pos = (new_x, new_y);
    }

    fn result(&mut self) -> usize {
        let mut trees: usize = 0;
        while self.has_next() {
            self.next();
            if self.tree() {
                trees += 1;
            }
        }
        trees
    }
}


fn read_input() -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let stdin = std::io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        if buffer == String::from("") {
            break;
        }

        result.push(buffer.chars().collect());
    }
    result
}



fn main() {
    let lines = read_input();

    let mut ride_1 = Ride::new(&lines, (3,1));
    let mut ride_2 = Ride::new(&lines, (1,1));
    let mut ride_3 = Ride::new(&lines, (5, 1));
    let mut ride_4 = Ride::new(&lines, (7,1));
    let mut ride_5 = Ride::new(&lines, (1,2));
    
    let ride_1_result = ride_1.result();
    println!("Part 1: {}", ride_1_result);
    
    let part2_result = ride_1_result * ride_2.result() * ride_3.result() * ride_4.result() * ride_5.result();
    println!("Part 2: {}", part2_result);
}


