use std::mem;

/// The width of the CRC calculation and result
type Crc = u8;

const WIDTH: usize = (8 * mem::size_of::<Crc>());
const TOPBIT: u8 = (1 << (WIDTH - 1));

/// Optimization we discard the top bit (was 0b0101100)
const POLYNOMIAL: u8 = 0b01011000;

fn main() {
    println!("TOPBIT: {:08b}", TOPBIT);
    println!("WIDTH: {}", WIDTH);

    let msg: u8 = 0b11100101;
    let crc = crc_slow(&[msg]);

    println!("CRC: {:04b}", crc);
    println!("Debug: {:08b}", crc);
}

fn crc_slow(message: &[u8]) -> u8 {
    let mut remainder: Crc = 0;

    // Perform modulo-2 division, a byte at a time
    for b in message {
        remainder ^= b << (WIDTH - 8);

        // For each bit position in the message
        for _ in 0..8 {
            println!("Reminder: {:08b}", remainder);
            // if the uppermost bit is a 1
            if remainder & TOPBIT == TOPBIT {
                // XOR the previous remainder with the divisor
                remainder = (remainder << 1) ^ POLYNOMIAL;
            } else {
                // Shift the next bit of the message into the remainder
                remainder = remainder << 1;
            }
        }
    }

    remainder
}
