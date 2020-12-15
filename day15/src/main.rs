use std::collections::HashMap;
use std::time::Instant;
// Optimization logs:  
//      these are not properly benchmarked, 
//      I just ran the thing a couple of times
//      with 30 million iterations and compared runtimes)
//
// 1. Change hashmap from <i32, [isize ; 2]> to <i32, i32> 
//    by leveraging the return value of hashmap.insert()
//      needs less calls to hashmap and less comparisons. 
//      Time down from ~48s to ~18s
// 2. change usize to i32
//      cuts runtime down from ~18s to ~17s
// 3. Build release binary instead of running debug binary
//      runtime down from ~16s to ~3s
fn calculate(start: &Vec<i32>, limit: i32) -> Option<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    let mut last = 0;
    let mut turn = 0;
    for s in start {
        turn += 1;
        last = *s;
        seen.insert(last, turn);
    };
    for i in turn..limit {
        // dbg!(&seen);
        last = match seen.insert(last, i) {
            Some(prev) => i - prev,
            None       => 0,
        };
    }
    Some(last)
}

fn main() {
    println!("Part 1: {:?}", calculate(&vec![16,11,15,0,1,7], 2020));
    let now = Instant::now();
    println!("Part 2: {:?}", calculate(&vec![16,11,15,0,1,7], 30000000));
    println!("Part 2 took {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1_1() {
        let input = vec![0,3,6];
        assert_eq!(calculate(&input, 2020), Some(436))
    }
    #[test]
    fn test_case_1_2() {
        let input = vec![1,3,2];
        assert_eq!(calculate(&input, 2020), Some(1))
    }
    #[test]
    fn test_case_1_3() {
        let input = vec![2,1,3];
        assert_eq!(calculate(&input, 2020), Some(10))
    }
    #[test]
    fn test_case_1_4() {
        let input = vec![1,2,3];
        assert_eq!(calculate(&input, 2020), Some(27))
    }
    #[test]
    fn test_case_2_1() {
        let input = vec![0,3,6];
        assert_eq!(calculate(&input, 30000000), Some(175594))
    }
}
