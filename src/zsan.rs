use crate::zsan_parser::{Block, NumericalBlock, retrave_blocks};

const ZSAN_FLAG_MASK: u8 = 0b_0100_0000;

/// 00:方式1:所有的Numerical都是正整数
const FIRST_BYTE_UNSIGNED_INTEGER: u8 = 0b_0000_0000;
/// 01:方式2:如果所有的Numerical都是整数但是包含负数
const FIRST_BYTE_INTEGER: u8 = 0b_0000_0010;

/// 10:方式3:如果所有的Numerical都是正数但是包含小数
const FIRST_BYTE_UNSIGNED_DECIMAL: u8 = 0b_0000_0001;
/// 11:方式4:所有的Numerical包含负数和小数
const FIRST_BYTE_DECIMAL: u8 = 0b_0000_0011;

const ENCODE_MODE_MASK: u8 = 0b_0000_0011;

const NEGATIVE_FLAG: u8 = 0b_0000_0010;
const DECIMAL_FLAG: u8 = 0b_0000_0001;

pub fn compress(src: &str, out: &mut Vec<u8>) {
    if src.is_empty() {
        return;
    }
    let src = src.as_bytes();
    let (has_negative, has_decimal, blocks) = retrave_blocks(src);

    let first_byte =
        if has_negative { NEGATIVE_FLAG } else { 0 } | if has_decimal { DECIMAL_FLAG } else { 0 };

    let numerical_compressor = match first_byte {
        FIRST_BYTE_UNSIGNED_INTEGER => {
            crate::all_ascii::unsigned_integer::compress_unsigned_integer
        }
        FIRST_BYTE_INTEGER => crate::all_ascii::integer::compress_integer,
        FIRST_BYTE_UNSIGNED_DECIMAL => {
            crate::all_ascii::unsigned_decimal::compress_unsigned_decimal
        }
        FIRST_BYTE_DECIMAL => crate::all_ascii::decimal::compress_decimal,
        _ => panic!("invalid first byte"),
    };

    out.push(first_byte | ZSAN_FLAG_MASK);
    let mut processed_len = 0;
    blocks.into_iter().for_each(|block| {
        match block {
            Block::Space(start, size) => {
                if start > processed_len {
                    out.extend_from_slice(&src[processed_len..start]);
                    processed_len += start - processed_len;
                }
                crate::all_ascii::space::compress_space(size, out);
                processed_len += size;
            }
            Block::Numerical(
                start,
                size,
                NumericalBlock {
                    base,
                    negative,
                    decimal_places,
                },
            ) => {
                if start > processed_len {
                    out.extend_from_slice(&src[processed_len..start]);
                    processed_len += start - processed_len;
                }
                if !numerical_compressor(base, negative, decimal_places as u8, out) {
                    out.extend_from_slice(&src[processed_len..processed_len + size]);
                }

                processed_len += size;
            }
        };
    });

    if processed_len < src.len() {
        out.extend_from_slice(&src[processed_len..]);
    }
}

pub fn decompress(input: &[u8], out: &mut Vec<u8>) {
    if input.is_empty() {
        return;
    }
    let first_byte = input[0];
    let numerical_decompressor = match first_byte & ENCODE_MODE_MASK {
        FIRST_BYTE_UNSIGNED_INTEGER => {
            crate::all_ascii::unsigned_integer::decompress_unsigned_integer
        }
        FIRST_BYTE_INTEGER => crate::all_ascii::integer::decompress_integer,
        FIRST_BYTE_UNSIGNED_DECIMAL => {
            crate::all_ascii::unsigned_decimal::decompress_unsigned_decimal
        }
        FIRST_BYTE_DECIMAL => crate::all_ascii::decimal::decompress_decimal,
        _ => panic!("invalid first byte"),
    };

    let mut index = 1;
    while index < input.len() {
        let &b = unsafe { input.get_unchecked(index) };
        if b < 0b_0111_1111 {
            out.push(b);
            index += 1;
        } else if crate::all_ascii::is_space(b) {
            let _ = crate::all_ascii::space::decompress_space(b, out);
            index += 1;
        } else if crate::all_ascii::is_numerical(b) {
            index += numerical_decompressor(&input[index..], out);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_compress() {
        let input = "A  B";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }

        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);
        for o in final_out.iter() {
            println!("out\t{:08b}", o);
        }

        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_integer_compression() {
        let input = "A   123  B";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }

        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);

        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_unsigned_integer_compression() {
        let input = "A   123  B-1234";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }

        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);

        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_decimal_compression() {
        let input = "A   -1.23  B0.1-234";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }
        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);
        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_unsigned_decimal_compression() {
        let input = "A   123  B0.1234";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }
        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);
        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_one_space() {
        let input = "123 ";
        let mut out = Vec::new();
        super::compress(input, &mut out);
        for o in out.iter() {
            println!("{:08b}", o);
        }
        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);
        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }

    #[test]
    fn test_mixed_types() {
        let input = "30";
        let mut out = Vec::new();
        super::compress(input, &mut out);

        for o in out.iter() {
            println!("{:08b}", o);
        }

        let mut final_out = vec![];
        super::decompress(&out, &mut final_out);
        assert_eq!(String::from_utf8(final_out).unwrap(), input);
    }
}
