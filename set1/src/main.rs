#[derive(Debug, PartialEq)]
enum CPError {
    InputParseError,

}

#[derive(Debug, PartialEq)]
struct AppError {
    kind: CPError,
    message: String,
}

fn hex_to_base64(hex_in: String) -> Result<String, AppError> {
    // Output
    let mut b64_out = String::new();

    // Chart
    let b64_chars: [char; 65] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k',
        'j', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/', '=',
    ];

    let mut hex_u8s = Vec::new();
    for (i, _u) in hex_in.chars().enumerate().step_by(2) {
        let hex_u8_raw = match hex_in.get(i..i + 2) {
            Some(hex_u8_raw) => hex_u8_raw,
            None =>
                return Err(AppError {
                    kind: CPError::InputParseError,
                    message: format!("Incomplete hex dected at [{}]", i),
                })
        };

        let hex_u8 = match u8::from_str_radix(hex_u8_raw, 16) {
            Ok(hex_u8) => hex_u8,
            Err(e) =>
                return Err(AppError {
                    kind: CPError::InputParseError,
                    message: format!("Error parsing hex symbol[{}]@[{}]: {}", hex_u8_raw, i, e),
                })
        };

        hex_u8s.push(hex_u8);
    }

    // Alias -- we'll be using it a lot
    let h = &hex_u8s;
    for (i, _u) in hex_u8s.iter().enumerate().step_by(3) {
        // Decompose 3 octets into 4 sextets
        let (s1, s2, s3, s4);

        let remaining_octets = hex_u8s.len() - i;
        if remaining_octets > 3 {
            s1 = h[i] >> 2;
            s2 = h[i] << 6 >> 2 | h[i + 1] >> 4;
            s3 = h[i + 1] << 4 >> 2 | h[i + 2] >> 6;
            s4 = h[i + 2] << 2 >> 2;
        } else {
            match remaining_octets % 3 {
                0 => {
                    // Perfectly padded
                    s1 = h[i] >> 2;
                    s2 = h[i] << 6 >> 2 | h[i + 1] >> 4;
                    s3 = h[i + 1] << 4 >> 2 | h[i + 2] >> 6;
                    s4 = h[i + 2] << 2 >> 2;
                }
                1 => {
                    s1 = h[i] >> 2;
                    s2 = h[i] << 6 >> 2;
                    s3 = 64;
                    s4 = 64;
                }
                2 => {
                    s1 = h[i] >> 2;
                    s2 = h[i] << 6 >> 2 | h[i + 1] >> 4;
                    s3 = h[i + 1] << 4 >> 2;
                    s4 = 64;
                }
                _ => panic!("Not sure how we're getting here")
            }
        }

        // Use each sextet to map to b64 character
        b64_out.push(b64_chars[s1 as usize]);
        b64_out.push(b64_chars[s2 as usize]);
        b64_out.push(b64_chars[s3 as usize]);
        b64_out.push(b64_chars[s4 as usize]);
    }

    Ok(b64_out)
}

#[cfg(test)]
mod set1_tests {

    use super::*;

    #[test]
    fn test_hex_to_b64_no_padding() {
        let hex_in =
            String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let b64_expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        assert_eq!(hex_to_base64(hex_in).unwrap(), b64_expected);
    }

    #[test]
    fn test_hex_to_b64_pad_1_octet() {
        let hex_in =
            String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d6d");
        let b64_expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29tbQ==");

        assert_eq!(hex_to_base64(hex_in).unwrap(), b64_expected);
    }

    #[test]
    fn test_hex_to_b64_pad_2_octets() {
        let hex_in =
            String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d6d6d");
        let b64_expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29tbW0=");

        assert_eq!(hex_to_base64(hex_in).unwrap(), b64_expected);
    }

    #[test]
    fn test_hex_to_b64_invalid_input() {
        let hex_in_invalid_input = String::from("4xx");

        match hex_to_base64(hex_in_invalid_input) {
            Ok(_v) => panic!("This cannot be"),
            Err(e) => {
                assert_eq!(e.kind, CPError::InputParseError);
                assert_eq!(e.message, "Error parsing hex symbol[4x]@[0]: invalid digit found in string");
            }
        }

        let hex_in_invalid_octet = String::from("4");
        match hex_to_base64(hex_in_invalid_octet) {
            Ok(_v) => panic!("This cannot be"),
            Err(e) => {
                assert_eq!(e.kind, CPError::InputParseError);
                assert_eq!(e.message, "Incomplete hex dected at [0]");
            }
        }
    }
}
