/// 变长编码
#[inline]
pub fn encode(input: u64, bits_count_of_fist_byte: u8) -> Vec<u8> {
    if input == 0 {
        return if bits_count_of_fist_byte == 1 {
            vec![0, 0]
        } else {
            vec![0]
        };
    }

    let mut value = input;
    let mut out = Vec::new();

    if bits_count_of_fist_byte == 1 {
        let val = (input & 1_u64) as u8;
        value = value >> 1;
        out.push(val);
        if value == 0 {
            // 特殊情况，如果第一个字节只有1个bit的时候，后面紧挨着的字节一定是数值
            // 由于input是1，第一个字节已经放完了，所有这里添加了一个0
            out.push(0);
        }
    } else if bits_count_of_fist_byte > 0 {
        let validate_bits = bits_count_of_fist_byte - 1_u8;
        let continuous_flag = 1_u8 << validate_bits;
        let validate_mask = continuous_flag - 1_u8;
        let val = (input & validate_mask as u64) as u8;
        value = value >> validate_bits;
        out.push(val | if value != 0 { continuous_flag } else { 0 });
    }
    if value == 0 {
        return out;
    }

    let validate_bits = 7_u8;
    let continuous_flag = 1_u8 << validate_bits;
    let validate_mask = continuous_flag - 1_u8;
    loop {
        let val = (value & validate_mask as u64) as u8;
        value = value >> validate_bits;
        if value > 0 {
            out.push(val | continuous_flag);
        } else {
            out.push(val);
            break;
        }
    }

    out
}

/// 变长编码
#[inline]
pub fn decode(input: &[u8], bits_count_of_fist_byte: u8) -> (u64, usize) {
    // input.iter().for_each(|x| {
    //     println!("input: {:08b}", x);
    // });

    let mut result = 0;
    let mut bits_cnt = 0;
    let mut index: usize = 0;

    // decode first byte
    if bits_count_of_fist_byte == 1 {
        result = (input[index] & 1) as u64;
        bits_cnt = 1;
        index += 1;
    } else if bits_count_of_fist_byte > 0 {
        let validate_bits = bits_count_of_fist_byte - 1_u8;
        let continuous_flag = 1_u8 << validate_bits;
        let validate_mask = continuous_flag - 1_u8;
        result = (input[index] & validate_mask) as u64;
        if input[index] & continuous_flag == 0 {
            return (result, index + 1);
        }
        bits_cnt = validate_bits;
        index += 1;
    }

    let validate_bits: u8 = 8_u8 - 1;
    let continuous_flag = 1_u8 << validate_bits;
    let validate_mask = continuous_flag - 1_u8;

    loop {
        let byte = input[index];
        let val = (byte & validate_mask) as u64;
        result = result | (val << bits_cnt);
        if (byte & continuous_flag) == 0 {
            return (result, index + 1);
        }
        bits_cnt += 7;
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::vle_variants::decode;

    #[test]
    fn test_encode() {
        let out = super::encode(0b_1_1101010_1001101_01100, 6);
        assert_eq!(
            out,
            vec![0b_0010_1100, 0b_1100_1101, 0b_1110_1010, 0b_0000_0001]
        );
    }

    #[test]
    fn test_encode_zero() {
        let out = super::encode(0b_1_1101010_1001101, 0);
        assert_eq!(out, vec![0b_1100_1101, 0b_1110_1010, 0b_0000_0001]);
    }
    #[test]
    fn test_encode_1() {
        let out = super::encode(0b_1101010_1001101_0, 1);
        assert_eq!(out, vec![0b_0000_0000, 0b_1100_1101, 0b_0110_1010]);
    }

    #[test]
    fn test_encode_1_1() {
        let out = super::encode(0b_0000_0001, 1);
        assert_eq!(out, vec![0b_0000_0001, 0b_0000_0000]);
    }

    #[test]
    fn test_all() {
        for i in 0..u8::MAX {
            let out = super::encode(i as u64, 0);
            assert_eq!(decode(out.as_slice(), 0).0, i as u64);
        }

        for i in 0..u8::MAX {
            let out = super::encode(i as u64, 1);
            assert_eq!(decode(out.as_slice(), 1).0, i as u64);
        }
        for i in 0..u8::MAX {
            let out = super::encode(i as u64, 5);
            assert_eq!(decode(out.as_slice(), 5).0, i as u64);
        }

        for i in [u64::MIN, 1258, 999999999, u64::MAX] {
            let out = super::encode(i as u64, 5);
            assert_eq!(decode(out.as_slice(), 5).0, i as u64);
        }

        for i in [u64::MIN, 1258, 999999999, u64::MAX] {
            let out = super::encode(i as u64, 1);
            assert_eq!(decode(out.as_slice(), 1).0, i as u64);
        }
        for i in [u64::MIN, 1258, 999999999, u64::MAX] {
            let out = super::encode(i as u64, 0);

            let len = out.len();
            assert_eq!(decode(out.as_slice(), 0), (i as u64, len));
        }
    }
}
