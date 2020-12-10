
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let mut count = 0;
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Could not read line.");
        let buffer = buffer.trim();
        if buffer == String::from("") {break;}
        
        let parts: Vec<&str> = buffer.split(" ").collect();

        // Parse parts

        let nums = re.find_iter(parts[0]).map(|x| x.as_str().parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let c = parts[1].chars().nth(0).unwrap();

        // 1
        //
        // let inner_count = parts[2].chars().fold(0, |acc, x| if x == c {acc + 1} else {acc});
        // 
        // if nums[0] <= inner_count && inner_count <= nums[1] {
        //     count += 1;
        // }

        // 2

        let first = parts[2].chars().nth(nums[0] - 1).unwrap();
        let second = parts[2].chars().nth(nums[1] - 1).unwrap();

        if (first == c) ^ (second == c) {
            count += 1;
        }
                
        

    }
    println!("{}", count);

}
