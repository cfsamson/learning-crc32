use std::mem;

/// The width of the CRC calculation and result
type Crc = u16;

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

#[cfg(feature="reflect")]
const fn reflect_data(data: u8) -> u8 {
    reflect(data as Crc, 8) as u8
}

#[cfg(feature="reflect")]
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


    let msg: &[u8] = "1234".as_bytes();
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7eda8cc57d56dba77e14e23d44bb7be6
    // CRC-CCITT
    // poly: 0x1021
    // initial remainder: 0xFFFF
    // expected: 101001101001001 # "1234"
    //
    //  init. reminder         message = "1234"          augmentation
    // ---------------|--------------------------------|--------------|
    // 1111111111111111000111110010000000100001001000100000000000000000 # "1234"
    //    trunc. poly.
    //  |--------------|
    // 10001000000100001
    // ^
    // Un-truncated poly adds 1 to the start
    //
    // 1111111111111111000111110010000000100001001000100000000000000000
    // 10001000000100001
    // 0111011111101111100111110010000000100001001000100000000000000000
    //
    // 111011111101111100111110010000000100001001000100000000000000000
    // 10001000000100001
    // 011001111100111110111110010000000100001001000100000000000000000
    //
    // 11001111100111110111110010000000100001001000100000000000000000
    // 10001000000100001
    // 01000111100011111111110010000000100001001000100000000000000000
    //
    // 1000111100011111111110010000000100001001000100000000000000000
    // 10001000000100001
    // 0000011100001111011110010000000100001001000100000000000000000
    //
    // 11100001111011110010000000100001001000100000000000000000
    // 10001000000100001
    // 01101001111111111010000000100001001000100000000000000000
    //
    // 1101001111111111010000000100001001000100000000000000000
    // 10001000000100001
    // 0101101111101111110000000100001001000100000000000000000
    //
    // 101101111101111110000000100001001000100000000000000000
    // 10001000000100001
    // 001111111100111100000000100001001000100000000000000000
    //
    // 1111111100111100000000100001001000100000000000000000
    // 10001000000100001
    // 0111011100101100100000100001001000100000000000000000
    //
    // 111011100101100100000100001001000100000000000000000
    // 10001000000100001
    // 011001100100100110000100001001000100000000000000000
    //
    // 11001100100100110000100001001000100000000000000000
    // 10001000000100001
    // 01000100100000111000100001001000100000000000000000
    //
    // 1000100100000111000100001001000100000000000000000
    // 10001000000100001
    // 0000000100010111100100001001000100000000000000000
    //
    // 100010111100100001001000100000000000000000
    // 10001000000100001
    // 000000111101100011001000100000000000000000
    //
    // 111101100011001000100000000000000000
    // 10001000000100001
    // 011111100010001010100000000000000000
    //
    // 11111100010001010100000000000000000
    // 10001000000100001
    // 01110100010101011100000000000000000
    //
    //                    |-------------|
    // 1110100010101011100000000000000000
    // 10001000000100001
    // 0110000010111011000000000000000000
    //
    //                   |-------------|
    // 110000010111011000000000000000000
    // 10001000000100001
    // 010010010110011010000000000000000
    //
    //                  |-------------|
    // 10010010110011010000000000000000
    // 10001000000100001
    // 00011010110111011000000000000000
    //
    // 11010110111011000000000000000
    // 10001000000100001
    // 01011110111111001000000000000
    //
    // 1011110111111001000000000000
    // 10001000000100001
    // 0011010111101001100000000000
    //
    // 11010111101001100000000000
    // 10001000000100001
    // 01011111101101101000000000
    //
    // 1011111101101101000000000
    // 10001000000100001
    // 0011011101111101100000000
    //
    // 11011101111101100000000
    // 10001000000100001
    // 01010101111001101000000
    //
    // 1010101111001101000000
    // 10001000000100001
    // 0010001111011101100000
    //
    // 10001111011101100000
    // 10001000000100001
    // 000001110110011010000
    //
    // 1110110011010000
    // 10001000000100001
    // 01100100110000001
    //
    // 0010100110100100 # exp

    // final xor of 0xFFFFFFF is the same as inverting the bits
    // (or complementing the value)
    let crc = crc_slow(0x1021, 0xFFFF, 0x0000, msg);

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
        #[cfg(feature="reflect")]
        let b = reflect_data(b);
        remainder ^= (b as Crc) << (WIDTH - 8);

        // For each bit position in the message
        for _ in 0..8 {
            // if the uppermost bit is a 1
            if remainder & TOPBIT == TOPBIT {
                // XOR the previous remainder with the divisor
                remainder = (remainder << 1) ^ poly as Crc; // because we know that poly actually starts with a 1 (not included) wich will result in a 0 that is shifted out
            } else {
                // Shift the next bit of the message into the remainder
                remainder = remainder << 1;
            }
        }
    }

    let remainder = remainder ^ final_xor;
    #[cfg(feature="reflect")]
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
#[cfg_attr(not(feature = "reflect"), ignore)]
fn test_crc() {
    let msg: &[u8] = "123456789".as_bytes();
    let test = crc_slow(0x04C11DB7, 0xFFFFFFFF, 0xFFFFFFFF, msg);
    assert_eq!(test, 0xCBF43926);
}