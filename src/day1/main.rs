use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("src/day1/input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    for sub in s.split('\n') {
        for sub2 in s.split('\n') {
            if sub == "" || sub2 == "" {
                continue;
            }
            println!("{}", sub);
            println!("{}", sub2);
            if sub.parse::<i32>().unwrap() + sub2.parse::<i32>().unwrap() == 2020 {
                println!(
                    "found it! {} * {} = {}",
                    sub,
                    sub2,
                    sub.parse::<i32>().unwrap() * sub2.parse::<i32>().unwrap()
                );
                return;
            }
        }
    }
}
