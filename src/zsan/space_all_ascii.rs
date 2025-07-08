use crate::zsan::SPACE_BYTE;

const MAX_COUNT_PER_BYTE: u8 = 0b_0011_1111; // 63
const SPACE_OUNT_MASK: u8 = 0b_0011_1111; // 63

/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
const SPACE_COUNT_FLAG_ALL_ASCII: u8 = 0b_1100_0000; // 192

/// Encodes the count of consecutive spaces for the `HEADER_MODE_ALL_ASCII` compression.
/// Spaces are represented by one or more count bytes where the MSB is set to 1.
/// Each count byte uses its lower 7 bits to store the space count (max 127).
///
/// Example: If `count` is 300, `out` will receive `(127 | 0b_1000_0000), (127 | 0b_1000_0000), (46 | 0b_1000_0000)`.
pub(super) fn encode_spaces_mode_all_ascii(count: &mut usize, out: &mut Vec<u8>) {

    let current_out_len = out.len();
 
    // encode the rest
    let num_full_chunks = *count / MAX_COUNT_PER_BYTE as usize;
    let remainder_count = *count % MAX_COUNT_PER_BYTE as usize;
    if num_full_chunks > 0 {
        out.resize(
            num_full_chunks + current_out_len,
            MAX_COUNT_PER_BYTE | SPACE_COUNT_FLAG_ALL_ASCII,
        );
    }
    if remainder_count > 0 {
        out.push(remainder_count as u8 | SPACE_COUNT_FLAG_ALL_ASCII);
    }
    *count = 0; // Reset the space count
}

/// Decompresses data encoded with `HEADER_MODE_ALL_ASCII`.
/// It interprets bytes with MSB=0 as regular characters and bytes with MSB=1 as space counts.
#[inline]
pub(super) fn decode_spaces_in_all_ascii(input: u8, out: &mut Vec<u8>) {
            // MSB is 11, it's a space count byte
            // there is a space
            let space_count = input & SPACE_OUNT_MASK;
            out.resize(space_count as usize + out.len(), SPACE_BYTE);
}

#[inline]
pub(super) fn is_space_char(input: u8) -> bool {
    (input & SPACE_COUNT_FLAG_ALL_ASCII) == SPACE_COUNT_FLAG_ALL_ASCII
}


mod test {
    

    #[test]
    fn test_encode_spaces_mode_all_ascii() {
        let mut count = 127;
        let mut out = vec![];
        super::encode_spaces_mode_all_ascii(&mut count, &mut out);
        assert_eq!(out, vec![0b_1111_1111, 0b_1111_1111, 0b_1100_0001]);
    }

    #[test]
    fn test_encode_spaces_mode_all_ascii_2() {
        let mut count = 63;
        let mut out = vec![];
        super::encode_spaces_mode_all_ascii(&mut count, &mut out);
        assert_eq!(out, vec![0b_1111_1111]);
    }

    #[test]
    fn test_encode_spaces_mode_all_ascii_3() {
        let mut count = 62;
        let mut out = vec![];
        super::encode_spaces_mode_all_ascii(&mut count, &mut out);
        assert_eq!(out, vec![0b_1111_1110]);
    }

    #[test]
    fn test_decode_spaces_in_all_ascii() {
        let mut out = vec![];
        super::decode_spaces_in_all_ascii(0b_1100_0011, &mut out);
        assert_eq!(out, vec![super::SPACE_BYTE; 3]);
    }
}