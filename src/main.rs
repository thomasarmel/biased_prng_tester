use std::fs::File;
use std::io;
use std::io::{Read, Write};

const BIAS_PERCENT: f64 = 7f64;

fn main() {
    let dev_random = File::open("/dev/random").unwrap();
    let mut dev_random = dev_random.bytes();
    loop {
        let mut output_biased_byte = 0u8;
        for bit_pos in 0..8 {
            let bit: u8 = compute_biased_bit_from_byte(dev_random.next().unwrap().unwrap()) as u8;
            output_biased_byte |= bit << bit_pos;
        }
        io::stdout().write(&[output_biased_byte]).unwrap();
    }
}

#[inline(always)]
fn compute_biased_bit_from_byte(byte: u8) -> bool {
    const BYTE_VALUE_COMPARISON: u8 = 127u8 + (128f64 * BIAS_PERCENT / 100f64) as u8;
    byte > BYTE_VALUE_COMPARISON
}