/// 最高两位是11, 代表这个位置是无符号整数，剩余的6个bit和后续字节表示整数值，整数值采用变长编码,
/// 例如: 64个空格会用0b_1011_1111和0b_1000_0001两个字节表示。
/// return true if the integer is compressed, false otherwise.
pub fn compress_unsigned_integer(
    val: u64,
    _negative: bool,
    _decimal_places: u8,
    out: &mut Vec<u8>,
) -> bool {
    if val < 10 || (val > 31 && val < 100) {
        return false;
    }

    let mut encoded = crate::vle_variants::encode_6(val);
    encoded[0] |= super::NUMERICAL_HOLDER_FLAG;
    out.extend_from_slice(encoded.as_slice());

    true
}

#[inline]
pub fn decompress_unsigned_integer(input: &[u8], out: &mut Vec<u8>) -> usize {
    let (mut value, len) = crate::vle_variants::decode_6(input);

    const MAX_LEN: usize = 20;
    let mut buf = [0u8; MAX_LEN];
    let mut index = MAX_LEN - 1;
    while value > 0 {
        let digit = (value % 10) as u8;
        buf[index] = b'0' + digit;
        value /= 10;
        index -= 1;
    }
    out.extend_from_slice(&buf[index + 1..]);

    len
}

#[cfg(test)]
mod test {
    #[test]
    fn test_unsigned_integer() {
        for x in 1..1000 {
            let mut out = Vec::new();
            if super::compress_unsigned_integer(x, false, 0, &mut out) {
                let result = format!("{}", x).into_bytes();
                println!("x:{x}\t {}\t>\t{}", result.len(), out.len());
                let mut final_out = Vec::new();
                super::decompress_unsigned_integer(out.as_slice(), &mut final_out);
                assert_eq!(final_out, result);
            }
        }
    }
}
