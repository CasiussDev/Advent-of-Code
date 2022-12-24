extern crate nom;

mod device;
mod parse_instr;

use parse_instr::parse_instructions;
use std::fs;

const FIRST_INTERESTING_CYCLE: u32 = 20;
const INTERESTING_CYCLE_STEP: usize = 40;
const LAST_INTERESTING_CYCLE: u32 = 220;

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let instr = parse_instructions(input_text.as_str());
    let mut instr_iter = instr.iter().cloned();

    let mut dev = device::Device::new();

    let interesting_iter = (FIRST_INTERESTING_CYCLE..=LAST_INTERESTING_CYCLE).step_by(INTERESTING_CYCLE_STEP);

    let signal_sum: i16 = interesting_iter.map(|cycles| {
        instr_iter.find(|&instr| {
            //println!("execute {instr:?}");
            dev.execute(instr);
            //println!("{dev:?}");
            dev.cycle_count() >= cycles
        });
        if dev.cycle_count() == cycles {
            //println!("\tAt cycle {cycles} strength {}", dev.x() * cycles as i16);
            dev.x() * cycles as i16
        } else if dev.cycle_count() > cycles {
            //println!("\tAt cycle {} prev strength {}", dev.cycle_count(), dev.prev_x() * cycles as i16);
            dev.prev_x() * cycles as i16
        } else {
            unreachable!()
        }
    }).sum();

    println!("Sum of signal strength at interesting time points is {signal_sum}");

    
}
