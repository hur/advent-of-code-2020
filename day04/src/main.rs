use std::ops::RangeInclusive;

fn main() {
    // Part 2
    let count = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .fold(0, |acc, passport| if passport.split_whitespace().filter(validator).count() == 7 {acc + 1} else {acc} );

    println!("{}", count);

}

fn validator(field: &&str) -> bool {
    let mut f = field.split(":");
    let key = f.next().unwrap();
    let val = f.next().unwrap();

    match key {
        "byr" => in_range(&val, 1920..=2002),
        "iyr" => in_range(&val, 2010..=2020),
        "eyr" => in_range(&val, 2020..=2030),
        "hgt" => validate_height(&val),
        "hcl" => validate_hcl(&val),
        "ecl" => validate_ecl(&val),
        "pid" => val.len() == 9 && val.chars().all(char::is_numeric),
        _     => false
    }
}


fn in_range(val: &str, range: RangeInclusive<usize>) -> bool {
    val.parse().map_or(false, |x: usize| range.contains(&x))
}

fn validate_height(val: &str) -> bool {
    val.strip_suffix("cm")
        .map_or(false, |x| in_range(&x, 150..=193)) ||  
    val.strip_suffix("in").map_or(false, |x| in_range(&x, 59..=76))
}

fn validate_hcl(val: &str) -> bool {
    val.strip_prefix("#").map_or(false, |x| x.chars().all(|c| c.is_ascii_hexdigit()))
}

fn validate_ecl(val: &str) -> bool {
    matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}
    /* Part 1, naive
    let mut valid_passports: usize = 0;
    for passport in passports {
        if  passport.contains("hcl") && passport.contains("byr") &&
            passport.contains("iyr") && passport.contains("eyr") &&
            passport.contains("hgt") && passport.contains("ecl") && 
            passport.contains("pid") {
            valid_passports += 1;
        }
    }

    println!("{}", valid_passports);
    */
