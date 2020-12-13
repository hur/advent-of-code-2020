use modinverse::modinverse;

fn part1(timestamp: &i64, buses: &Vec<(i64, i64)>) {
    let mut min_diff = std::i64::MAX;
    let mut bus_id = 0;
    buses.iter().for_each(|(_, ts)| {
        let diff = ((timestamp + ts - 1) / ts) * ts - timestamp;
        if diff < min_diff {
            min_diff = diff;
            bus_id = *ts;
        }
    });
    println!("Part 1: {}", bus_id * min_diff);
}

fn part2(buses: &Vec<(i64, i64)>) {
    let residues: Vec<i64> = buses.iter()
        .map(|(i, x)|(x - i))
        .collect();
    let moduli: Vec<i64> = buses.iter()
    .map(|(_, x)| *x )
    .collect();

    let res = chinese_remainder(residues.as_slice(), &moduli.as_slice());
    println!("Part 2: {:?}", res);
}

// Rosetta code
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * modinverse(p, modulus)? * p;
    }
    Some(sum % prod)
}
fn main() {
    let stdin = std::io::stdin();
    let mut timestamp = String::new();
    stdin.read_line(&mut timestamp).expect("Could not read line");
    let mut buses= String::new();
    stdin.read_line(&mut buses).expect("Could not read line.");
    let timestamp = timestamp.trim().parse::<i64>().unwrap();
    let buses: Vec<(i64, i64)> = buses.trim().split(",").enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect();
    part1(&timestamp, &buses);
    part2(&buses);
}