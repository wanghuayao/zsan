/// 变长编码
#[inline]
pub fn encode_0(input: u64) -> Vec<u8> {
    let mut value = input;
    let mut out = Vec::with_capacity(16);
    vle_encode_loop!(value, out);
    out
}

/// 变长编码
#[inline]
pub fn decode_0(input: &[u8]) -> (u64, usize) {
    let mut result = 0;
    let mut bits_cnt = 0;
    let mut index: usize = 0;
    vle_decode_loop!(input, result, bits_cnt, index);
    (result, index + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let out = super::encode_0(0b_1_1101010_1001101_01100);
        out.iter().for_each(|i| {
            println!("{i:08b}");
        });
        assert_eq!(out, vec![0b_1010_1100, 0b_1101_0011, 0b_0011_1010]);
    }

    #[test]
    fn test_all() {
        for i in [u64::MIN, 1258, 999999999, u64::MAX] {
            let out = super::encode_0(i as u64);
            let len = out.len();
            assert_eq!(super::decode_0(out.as_slice()), (i as u64, len));
        }
    }
}
