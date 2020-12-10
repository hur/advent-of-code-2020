use std::collections::HashSet;

fn part1(data: &String) {
    let ans: usize = data.split("\n\n")
        .map(|x| {
            x.lines()
                .flat_map(|c| c.chars())
                .collect::<HashSet<_>>()
                .len()
        }).sum();

    println!("{}", ans);
}

fn part2(data: &String) {
    let res: usize = data.split("\n\n")
        .map(|g| {
            let all: HashSet<_> = g.lines().flat_map(|x| x.chars()).collect();

            g.lines().map(|x| x.chars().collect::<HashSet<_>>())
                .fold(all, |a, x| a.intersection(&x).cloned().collect()).len()
        }).sum();

    println!("{}", res);
}
fn main() {
    let ans: String = std::fs::read_to_string("input.txt").unwrap();

    part1(&ans);
    part2(&ans);
}

