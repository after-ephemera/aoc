use eyre::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::repeat;

const INTEGER_SIZE: usize = 36;

#[derive(Debug)]
enum ProgramType {
    Dma,
    Decoder,
}

#[derive(Debug)]
struct Program {
    mask: String,
    memspace: HashMap<usize, usize>,
    program_type: ProgramType,
}

impl Program {
    fn new(program_type: ProgramType) -> Self {
        Program {
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            memspace: HashMap::new(),
            program_type,
        }
    }

    fn mask(&mut self, mask_str: &str) {
        //println!("mask in: {}", mask_str);
        self.mask = mask_str.to_string();
    }

    fn set_mem(&mut self, address: usize, value: usize) -> Result<()> {
        match self.program_type {
            ProgramType::Dma => {
                let bin_val = &format!("{:b}", value);
                let prefix = repeat('0').take(INTEGER_SIZE - bin_val.len());
                let val_iter = prefix.chain(bin_val.chars());
                let mut masked_vec = vec![];
                for (mask_ch, val_ch) in self.mask.chars().zip(val_iter) {
                    match mask_ch {
                        '0' | '1' => masked_vec.push(mask_ch),
                        'X' => masked_vec.push(val_ch),
                        _ => println!("ffailure"),
                    }
                }
                let result = usize::from_str_radix(&masked_vec.iter().collect::<String>(), 2)?;
                self.memspace.insert(address, result);
                //println!("final result: {}", result);
            }
            ProgramType::Decoder => {
                let mut addr_str = format!("{:b}", address);
                println!("old addr string: {}", addr_str);
                let num_floating = self.mask.chars().filter(|c| *c == 'X').count();
                let address_count = 2usize.pow(num_floating as u32);
                println!(
                    "{} floating in {}. Requires {} addresses for {}.",
                    num_floating, self.mask, address_count, addr_str
                );
                let prefix = repeat('0').take(INTEGER_SIZE - addr_str.len());
                addr_str = prefix.chain(addr_str.chars()).collect();
                // replace address values with 'X'
                for (index, ch) in self.mask.chars().enumerate() {
                    if ch == 'X' {
                        addr_str = addr_str
                            .chars()
                            .enumerate()
                            .map(|(i, ch)| if i == index { 'X' } else { ch })
                            .collect();
                    } else if ch == '1' {
                        addr_str = addr_str
                            .chars()
                            .enumerate()
                            .map(|(i, ch)| if i == index { '1' } else { ch })
                            .collect();
                    }
                }
                println!("new addr string: {}", addr_str);

                let mut addrs_to_write = vec![addr_str];
                for _ in 0..num_floating {
                    let mut new_addrs_to_write = vec![];
                    for addr in &mut addrs_to_write {
                        new_addrs_to_write.push(addr.clone().replacen('X', "0", 1));
                        new_addrs_to_write.push(addr.clone().replacen('X', "1", 1));
                    }
                    addrs_to_write = new_addrs_to_write;
                }
                println!("created new addresses {:?}", addrs_to_write);
                for addr in addrs_to_write {
                    self.memspace
                        .insert(usize::from_str_radix(&addr, 2)?, value);
                }
            }
        }
        Ok(())
    }

    fn sum(&self) -> usize {
        self.memspace.values().sum()
    }
}

fn main() -> Result<()> {
    let input = read_to_string("src/day14/input.txt")?;
    //let input = read_to_string("src/day14/input-sample4.txt")?;
    let mut program = Program::new(ProgramType::Dma);
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let re = Regex::new(r"(mask = (.*))?(mem\[(.*)\] = (.*))?").unwrap();
        let captures = re.captures(line).unwrap();
        //println!("capture {:?}", captures);
        if let Some(mask) = captures.get(2) {
            //println!("got mask {}", mask.as_str());
            program.mask(mask.as_str());
        } else if let (Some(address), Some(val)) = (captures.get(4), captures.get(5)) {
            //println!("address: {}, val: {}", address.as_str(), val.as_str());
            program.set_mem(
                address.as_str().parse::<usize>()?,
                val.as_str().parse::<usize>()?,
            )?;
        }
    }
    println!("{:?}", program);
    println!("final sum: {}", program.sum());

    // part 2
    let mut program = Program::new(ProgramType::Decoder);
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let re = Regex::new(r"(mask = (.*))?(mem\[(.*)\] = (.*))?").unwrap();
        let captures = re.captures(line).unwrap();
        if let Some(mask) = captures.get(2) {
            program.mask(mask.as_str());
        } else if let (Some(address), Some(val)) = (captures.get(4), captures.get(5)) {
            //println!("address: {}, val: {}", address.as_str(), val.as_str());
            program.set_mem(
                address.as_str().parse::<usize>()?,
                val.as_str().parse::<usize>()?,
            )?;
        }
    }
    println!("{:?}", program);
    println!("final sum: {}", program.sum());

    Ok(())
}
