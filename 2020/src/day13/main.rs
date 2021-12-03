use eyre::Result;
use std::fs::read_to_string;

fn find_earliest_bus_from(schedule_str: &str) -> Result<()> {
    let mut schedule_parts = schedule_str.split_whitespace();
    let earliest_departure = schedule_parts.next().unwrap().parse::<usize>()?;
    let schedule: Vec<(usize, usize)> = schedule_parts
        .next()
        .unwrap()
        .split(',')
        .filter(|i| *i != "x")
        .map(|i| {
            let bus_id = i.parse::<usize>().unwrap();
            //println!("searching for earliest departure of {}", bus_id);
            let mut result = vec![0; bus_id + 1];
            result[0] = 1;
            result[bus_id] = 1;
            //println!(
            //    "{} created {:?}",
            //    bus_id,
            //    result
            //        .iter()
            //        .clone()
            //        .skip(1)
            //        .cycle()
            //        .enumerate()
            //        .map(|(index, val)| (index + 1, val))
            //        .skip(earliest_departure - 4)
            //        .take(25)
            //        .collect::<Vec<(usize, &usize)>>()
            //);
            result
                .iter()
                // skip initial departure at t==0
                .skip(1)
                // cycle the schedule to allow for arbitrary departure time checks
                .cycle()
                .enumerate()
                // indices are off by one because oof the above skip
                .map(|(bus_id, val)| (bus_id + 1, val))
                // skip to the time in question
                .skip(earliest_departure)
                // find the next departure
                .find(|(_, x)| **x == 1)
                // salient information is contained in the departure time and the bus id
                .map(|(j, _)| (j, bus_id))
                .unwrap()
        })
        .collect();

    //println!("times: {:?}, buses: {:?}", times, buses);
    let best_time = schedule.iter().min_by_key(|(time, _)| time).unwrap();
    println!("best time is {:?}", best_time);
    println!(
        "result is {}",
        (best_time.0 - earliest_departure) * best_time.1
    );
    Ok(())
}

fn find_earliest_consecutive_departures(schedule_str: &str) -> usize {
    let buses: Vec<(usize, usize)> = schedule_str
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
        .collect();
    let mut result = 0;
    let mut lcd = 1;
    for bus in buses {
        println!("bus: {:?}", bus);
        while (result + bus.0) % bus.1 > 0 {
            result += lcd;
        }
        lcd *= bus.1;
    }
    result
}

fn main() -> Result<()> {
    //let input = read_to_string("src/day13/input.txt")?;
    let input = read_to_string("src/day13/input-sample6.txt")?;
    find_earliest_bus_from(&input)?;
    println!("result {}", find_earliest_consecutive_departures(&input));
    Ok(())
}
