// CRC-32
// Truncated Poly.:     0x04C11DB7
// Initial remainder:   0xFFFFFFFF
// Reflect data:        yes
// Reflect remainder:   yes
// Expected result:     0xDBF1FE9A
//
//        initial reminder                                                                       message
// --------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
// 11111111111111111111111111111111 0100100101001000010001000101001000000000000000000000001110001100000000000000000000000010110100000000100000000110000000000000000000000000
// 100000100110000010001110110110111
//  |------------------------------|
//    Trunc. polynomial (32 bits)
// |-------------------------------|
//    Full polynomial (33 bits)
//
// - We know that any valid polynomial has to start with a 1 bit so the
// "interesting" part is everythin after the first bit (called the truncated
// polynomial). See comment in the `crc` function.
//
fn main() {

    let msg: &[u8] = &[0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x03, 0x8C, 0x00, 0x00, 0x02, 0xD0, 0x08, 0x06, 0x00, 0x00, 0x00];
    let crc = crc(0x04C11DB7, 0xFFFFFFFF, 0xFFFFFFFF, msg);

    print!("msg: ");
    for b in msg {
        print!("{:08b}", b);
    }
    print!("\n\n");
    println!("------- Results -------");
    println!("CRC bits: {:032b}", crc);
    println!("CRC hex: {:0X}", crc);
}

/// This is what we do for CRC-32, step by step:
///
/// 1. We make a copy of the remainder that we can change, which will store the
/// final CRC
/// 2. For each byte in the message:
///     1. Reverse the bits og the byte (1100) => (0011)
///     2. We push the message bytes to the leftmost bits of the remainder
///     3. For each of the 8 bits of the "message byte" we:
///         1. Shift the bits 1 position left if the leftmost is a 0
///         2. When the leftmost is a 1, we shift out 1 byte (since it will always end up a 0)
///            then we divide the remainder with the truncated polynomial
/// 3. In CRC-32 we XOR the result with the value 0xFFFFFFF (which is the same as
///    inverting the bits)
/// 4. We reverse the bits of the final remainder to get our CRC
fn crc(poly: u32, remainder: u32, final_xor: u32, message: &[u8]) -> u32 {
    println!("trunc. polynomial: {:032b}", poly);
    println!("initial_remainder: {:032b}", remainder);
    println!("final_xor: {:032b}", final_xor);
    println!("remainder: {:032b}", remainder);

    // 1. Just make a mutable variable storing the remainder
    let mut remainder: u32 = remainder;

    // 2. Perform modulo-2 division, a byte at a time
    for b in message {
        let b = *b;
        // 2.1 reverse bits
        let b = b.reverse_bits();

        // 2.2 Shift the reflected message bytes to the leftmost bits of remainder. Ex:
        // beofre: 11111111111111111111111111111111
        // after:  01101101111111111111111111111111
        remainder ^= (b as u32) << (32 - 8);

        // 2.3 For each bit position in the message
        for _ in 0..8 {
            if remainder & 0b10000000000000000000000000000000 == 0b10000000000000000000000000000000 {
                // 2.3.2 (see initial explanation of steps)
                remainder = (remainder << 1) ^ poly as u32;
                //          |--------------|
                //                  ^
                //                  IMPORTANT: because we know that the "full poly"
                //                  actually starts with a 1 (which is not included
                //                  in the trucated poly) it will always "switch" the
                //                  leftmost bit to a 0 whcih is shifted out so we
                //                  actually shift it out here before we divide the
                //                  rest with the truncated poly.
            } else {
                // 2.3.1 Shift the bits 1 step left
                remainder = remainder << 1;
            }
        }
    }

    // 3. XOR with the value 0xFFFFFFFF
    let remainder = remainder ^ final_xor;
    // 4. reverse bits to get the final CRC
    let remainder: u32 = remainder.reverse_bits();
    remainder
}
