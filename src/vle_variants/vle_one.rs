/// 变长编码
/// 第一个字节只有1个有效位
#[inline]
pub fn encode_1(input: u64) -> Vec<u8> {
    let mut value = input;
    let mut out = Vec::with_capacity(16);
    let val = (input & 1_u64) as u8;
    value = value >> 1;
    out.push(val);
    vle_encode_loop!(value, out);
    out
}

/// 变长编码
/// 第一个字节只有1个有效位
#[inline]
pub fn decode_1(input: &[u8]) -> (u64, usize) {
    // decode first byte
    let mut result = (input[0] & 1) as u64;
    let mut bits_cnt = 1;
    let mut index: usize = 1;
    vle_decode_loop!(input, result, bits_cnt, index);
    (result, index + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_1() {
        let out = super::encode_1(0b_1101010_1001101_0);
        assert_eq!(out, vec![0b_0000_0000, 0b_1100_1101, 0b_0110_1010]);
    }
    #[test]
    fn test_encode_1_1() {
        let out = super::encode_1(0b_0000_0001);
        assert_eq!(out, vec![0b_0000_0001, 0b_0000_0000]);
    }
    #[test]
    fn test_all() {
        for i in 0..u8::MAX {
            let out = super::encode_1(i as u64);
            assert_eq!(super::decode_1(out.as_slice()).0, i as u64);
        }
    }
}
