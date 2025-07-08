


/// Maximum space count that can be represented by a single byte in `HEADER_MODE_NON_ASCII`.
const SIMPLE_COMPRESS_MAX_SPACE_COUNT: u8 = u8::MAX; // 255

/// Encodes the count of consecutive spaces for the `HEADER_MODE_NON_ASCII` compression.
/// Spaces are represented by a `SPACE_BYTE` marker followed by one or more count bytes.
/// Each count byte can hold up to `u8::MAX` (255) spaces.
///
/// Example: If `count` is 300, `out` will receive `SPACE_BYTE, 255, 45`.
/// Example: If `count` is 255, `out` will receive `SPACE_BYTE, 255, 0`.
pub(super) fn encode_spaces_mode_non_ascii(count: &mut usize, out: &mut Vec<u8>) {
    out.push(super::SPACE_BYTE);
    let times = *count / SIMPLE_COMPRESS_MAX_SPACE_COUNT as usize;
    let remainder = *count % SIMPLE_COMPRESS_MAX_SPACE_COUNT as usize;
    if times > 0 {
        out.resize(times + out.len(), SIMPLE_COMPRESS_MAX_SPACE_COUNT);
    }
    // Add the remaining spaces
    out.push(remainder as u8);
    *count = 0; // Reset the space count
}

/// Decompresses data encoded with `HEADER_MODE_NON_ASCII`.
/// It expects `SPACE_BYTE` markers followed by one or more count bytes.
pub fn decode_spaces_mode_non_ascii(input: &[u8], out: &mut Vec<u8>) {
    let mut has_space = false;
    for &val in input {
        // space size
        if has_space {
            // out.extend(std::iter::repeat(SPACE).take(val as usize));
            out.resize(val as usize + out.len(), super::SPACE_BYTE);
            if val < SIMPLE_COMPRESS_MAX_SPACE_COUNT {
                has_space = false;
            }
            continue;
        }

        // not space
        if val != super::SPACE_BYTE {
            out.push(val);
            continue;
        }

        // space
        has_space = true;
    }
}