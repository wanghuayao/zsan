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

    let encoded = if decimal_places > 0 {
        let mut encoded: Vec<u8> = crate::vle_variants::encode_1(val);

        encoded[0] |= super::NUMERICAL_HOLDER_FLAG | DECIMAL_FLAG | (decimal_places << 1);
        encoded
    } else {
        let mut encoded: Vec<u8> = crate::vle_variants::encode_4(val);

        encoded[0] |= super::NUMERICAL_HOLDER_FLAG;
        encoded
    };

    out.extend_from_slice(encoded.as_slice());

    true
}

#[inline]
pub fn decompress_unsigned_decimal(input: &[u8], out: &mut Vec<u8>) -> usize {
    let (mut value, decimal_places, len) = if input[0] & DECIMAL_FLAG != 0 {
        let (value, len) = crate::vle_variants::decode_1(input);
        (value, (input[0] >> 1) & 0b_0000_1111, len)
    } else {
        let (value, len) = crate::vle_variants::decode_4(input);
        (value, 0, len)
    };

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
