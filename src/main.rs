use std::mem;

/// The width of the CRC calculation and result
type Crc = u32;

const WIDTH: u8 = 8 * mem::size_of::<Crc>() as u8;
const TOPBIT: Crc = 1 << (WIDTH - 1);

const fn reflect(data: Crc, n_bits: u8) -> Crc {
    let mut data = data;
    let mut reflection = 0;
    // Reflect the data about the center bit

    let mut bit = 0;
    while bit < n_bits {
        // If the LSB bit is set, set the reflection of it
        if data & 0x01 == 0x01 {
            reflection |= 1 <<((n_bits - 1) - bit);
        }

        data = data >> 1;
        bit += 1;
    }

    reflection
}

#[cfg(feature="reflect_data")]
const fn reflect_data(data: u8) -> u8 {
    reflect(data as Crc, 8) as u8
}

#[cfg(feature="reflect_remainder")]
const fn reflect_remainder(remainder: Crc) -> Crc {
    reflect(remainder, WIDTH)
}

/// Reflect Data: reverse bit ordering of data
/// Reflect Remainder: reverse bit ordering of remainder
/// Check value: The result of CRC'ing the ASCII char[]: "123456789"
/// See: https://barrgroup.com/Embedded-Systems/How-To/CRC-Calculation-C-Code
fn main() {
    println!("TOPBIT: {:08b}", TOPBIT);
    println!("WIDTH: {}", WIDTH);

    let msg: &[u8] = "123456789".as_bytes();
    // final xor of 0xFFFFFFF is the same as inverting the bits
    // (or complementing the value)
    let crc = crc_slow(0x04C11DB7, 0xFFFFFFFF, 0xFFFFFFFF, msg);

    println!("CRC: {:04b}", crc);
    println!("Debug: {:0x}", crc);
}

/// Fast is to precalculate all the 256 results from the divison and memoize it
fn crc_slow(poly: Crc, remainder: Crc, final_xor: Crc, message: &[u8]) -> Crc {
    println!("polynominal: {:032b}", poly);
    println!("initial_remainder: {:032b}", remainder);
    println!("final_xor: {:032b}", final_xor);

    let mut remainder: Crc = remainder;
    println!("remainder: {:032b}", remainder);

    println!("reflected: {:032b}", remainder);
    // Perform modulo-2 division, a byte at a time
    for b in message {
        let b = *b;
        #[cfg(feature="reflect_data")]
        let b = reflect_data(b);
        remainder ^= (b as Crc) << (WIDTH - 8);

        // For each bit position in the message
        for _ in 0..8 {
            // if the uppermost bit is a 1
            if remainder & TOPBIT == TOPBIT {
                // XOR the previous remainder with the divisor
                remainder = (remainder << 1) ^ poly as Crc;
            } else {
                // Shift the next bit of the message into the remainder
                remainder = remainder << 1;
            }
        }
    }

    let remainder = remainder ^ final_xor;
    #[cfg(feature="reflect_remainder")]
    let remainder: Crc = reflect_remainder(remainder);
    remainder
}


#[test]
fn reflect_u8_works() {
    let test = 0b00001111;
    let exp = 0b11110000;
    let test = reflect(test, 8);
    assert_eq!(exp, test);
}

#[test]
fn reflect_u32_works() {
    let test: u32 = 0b00000000000000001111111111111111;
    let exp: u32  = 0b11111111111111110000000000000000;
    let test = reflect(test, 32);
    assert_eq!(exp, test);
}

#[test]
#[cfg_attr(not(feature = "reflect_all"), ignore)]
fn test_crc() {
    let msg: &[u8] = "123456789".as_bytes();
    let test = crc_slow(0x04C11DB7, 0xFFFFFFFF, 0xFFFFFFFF, msg);
    assert_eq!(test, 0xCBF43926);
}