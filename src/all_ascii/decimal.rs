const BITS_COUNT_OF_FIRST_BYTE: u8 = 4;
const BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL: u8 = 0;

const NEGATIVE_FLAG: u8 = 0b_0010_0000;
const DECIMAL_FLAG: u8 = 0b_0001_0000;

/// 最高两位是11, 代表这个位置是无符号整数，剩余的6个bit和后续字节表示整数值，整数值采用变长编码,
/// 例如: 64个空格会用0b_1011_1111和0b_1000_0001两个字节表示。
/// return true if the integer is compressed, false otherwise.
pub fn compress_decimal(val: u64, negative: bool, decimal_places: u8, out: &mut Vec<u8>) -> bool {
    if (!negative && decimal_places == 0) && val < 100 {
        return false;
    }

    let first_byte_msb = super::NUMERICAL_HOLDER_FLAG
        | if negative { NEGATIVE_FLAG } else { 0 }
        | if decimal_places > 0 { DECIMAL_FLAG } else { 0 };
    let encoded = if decimal_places > 0 {
        out.push(first_byte_msb | decimal_places);
        crate::vle_variants::encode(val, BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL)
    } else {
        let mut encoded = crate::vle_variants::encode(val, BITS_COUNT_OF_FIRST_BYTE);
        encoded[0] |= super::NUMERICAL_HOLDER_FLAG
            | if negative { NEGATIVE_FLAG } else { 0 }
            | if decimal_places > 0 { DECIMAL_FLAG } else { 0 };
        encoded
    };

    out.extend_from_slice(encoded.as_slice());

    true
}

#[inline]
pub fn decompress_decimal(input: &[u8], out: &mut Vec<u8>) -> usize {
    if input[0] & NEGATIVE_FLAG != 0 {
        out.push(b'-');
    }

    let mut size = 0;

    let (bits_count_of_fist_byte, decimal_places, input) = if input[0] & DECIMAL_FLAG != 0 {
        size += 1;
        (
            BITS_COUNT_OF_FIRST_BYTE_WITH_DECIMAL,
            input[0] & 0b_0000_1111,
            &input[1..],
        )
    } else {
        (BITS_COUNT_OF_FIRST_BYTE, 0, input)
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

    size + len
}

#[cfg(test)]
mod test {
    #[test]
    fn test_integer_positive() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_decimal(x, false, 0, &mut out) {
                let result = format!("{}", x).into_bytes();
                println!("x:{x}\t {}\t>\t{}", result.len(), out.len());
                let mut final_out = Vec::new();
                super::decompress_decimal(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result);
            }
        }
    }
    #[test]
    fn test_integer_negative() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_decimal(x, true, 0, &mut out) {
                let result = format!("-{}", x).into_bytes();
                println!("x:{x}\t {}\t>\t{}", result.len(), out.len());
                let mut final_out = Vec::new();
                super::decompress_decimal(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result);
            }
        }
    }

    #[test]
    fn test_decimal_positive() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_decimal(x, false, 1, &mut out) {
                let result = format!("{:.1}", x as f64 / 10.0);
                println!("x:{x}\t{}\t>\t{}\t{}", result.len(), out.len(), result);
                let mut final_out = Vec::new();
                super::decompress_decimal(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result.into_bytes());
            }
        }
    }
}
