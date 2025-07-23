/// neightly feature #![feature(macro_metavar_expr_concat)]
/// 变长编码
/// 首字节有效位数大于1
// macro_rules! vle_n {
//   ($n_val: literal, $n_name: literal) => {
//     const ${ concat(VALIDATE_BITS_, $n_name) }:u8 = $n_val - 1_u8;
//     const ${ concat(CONTINUOUS_FLAG_, $n_name) }:u8 = 1_u8 << ${ concat(VALIDATE_BITS_, $n_name) };
//     const ${ concat(VALIDATE_MASK_, $n_name) }:u8 = ${ concat(CONTINUOUS_FLAG_, $n_name) } - 1_u8;
//     #[inline]
//     pub fn ${ concat(encode_, $n_name) }(input: u64) -> Vec<u8> {
//       let mut value = input;
//       let mut out = Vec::new();
//       let val = (input & ${ concat(VALIDATE_MASK_, $n_name) } as u64) as u8;
//       value = value >> ${ concat(VALIDATE_BITS_, $n_name) };
//       if value > 0 {
//          out.push(val | ${ concat(CONTINUOUS_FLAG_, $n_name) });
//          vle_encode_loop!(value, out);
//       } else {
//          out.push(val);
//       }
//       out
//     }

//     /// 变长编码
//     /// 首字节有效位数大于1
//     #[inline]
//     pub fn ${ concat(decode_, $n_name) }(input: &[u8]) -> (u64, usize) {
//       let mut result = (input[0] & ${ concat(VALIDATE_MASK_, $n_name) }) as u64;
//       if input[0] & ${ concat(CONTINUOUS_FLAG_, $n_name) } == 0 {
//         return (result, 1);
//       }
//       let mut bits_cnt = ${ concat(VALIDATE_BITS_, $n_name) };
//       let mut index = 1;

//       vle_decode_loop!(input, result, bits_cnt, index);
//       (result, index + 1)
//     }
//   };

// }

// vle_n!(4, "4");
// vle_n!(5, "5");

const VALIDATE_BITS_4: u8 = 4 - 1_u8;
const CONTINUOUS_FLAG_4: u8 = 1_u8 << VALIDATE_BITS_4;
const VALIDATE_MASK_4: u8 = CONTINUOUS_FLAG_4 - 1_u8;
#[inline]
pub fn encode_4(input: u64) -> Vec<u8> {
    let mut value = input;
    let mut out = Vec::with_capacity(16);
    let val = (input & VALIDATE_MASK_4 as u64) as u8;
    value = value >> VALIDATE_BITS_4;
    if value > 0 {
        out.push(val | CONTINUOUS_FLAG_4);
        vle_encode_loop!(value, out);
    } else {
        out.push(val);
    }
    out
}

/// 变长编码
/// 首字节有效位数大于1
#[inline]
pub fn decode_4(input: &[u8]) -> (u64, usize) {
    let mut result = (input[0] & VALIDATE_MASK_4) as u64;
    if input[0] & CONTINUOUS_FLAG_4 == 0 {
        return (result, 1);
    }
    let mut bits_cnt = VALIDATE_BITS_4;
    let mut index = 1;

    vle_decode_loop!(input, result, bits_cnt, index);
    (result, index + 1)
}

const VALIDATE_BITS_5: u8 = 5 - 1_u8;
const CONTINUOUS_FLAG_5: u8 = 1_u8 << VALIDATE_BITS_5;
const VALIDATE_MASK_5: u8 = CONTINUOUS_FLAG_5 - 1_u8;
#[inline]
pub fn encode_5(input: u64) -> Vec<u8> {
    let mut value = input;
    let mut out = Vec::with_capacity(16);
    let val = (input & VALIDATE_MASK_5 as u64) as u8;
    value = value >> VALIDATE_BITS_5;
    if value > 0 {
        out.push(val | CONTINUOUS_FLAG_5);
        vle_encode_loop!(value, out);
    } else {
        out.push(val);
    }
    out
}

/// 变长编码
/// 首字节有效位数大于1
#[inline]
pub fn decode_5(input: &[u8]) -> (u64, usize) {
    let mut result = (input[0] & VALIDATE_MASK_5) as u64;
    if input[0] & CONTINUOUS_FLAG_5 == 0 {
        return (result, 1);
    }
    let mut bits_cnt = VALIDATE_BITS_5;
    let mut index = 1;

    vle_decode_loop!(input, result, bits_cnt, index);
    (result, index + 1)
}

const VALIDATE_BITS_6: u8 = 6 - 1_u8;
const CONTINUOUS_FLAG_6: u8 = 1_u8 << VALIDATE_BITS_6;
const VALIDATE_MASK_6: u8 = CONTINUOUS_FLAG_6 - 1_u8;
#[inline]
pub fn encode_6(input: u64) -> Vec<u8> {
    let mut value = input;
    let mut out = Vec::with_capacity(16);
    let val = (input & VALIDATE_MASK_6 as u64) as u8;
    value = value >> VALIDATE_BITS_6;
    if value > 0 {
        out.push(val | CONTINUOUS_FLAG_6);
        vle_encode_loop!(value, out);
    } else {
        out.push(val);
    }

    out
}

/// 变长编码
/// 首字节有效位数大于1
#[inline]
pub fn decode_6(input: &[u8]) -> (u64, usize) {
    let mut result = (input[0] & VALIDATE_MASK_6) as u64;
    if input[0] & CONTINUOUS_FLAG_6 == 0 {
        return (result, 1);
    }
    let mut bits_cnt = VALIDATE_BITS_6;
    let mut index = 1;

    vle_decode_loop!(input, result, bits_cnt, index);
    (result, index + 1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode() {
        let out = super::encode_6(0b_1_1101010_1001101_01100);
        out.iter().for_each(|i| println!("{i:08b}"));
        assert_eq!(
            out,
            vec![0b_0010_1100, 0b_1100_1101, 0b_1110_1010, 0b_0000_0001]
        );
    }

    #[test]
    fn test_all() {
        for i in 0..u8::MAX {
            let out = super::encode_5(i as u64);
            assert_eq!(super::decode_5(out.as_slice()).0, i as u64);
        }

        for i in [u64::MIN, 1258, 999999999, u64::MAX] {
            let out = super::encode_5(i as u64);
            assert_eq!(super::decode_5(out.as_slice()).0, i as u64);
        }
    }
}
