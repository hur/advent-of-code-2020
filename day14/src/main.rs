use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask  { off: u64, on: u64 },
    Write { address: u64, value: u64 },
}

fn part1(data: &Vec<(Instruction, Vec<Instruction>)>) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    data.iter().for_each(|(mask, instrs)| {
        if let Instruction::Mask {off, on} = mask {
            instrs.iter().for_each(|instr| {
                if let Instruction::Write {address, value} = instr {
                    // Apply mask and manipulate memory
                    mem.insert(*address, (*value | off) & on);
                };
            });
        }
    });
    println!("Part 1: {}", mem.values().sum::<u64>());
}

fn part2(data: &Vec<(Instruction, Vec<Instruction>)>) {
    const THIRTYSIXBITS: u64 = (1 << 36) - 1;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    data.iter().for_each(|(mask, instrs)| {
        if let Instruction::Mask {off, on} = mask {
            instrs.iter().for_each(|instr| {
                if let Instruction::Write {address, value} = instr {
                    // base_address is address but with floating bits set to zero
                    // off | !on gives binary where the floating bits are zero and others are 1
                    let base_address = (address | off) & (off | !on);
                    let mut floating_mask = off | !on;
                    loop {
                        let floating_inverted = (!floating_mask) & THIRTYSIXBITS;
                        let new_address = base_address | floating_inverted;
                        mem.insert(new_address, *value);
                        if floating_mask & THIRTYSIXBITS == THIRTYSIXBITS {
                            break;
                        }
                        // Keep adding ones to set all the different bits
                        // or with the original inverted mask handles overflow
                        floating_mask += 1;
                        floating_mask |= off | !on;
                    }
                };
            });
        };
    });
    println!("Part 2: {}", mem.values().sum::<u64>());
}

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();
    
    let instructions: Vec<(Instruction, Vec<Instruction>)> = data.split("mask = ").skip(1)
        .map(|block| {
            let mut iterator = block.split("\n"); // Get lines
            let mask = iterator.next().unwrap();
            let mask = Instruction::Mask {
                off: u64::from_str_radix(&mask.replace("X", "0" ), 2).unwrap(),
                on:  u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap()
            };
            let lines: Vec<Instruction> = iterator.filter_map(|line| {
                if line == "" {return None}
                line.strip_prefix("mem[").and_then(|s| {
                    let mut iter = s.splitn(2, "] = ");
                    Some(Instruction::Write {
                        address: iter.next().unwrap_or("-1").parse().unwrap(),
                        value: iter.next().unwrap_or("-1").parse().unwrap()
                    })
                })
            }).collect();
            (mask, lines)
        }).collect();
    part1(&instructions);
    part2(&instructions);
}
