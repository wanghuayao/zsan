mod numeric_all_ascii;
mod space_all_ascii;
mod space_has_non_ascii;

use std::io::Bytes;

use numeric_all_ascii::*;
use space_all_ascii::*;
use space_has_non_ascii::*;

use crate::numeric;

macro_rules! fetch_and_continue {
    ($input:expr, $input_len:expr,$index:ident, $byte:ident) => {
        if $index < $input_len {
            $byte = $input[$index];
            $index += 1;
            continue;
        } else {
            break;
        }
    };
}

const SPACE_BYTE: u8 = 0x20; // ASCII value for space character

/// Header byte indicating that the compressed data contains non-ASCII characters.
/// In this mode, space counts are preceded by a `SPACE_BYTE` and followed by
/// one or more count bytes where the most significant bit (MSB) is 0.
/// Example: `[HEADER_MODE_NON_ASCII, 'A', SPACE_BYTE, 255, 45, 'B']` for "A" + 300 spaces + "B"
const HEADER_MODE_NON_ASCII: u8 = 0b_0010_0000;

/// Header byte indicating that all characters in the original data are ASCII.
/// In this mode, space counts are encoded directly into bytes where the MSB is 1,
/// and the lower 7 bits represent the count (max 127). No `SPACE_BYTE` marker is used.
/// Example: `[HEADER_MODE_ALL_ASCII, 'A', (127 | 0b_1000_0000), (127 | 0b_1000_0000), (46 | 0b_1000_0000), 'B']` for "A" + 300 spaces + "B"
const HEADER_MODE_ALL_ASCII: u8 = 0b_1010_0000;

/// Compresses a byte slice by applying Run-Length Encoding (RLE) specifically to consecutive spaces.
///
/// The compression strategy depends on whether the input contains any non-ASCII bytes.
/// A header byte is added to the `out` vector to indicate the chosen compression mode:
///
/// - **Mode 1: `HEADER_MODE_ALL_ASCII`** (All input bytes are standard ASCII, <= 0x7F)
///   - **Header:** `0b_1010_0000`
///   - **Compression:** Consecutive spaces are replaced by one or more count bytes.
///     Each count byte has its Most Significant Bit (MSB) set to 1 (`0b_1000_0000`),
///     and its lower 7 bits represent the number of spaces (max 127).
///     E.g., 3 spaces become `(3 | 0b_1000_0000)`.
///     For counts > 127, multiple count bytes are used (e.g., 300 spaces: `(127 | 0b_1000_0000), (127 | 0b_1000_0000), (46 | 0b_1000_0000)`).
///
/// - **Mode 2: `HEADER_MODE_NON_ASCII`** (Input bytes contain at least one non-ASCII byte, > 0x7F)
///   - **Header:** `0b_0010_0000`
///   - **Compression:** Consecutive spaces are replaced by a single `SPACE_BYTE` (0x20)
///     followed by one or more count bytes. These count bytes use their full 8 bits
///     to represent the number of spaces (max 255), with their MSB always being 0
///     (this is implicitly handled as the header differentiates the modes).
///     E.g., 3 spaces become `SPACE_BYTE, 3`.
///     For counts >= 255, multiple count bytes are used (e.g., 300 spaces: `SPACE_BYTE, 255, 45`).
///
/// # Arguments
/// * `input` - The byte slice to compress.
/// * `out` - A mutable vector where the compressed bytes will be written.
pub fn compress(input: &[u8], out: &mut Vec<u8>) {
    if input.is_empty() {
        return;
    }

    // Check if any non-ASCII characters are present in the input
    // Select the appropriate compression encoder function and push the header byte
    let space_encoder_fn = if input.is_ascii() {
        out.push(HEADER_MODE_ALL_ASCII);
        encode_spaces_mode_all_ascii
    } else {
        out.push(HEADER_MODE_NON_ASCII);
        encode_spaces_mode_non_ascii
    };

    let mut numeric_mode = false;
    let mut decimal_cnt: u8 = 0;
    let mut numeric_val: u64 = 0;
    let mut numeric_sign = Sign::Positive;

    let mut current_space_count: usize = 0;
    let input_len = input.len();
    let mut index = 0;

    let mut byte = input[index];
    index += 1;

    loop {
        // space
        if byte == b' ' {
            // space
            current_space_count += 1;
            fetch_and_continue!(input, input_len, index, byte);
        }

        if current_space_count > 0 {
            // encode the accumulated space count.
            space_encoder_fn(&mut current_space_count, out);
        }

        // numeris
        if byte == b'-' {
            if index >= input_len {
                out.push(b'-');
                break;
            }
            // enter numeric mode
            let byte = input[index];
            index += 1;
            if !byte.is_ascii_digit() {
                out.push(b'-');
                continue;
            }

            numeric_sign = Sign::Negative;
        } else if !byte.is_ascii_digit() {
            out.push(byte);

            fetch_and_continue!(input, input_len, index, byte);
        }

        // numeric

        if byte == b'0' {
            // must be 0.x

            if index + 2 > input_len {
                // not numeric
                if numeric_sign == Sign::Negative {
                    numeric_sign = Sign::Positive;
                    out.push(b'-');
                }
                out.push(b'0');

                fetch_and_continue!(input, input_len, index, byte);
            }

            byte = input[index];
            index += 1;
            if byte != b'.' {
                // not number
                if numeric_sign == Sign::Negative {
                    numeric_sign = Sign::Positive;
                    out.push(b'-');
                }
                out.push(b'0');
                continue;
            }

            // 0.
            byte = input[index];
            index += 1;
            if byte.is_ascii_digit() {
                // not number
                if numeric_sign == Sign::Negative {
                    numeric_sign = Sign::Positive;
                    out.push(b'-');
                }
                out.push(b'0');
                out.push(b'.');
                continue;
            }

            numeric_mode = true;
            decimal_cnt = 1;
            numeric_val = (byte - b'0') as u64;
        } else {
            // not zero
            numeric_mode = true;
            numeric_val = (byte - b'0') as u64;
        }

        // while cur < input_len {
        //     let b = index[cur];
        //     match b {
        //         b'0' => {
        //         }
        //     }
        // }

        if decimal_cnt == 0 {
            // integer mode
            // TODO integer mode
        }

        // decimal mode
        let mut need_fetch = true;
        while index < input_len {
            let byte = input[index];
            index += 1;
            if !byte.is_ascii_digit() {
                // end numeric encode number
                compress_numeric_in_all_ascii(numeric_val, decimal_cnt, numeric_sign, out);
                need_fetch = false;
                break;
            }

            decimal_cnt += 1;
            numeric_val = (numeric_val * 10) + (byte - b'0') as u64;

            if decimal_cnt >= 15 || numeric_val >= 999_999_999_999_999 {
                compress_numeric_in_all_ascii(numeric_val, decimal_cnt, numeric_sign, out);
                break;
            }
        }

        if need_fetch {
            fetch_and_continue!(input, input_len, index, byte);
        } else {
            continue;
        }
    }

    for &byte in input {
        if byte == SPACE_BYTE {
            // Accumulate consecutive space count
            current_space_count += 1;
            continue;
        }

        // Add the non-space character to the output
        out.push(byte);
    }

    // Handle any trailing spaces at the end of the input
    if current_space_count > 0 {
        space_encoder_fn(&mut current_space_count, out)
    }
}

