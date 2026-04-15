use std::fmt::Write;

pub fn inspect_packet(data: &[u8]) {
    println!("=== Packet Inspector ===");
    println!("Length: {} bytes", data.len());

    // Hex dump
    let mut hex_output = String::new();
    for (i, byte) in data.iter().enumerate() {
        write!(hex_output, "{:02X} ", byte).unwrap();

        if (i + 1) % 16 == 0 {
            hex_output.push('\n');
        }
    }

    println!("Hex Dump:\n{}", hex_output);

    // ASCII view
    let ascii: String = data
        .iter()
        .map(|b| {
            if b.is_ascii_graphic() {
                *b as char
            } else {
                '.'
            }
        })
        .collect();

    println!("ASCII:\n{}", ascii);
}
