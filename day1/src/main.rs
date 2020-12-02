
fn main() {
    let mut vec = Vec::new();
    while true {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed");
        let buffer = buffer.trim();
        if buffer == String::from("") {break;}
        vec.push(buffer.parse::<i64>().unwrap());
    }
    for v in vec.iter() {
        for v2 in vec.iter() {
            for v3 in vec.iter() {
                if v + v2 +v3 == 2020 {
                    println!("{}", v*v2*v3);
                }

            }
        }
    }
}