/// Decompresses a byte slice that was compressed using the `compress` function.
/// It reads the header byte to determine the compression mode used and
/// then applies the corresponding decompression logic.
///
/// # Arguments
/// * `input` - The compressed byte slice.
/// * `out` - A mutable vector where the decompressed bytes will be written.
///
/// # Panics
/// Panics if the input slice is empty (cannot read header).
pub fn decompress(input: &[u8], out: &mut Vec<u8>) {
    if input.is_empty() {
        return;
    }
    let header = input[0];
    let data_slice: &[u8] = &input[1..];
    match header {
        HEADER_MODE_ALL_ASCII => decode_all_ascii(data_slice, out),
        HEADER_MODE_NON_ASCII => decode_spaces_mode_non_ascii(data_slice, out),
        _ => {
            // Handle unknown header - for robust applications, this might return a Result<_, Error>
            eprintln!(
                "Error: Unknown compression header byte detected: {:#02x}",
                header
            );
        }
    }
}
fn decode_all_ascii(input: &[u8], out: &mut Vec<u8>) {
    for &val in input {
        if is_space_char(val) {
            decode_spaces_in_all_ascii(val, out);
        } else {
            out.push(val);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn test_compress_all_ascii() {
        let input = "   a      ";
        let mut out = vec![];
        compress(input.as_bytes(), &mut out);
        assert_eq!(
            out,
            vec![
                HEADER_MODE_ALL_ASCII,
                3u8 | 0b_1100_0000,
                b'a',
                6u8 | 0b_1100_0000
            ]
        );
    }

    #[test]
    fn test_compress_has_non_ascii() {
        let input = format!("   你好   世界   A{}B ", " ".repeat(1001)); // 3个空格，你好，3个空格，世界，3个空格

        // '你' 和 '好' 是非 ASCII 字符
        let mut out = vec![];
        compress(input.as_bytes(), &mut out);

        let mut final_out = vec![];
        decompress(&out, &mut final_out);

        assert_eq!(final_out, input.as_bytes());
    }

    #[test]
    fn test_decompress() {
        let input = format!(
            "000050691530       3962U         -8      -0.20    4555300     993500CU   3960.053153000    "
        );
        let mut compress_out = vec![];
        compress(input.as_bytes(), &mut compress_out);

        println!(
            "ora len: {}, compress len: {}",
            input.as_bytes().len(),
            compress_out.len()
        );

        let mut out = vec![];
        decompress(&compress_out, &mut out);

        assert_eq!(out, input.as_bytes());
    }

    #[test]
    fn test_decompress_2() {
        let input = format!("{}", " ".repeat(1023));
        let mut compress_out = vec![];
        compress(input.as_bytes(), &mut compress_out);

        let mut out = vec![];
        decompress(&compress_out, &mut out);

        assert_eq!(out, input.as_bytes());
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let mut compressed_out = vec![];
        compress(input.as_bytes(), &mut compressed_out);
        assert!(compressed_out.is_empty()); // Empty input should result in empty compressed output

        let mut decompressed_out = vec![];
        decompress(&compressed_out, &mut decompressed_out);
        assert_eq!(decompressed_out, input.as_bytes());
    }

    #[test]
    fn test_decompress_empty_input() {
        let input = vec![HEADER_MODE_ALL_ASCII];

        let mut decompressed_out = vec![];
        decompress(&input, &mut decompressed_out);
        assert_eq!(decompressed_out, "".as_bytes());
    }

    #[test]
    fn test_decompress_wrong_header() {
        let input = vec![0b_0000_0001];
        let mut decompressed_out = vec![];
        decompress(&input, &mut decompressed_out);
        assert_eq!(decompressed_out, "".as_bytes());
    }
}
