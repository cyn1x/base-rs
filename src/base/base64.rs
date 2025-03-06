pub fn encode(input: &[u8]) -> String {
    const BASE64_TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in input {
        buffer = (buffer << 8) | (byte as u32);
        bits_collected += 8;

        while bits_collected >= 6 {
            bits_collected -= 6;
            let index = (buffer >> bits_collected) & 0x3F;
            output.push(BASE64_TABLE[index as usize] as char);
        }
    }

    if bits_collected > 0 {
        buffer <<= 6 - bits_collected;
        output.push(BASE64_TABLE[(buffer & 0x3F) as usize] as char);
    }

    while output.len() % 4 != 0 {
        output.push('=');
    }

    output
}
