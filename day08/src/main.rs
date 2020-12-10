use std::io::{self, BufRead};

fn part1(lines: &Vec<String>) -> (i32, usize) {
    let mut acc: i32 = 0;
    let mut visited = vec![false; lines.len()];
    let mut i = 0;
    while i < lines.len() {
        if visited[i] == true {
            break;
        } else {
            visited[i] = true;
        }
        // Proper signed integer handling would simplify this a lot.
        let (instruction, n): (&str, i32) = parse(&lines[i]);
        match instruction {
            "acc" => {acc += n; i+=1},
            "jmp" => if n >= 0 {i+= n as usize} else {i -= -n as usize},
            _     => i += 1
        }
    }
    (acc, i)
}

fn part2(lines: &Vec<String>) {
    // Brute force
    for i in 0..lines.len() {
        let line: Vec<&str> = lines[i].split(" ").collect();

        let mut lines_tmp = lines.clone(); 
        let r = match line[0] {
            "jmp" => String::from("nop "),
            "nop" => String::from("jmp "),
            &_ => continue
        };
        lines_tmp[i] = r + line[1];
        let (acc, i) = part1(&lines_tmp);
        if i == lines.len() {println!("Part 2: {}:{}", acc, i)}
    }
}

fn parse (line: &String) -> (&str, i32) {
    let mut iter = line.split(" ");
    let ins: &str = iter.next().unwrap();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    (ins, n)
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    
    let (acc, _) = part1(&lines);
    println!("Part 1: {}", acc);
    part2(&lines);
}