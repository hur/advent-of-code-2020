#![feature(str_split_once)] // Requires rust nightly
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;


fn part1(data: &Vec<String>) {
    let bags: HashMap<String, Vec<String>> = data.iter().map(|line| parse(&line)).collect();
    let first = String::from("shiny gold bag");
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(first);
    let mut s: HashSet<String> = HashSet::new();
    while !q.is_empty() {
        let elem = q.pop_front().unwrap();
        bags.iter().filter(|(_, v)| v.contains(&elem)).for_each(|(k, _)| {
            q.push_back(k.clone());
            s.insert(k.clone());
        });
        
    }
    println!("{}", s.len());

}

fn parse(line: &String) -> (String, Vec<String>) {
    let (bag, unparsed_contents) = line.split(" contain ").collect_tuple().unwrap();
    let mut contents: Vec<String> = Vec::new();
    for elem in unparsed_contents.split(", ") {
        let (_, target) = elem.split_once(" ").unwrap();
        contents.push(target.replace(".", "").to_string());

    }
    (bag.to_string(), contents)
     
}

fn part2(data: &Vec<String>) {
    let bags: HashMap<String, Vec<(usize, String)>> = data.iter().map(|line| parse2(&line)).collect();
    let count = recurse(&bags, &String::from("shiny gold bag"));
    // Counts the initial bag so need to remove it
    println!("{}", count - 1);
}

fn recurse(bags: &HashMap<String,Vec<(usize, String)>>, bag: &String) -> usize {
    let mut c = 1;
    if let Some(b) = bags.get(bag) {
        for (n, x) in b.iter() {
            c += recurse(bags, x) * n;
        }
    }
    c
}

fn parse2(line: &String) -> (String, Vec<(usize, String)>) {
    let (bag, unparsed_contents) = line.split(" contain ").collect_tuple().unwrap();
    let mut contents: Vec<(usize, String)> = Vec::new();
    for elem in unparsed_contents.split(", ") {
        let (n, target) = elem.split_once(" ").unwrap();
        if !n.contains("no") {
            contents.push((n.parse().unwrap(), target.replace(".", "").to_string()));
        }
    }
    (bag.to_string(), contents)
     
}


fn main() {
    let input: Vec<String> = io::stdin().lock().lines()
        .map(|x| x.unwrap().replace("bags", "bag"))
        .collect();
    part1(&input);
    part2(&input);
    
    
}
