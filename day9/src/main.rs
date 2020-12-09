use std::io::{self, BufRead};

fn part1(data: &Vec<usize>) -> Option<usize> {
    let result = data.windows(26).filter_map(|w| not_sum_of_previous(w[25], &w[0..25].to_vec())).nth(0);
    println!("Part 1: {:?}", result);
    result
}

fn not_sum_of_previous(x: usize, prev: &Vec<usize>) -> Option<usize>  {
    let mut c = prev.clone();
    c.sort();
    let mut left = 0;
    let mut right = c.len() - 1;
    let mut sum;
    while right > left {
        sum = c[left] + c[right];
        if sum > x {
            right -= 1;
        } else if sum < x {
            left += 1;
        } else {
            return None;
        }
    }
    Some(x)
}

fn part2(data: &Vec<usize>, target: usize) {
    let mut slow = 0;
    let mut curr_sum = data[0];
    for (i, v) in data.iter().skip(1).enumerate() {
        while curr_sum > target && slow < i - 1 {
            curr_sum -= data[slow];
            slow += 1;
        }
        if curr_sum == target {
            let mut result = data[slow..i+1].to_vec();
            result.sort();
            println!("Part 2: {}", result[0] + result[result.len() - 1]);
            break;
        }

        if i < data.len() {
            curr_sum += v;
        }
    }
}

fn main() {
    let data: Vec<usize> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let part1_result = part1(&data).unwrap();
    part2(&data, part1_result);
}

