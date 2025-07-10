/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
const NUM_COUNT_FLAG: u8 = 0b_1000_0000;
const NUM_COUNT_MASK: u8 = 0b_1100_0000;

/// 0b_00?0_0000: ?=0: positive: ?=1: negative.
const NEGATIVE_FLAG: u8 = 0b_0010_0000;
const NEGATIVE_MASK: u8 = 0b_0010_0000;

/// 0b_000?_0000: ?=0: integer: ?=1: decimal.
const DECIMAL_FLAG: u8 = 0b_0001_0000;
const DECIMAL_MASK: u8 = 0b_0001_0000;

const DECIMAL_BYTE_LEGHT: usize = 4; // 3 bits
const DECIMAL_BYTE_MASK: u8 = (1 << DECIMAL_BYTE_LEGHT) - 1; // 3 bits
const DECIMAL_BYTE_MAX_VALUE: u8 = (1 << DECIMAL_BYTE_LEGHT) - 1; // 3 bits

const FIRST_BYTE_LEGHT: usize = 3; // 3 bits
const FIRST_BYTE_CONTINUANCE_FLAG: u8 = 1 << FIRST_BYTE_LEGHT; // 3 bits
const FIRST_BYTE_LEGHT_MASK: u64 = (1 << FIRST_BYTE_LEGHT) - 1; // 3 bits
const FIRST_BYTE_MAX_VALUE: u64 = (1 << FIRST_BYTE_LEGHT) - 1; // 3 bits

const FOLLOWING_BYTE_LEGHT: usize = 7; // 7 bits
const FOLLOWING_BYTE_LEGHT_FLAG: u8 = 1 << FOLLOWING_BYTE_LEGHT; // 7 bits
const FOLLOWING_BYTE_LEGHT_MASK: u64 = (1 << FOLLOWING_BYTE_LEGHT) - 1; // 7 bits

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Sign {
    Positive = 0b_0000_0000,
    Negative = NEGATIVE_FLAG as isize,
}

/// Encodes the count of consecutive spaces for the `HEADER_MODE_ALL_ASCII` compression.
/// Spaces are represented by one or more count bytes where the MSB is set to 1.
/// Each count byte uses its lower 7 bits to store the space count (max 127).
///
/// Example: If `count` is 300, `out` will receive `(127 | 0b_1000_0000), (127 | 0b_1000_0000), (46 | 0b_1000_0000)`.
///
pub(super) fn compress_numeric_in_all_ascii(
    mut value: u64,
    decimal_cnt: u8,
    sign: Sign,
    out: &mut Vec<u8>,
) {
    // ????_0000
    let msb = NUM_COUNT_FLAG | sign as u8 | if decimal_cnt > 0 { DECIMAL_FLAG } else { 0 };

    if decimal_cnt == 0 {
        // integer
        let first_byte = (value & FIRST_BYTE_LEGHT_MASK) as u8;
        value = value >> FIRST_BYTE_LEGHT;

        if value == 0 {
            // encode the first byte
            out.push(first_byte as u8 | NUM_COUNT_FLAG);
            return;
        }
        // encode the first byte
        out.push(first_byte as u8 | msb | FIRST_BYTE_CONTINUANCE_FLAG);
    } else {
        // float
        if decimal_cnt > DECIMAL_BYTE_MAX_VALUE {
            panic!(
                "The number of decimal places exceeds the maximum supported number of digits ({DECIMAL_BYTE_MAX_VALUE})"
            )
        }

        // first byte only include decimal length
        out.push(decimal_cnt as u8 | msb | DECIMAL_FLAG);
    }

    loop {
        let following_byte = (value & FOLLOWING_BYTE_LEGHT_MASK) as u8;
        value = value >> FOLLOWING_BYTE_LEGHT;

        if value == 0 {
            out.push(following_byte);
            break;
        }
        out.push(following_byte | FOLLOWING_BYTE_LEGHT_FLAG)
    }
}

/// Decompresses data encoded with `HEADER_MODE_ALL_ASCII`.
/// It interprets bytes with MSB=0 as regular characters and bytes with MSB=1 as space counts.
pub(super) fn decompress_numeric_in_all_ascii(mut value: u64, first_byte: u8, out: &mut Vec<u8>) {
    if (first_byte & NEGATIVE_MASK) > 0 {
        // negative sign
        out.push(b'-');
    }

    const BUFFER_SIZE: usize = 22;

    let mut buffer = [b'0'; BUFFER_SIZE]; // 固定大小的栈分配数组
    let mut start_index = BUFFER_SIZE; // 从数组的末尾开始填充
    while value > 0 {
        start_index -= 1; // 移动到前一个位置
        let digit = (value % 10) as u8; // 获取个位数
        buffer[start_index] = b'0' + digit;
        value /= 10; // 移除个位数
    }

    let decimal_len = if (first_byte & DECIMAL_MASK) > 0 {
        (first_byte & DECIMAL_BYTE_MASK) as usize
    } else {
        0
    };

    if decimal_len == 0 {
        // Integer
        out.extend_from_slice(&buffer[start_index..]);
    } else {
        let decimal_start_index = BUFFER_SIZE - decimal_len;

        // Integer parts
        // in case of 0.xx, so can't use start_index, instadeof use decimal_len - 1
        let integer_start_index = (start_index).min(decimal_start_index - 1);
        out.extend_from_slice(&buffer[integer_start_index..decimal_start_index]);

        // Dot
        out.push(b'.');

        // Decimal parts
        out.extend_from_slice(&buffer[decimal_start_index..]);
    }
}

#[inline]
pub(super) fn should_compress(val: u64) -> bool {
    if val >= 10 && val <= FIRST_BYTE_MAX_VALUE {
        // two byte -> 1 byte
        return true;
    }
    val > 100
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_compress_numeric_mode_all_ascii() {
        let mut out_len = 0;
        let mut max = 0;
        for val in 1..1000 {
            let mut out: Vec<u8> = vec![];
            super::compress_numeric_in_all_ascii(val, 0, super::Sign::Positive, &mut out);

            if out.len() > out_len {
                if max > 0 {
                    println!(" ~ {max}");
                }
                print!("{}\t{val}", out.len());
                out_len = out.len();
            }
            max = val;
        }
        println!(" ~ {max}");

        // Variable-Length Quantity
        // VLQ (或其变体 LEB128)

        let mut out = vec![];
        super::compress_numeric_in_all_ascii(
            0b110010_01101_0010,
            0,
            super::Sign::Positive,
            &mut out,
        );
        assert_eq!(out, vec![0b_1000_1010, 0b_1001_1010, 0b_0001_1001]);
    }

    #[test]
    fn test_decode_spaces_in_all_ascii() {
        let mut out = vec![];
        super::decompress_numeric_in_all_ascii(123, 0b0000_0000, &mut out);
        assert_eq!(out, vec![b'1', b'2', b'3']);
    }
    #[test]
    fn test_decode_spaces_in_all_ascii_nagivate() {
        let mut out = vec![];
        super::decompress_numeric_in_all_ascii(123, 0b0010_0000, &mut out);
        assert_eq!(out, vec![b'-', b'1', b'2', b'3']);
    }

    #[test]
    fn test_decode_spaces_in_all_ascii_decimal() {
        let mut out = vec![];
        super::decompress_numeric_in_all_ascii(123, 0b0011_0001, &mut out);
        assert_eq!(out, vec![b'-', b'1', b'2', b'.', b'3']);
    }
}
