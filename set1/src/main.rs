fn hex_to_base64(hex_in: String) -> String {
    // Output
    let mut b64_out = String::new();

    // Chart
    let b64_chars: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k',
        'j', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    let mut hex_u8s = Vec::new();
    for (i, _u) in hex_in.chars().enumerate().step_by(2) {
        match hex_in.get(i..i + 2) {
            Some(v) => match u8::from_str_radix(v, 16) {
                Ok(vv) => hex_u8s.push(vv),
                Err(e) => println!(
                    "Error parsing hex symbol[{}]@[{}]: {}",
                    hex_in.get(i..i + 2).unwrap(),
                    i,
                    e
                ),
            },
            None => println!("Incomplete hex detected at [{}]", i),
        };
    }

    // Alias -- we'll be using it a lot
    let h = &hex_u8s;

    for (i, _u) in hex_u8s.iter().enumerate().step_by(3) {
        // Decompose 3 octets into 4 sextets
        let s1 = h[i] >> 2;
        let s2 = h[i] << 6 >> 2 | h[i + 1] >> 4;
        let s3 = h[i + 1] << 4 >> 2 | h[i + 2] >> 6;
        let s4 = h[i + 2] << 2 >> 2;

        // Use each sextet to map to b64 character
        b64_out.push(b64_chars[s1 as usize]);
        b64_out.push(b64_chars[s2 as usize]);
        b64_out.push(b64_chars[s3 as usize]);
        b64_out.push(b64_chars[s4 as usize]);
    }

    b64_out
}

#[cfg(test)]
mod set1_tests {

    use super::*;

    #[test]
    fn test_hex_to_base() {
        let hex_in =
            String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let base64_out =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(hex_to_base64(hex_in), base64_out);
    }
}
