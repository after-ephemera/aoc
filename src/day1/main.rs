use std::fs::File;
use std::io::Read;

fn part_2() {
    let mut f = File::open("src/day1/input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    for sub in s.split('\n') {
        for sub2 in s.split('\n') {
            for sub3 in s.split('\n') {
                if sub == "" || sub2 == "" || sub3 == "" {
                    continue;
                }
                //println!("{}", sub);
                //println!("{}", sub2);
                if sub.parse::<i32>().unwrap()
                    + sub2.parse::<i32>().unwrap()
                    + sub3.parse::<i32>().unwrap()
                    == 2020
                {
                    println!(
                        "found it! {} * {} * {} = {}",
                        sub,
                        sub2,
                        sub3,
                        sub.parse::<i32>().unwrap()
                            * sub2.parse::<i32>().unwrap()
                            * sub3.parse::<i32>().unwrap()
                    );
                    return;
                }
            }
        }
    }
}

fn part_1() {
    let mut f = File::open("src/day1/input2.txt").unwrap();
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

fn main() {
    part_1();
    part_2();
}
