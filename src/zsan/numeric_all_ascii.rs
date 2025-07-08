use crate::zsan::SPACE_BYTE;

/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
const NUM_COUNT_FLAG: u8 = 0b_1000_0000; // 192
const NUM_COUNT_MASK: u8 = 0b_1100_0000; // 192

const FIRST_BYTE_LEGHT: usize = 5; // 5 bits
const FIRST_BYTE_LEGHT_FLAG: u8 = 1 << FIRST_BYTE_LEGHT; // 5 bits
const FIRST_BYTE_LEGHT_MASK: u64 = (1 << FIRST_BYTE_LEGHT) - 1; // 5 bits
const FIRST_BYTE_MAX_VALUE: u64 = (1 << FIRST_BYTE_LEGHT) - 1; // 5 bits

const FOLLOWING_BYTE_LEGHT: usize = 7; // 7 bits
const FOLLOWING_BYTE_LEGHT_FLAG: u8 = 1 << FOLLOWING_BYTE_LEGHT; // 5 bits
const FOLLOWING_BYTE_LEGHT_MASK: u64 = (1 << FOLLOWING_BYTE_LEGHT) - 1; // 5 bits

/// Encodes the count of consecutive spaces for the `HEADER_MODE_ALL_ASCII` compression.
/// Spaces are represented by one or more count bytes where the MSB is set to 1.
/// Each count byte uses its lower 7 bits to store the space count (max 127).
///
/// Example: If `count` is 300, `out` will receive `(127 | 0b_1000_0000), (127 | 0b_1000_0000), (46 | 0b_1000_0000)`.
/// 
pub(super) fn compress_numeric_in_all_ascii(input: &[u8], count: &mut usize, value:u64, out: &mut Vec<u8>) {
    
    let first_byte  = (value & FIRST_BYTE_LEGHT_MASK) as u8;
    let mut value = value >> FIRST_BYTE_LEGHT;

    if value == 0 {
        // encode the first byte
        out.push(first_byte as u8 | NUM_COUNT_FLAG);
        *count = 0; // Reset the space count
        return;
    }

    // encode the first byte
    out.push(first_byte as u8 | NUM_COUNT_FLAG | FIRST_BYTE_LEGHT_FLAG);

    loop {  
        let following_byte = (value & FOLLOWING_BYTE_LEGHT_MASK) as u8;
        value = value >> FOLLOWING_BYTE_LEGHT;

        if value == 0 {
            out.push(following_byte );
            break;
        }
        out.push(following_byte | FOLLOWING_BYTE_LEGHT_FLAG)
    }

    *count = 0; // Reset the space count
}

/// Decompresses data encoded with `HEADER_MODE_ALL_ASCII`.
/// It interprets bytes with MSB=0 as regular characters and bytes with MSB=1 as space counts.
pub(super) fn uncompress_numeric_in_all_ascii(value: u64, out: &mut Vec<u8>) {
    // let value = 
}

#[inline]
pub(super)  fn should_compress(val: u64) -> bool {
    if val >= 10 && val <= FIRST_BYTE_MAX_VALUE {
        // two byte -> 1 byte
        return true;
    }
    val >100 
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_compress_numeric_mode_all_ascii() {
        let mut count = 0;
        
        for (index, &val) in vec![1,30,31,32,33,199999,1999999,19999999,199999999].iter().enumerate() {
            let mut out = vec![];
            super::compress_numeric_in_all_ascii(&[1, 2, 3], &mut count, val, &mut out);
            println!("{index}\t{val}\t{}",out.len());
        }

        let mut out = vec![];
        super::compress_numeric_in_all_ascii(&[1, 2, 3], &mut count, 0b110010_01101_0010, &mut out);

        assert_eq!(out, vec![0b_1011_0010, 0b_1010_0110, 0b_0000_0110]);
    }
}

