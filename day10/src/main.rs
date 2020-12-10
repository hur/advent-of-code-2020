use std::io::{self, BufRead};
use std::collections::HashMap;

fn part1(data: &[u32]) {
    // Diffs is a mapping of the magnitude of difference between two elements in data and the count
    // of the difference
    let mut diffs = HashMap::<u32,u32>::new();
    let mut prev = 0u32;
    for d in data.iter() {
        let curr_diff = d - prev;
        *diffs.entry(curr_diff).or_insert(0) += 1;
        prev = *d;
    }
    println!("{:?}", diffs);
    println!("Part 1: {}*{}={}", diffs.get(&1).unwrap(), diffs.get(&3).unwrap(), diffs.get(&1).unwrap() * diffs.get(&3).unwrap());
}

fn part2(data: &[u32]) {
    // We describe the number of ways to chain _i_ adapters (steps[i]]) 
    // as the sum of ways to chain the previous adapters that can be connected to it
    // with steps[0] = 1
    // and use this to build the final result from steps[0]
    let n = data.len();
    let mut steps: Vec<u64> = vec![0; n];
    steps.insert(0, 1);
    for i in 0..n {
        let mut j = i + 1;
        while j < n && data[j] <= data[i] + 3 {
            steps[j] += steps[i];
            j += 1;
        }
    }
    println!("part 2: {}", steps[n- 1]);
}

fn main() {
    // Data is a sorted vector of adapter joltages including 
    // the starting joltage 0 and the device joltage
    let mut data: Vec<u32> = vec![0];
    data.extend(io::stdin().lock().lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap()));
    data.sort_unstable();
    data.push(&data[data.len() - 1] + 3); // Add the device joltage
    part1(&data);
    part2(&data);
}
