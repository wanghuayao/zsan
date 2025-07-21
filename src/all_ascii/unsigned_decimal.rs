const BITS_COUNT_OF_FIRST_BYTE: u8 = 4;
const BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL: u8 = 1;

const DECIMAL_FLAG: u8 = 0b_0010_0000;

/// 最高两位是11, 代表这个位置是无符号整数，剩余的6个bit和后续字节表示整数值，整数值采用变长编码,
/// 例如: 64个空格会用0b_1011_1111和0b_1000_0001两个字节表示。
/// return true if the integer is compressed, false otherwise.
pub fn compress_unsigned_decimal(
    val: u64,
    _negative: bool,
    decimal_places: u8,
    out: &mut Vec<u8>,
) -> bool {
    if decimal_places == 0 && val < 100 {
        return false;
    }

    let (first_byte_msb, bits_count_of_fist_byte) = if decimal_places > 0 {
        (
            super::NUMERICAL_HOLDER_FLAG | DECIMAL_FLAG | (decimal_places << 1),
            BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL,
        )
    } else {
        (super::NUMERICAL_HOLDER_FLAG, BITS_COUNT_OF_FIRST_BYTE)
    };
    let mut encoded: Vec<u8> = crate::vle_variants::encode(val, bits_count_of_fist_byte);

    encoded[0] |= first_byte_msb;

    out.extend_from_slice(encoded.as_slice());

    true
}

#[inline]
pub fn decompress_unsigned_decimal(input: &[u8], out: &mut Vec<u8>) -> usize {
    let (bits_count_of_fist_byte, decimal_places) = if input[0] & DECIMAL_FLAG != 0 {
        (
            BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL,
            (input[0] >> 1) & 0b_0000_1111,
        )
    } else {
        (BITS_COUNT_OF_FIRST_BYTE, 0)
    };

    let (mut value, len) = crate::vle_variants::decode(input, bits_count_of_fist_byte);

    const MAX_LEN: usize = 20;
    let mut buf = [b'0'; MAX_LEN];
    let mut index = MAX_LEN - 1;
    while value > 0 {
        let digit = (value % 10) as u8;
        buf[index] = b'0' + digit;
        value /= 10;
        index -= 1;
    }
    if decimal_places > 0 {
        let integer_end = MAX_LEN - decimal_places as usize;
        let integer_start = (index + 1).min(integer_end - 1);
        out.extend_from_slice(&buf[integer_start..integer_end]);
        out.push(b'.');
        out.extend_from_slice(&buf[integer_end..]);
    } else {
        out.extend_from_slice(&buf[index + 1..]);
    }

    len
}

#[cfg(test)]
mod test {
    #[test]
    fn test_integer_positive() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_unsigned_decimal(x, false, 0, &mut out) {
                let result = format!("{}", x).into_bytes();
                println!("x:{x}\t {}\t>\t{}", result.len(), out.len());
                let mut final_out = Vec::new();
                super::decompress_unsigned_decimal(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result);
            }
        }
    }

    #[test]
    fn test_decimal_positive() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_unsigned_decimal(x, false, 1, &mut out) {
                let result = format!("{:.1}", x as f64 / 10.0);
                println!("x:{x}\t{}\t>\t{}\t{}", result.len(), out.len(), result);
                let mut final_out = Vec::new();
                super::decompress_unsigned_decimal(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result.into_bytes());
            }
        }
    }
}
