use crate::zsan_parser::{Block, NumericalBlock, retrave_blocks};

/// 00:方式1:所有的Numerical都是正整数
const FIRST_BYTE_UNSIGNED_INTEGER: u8 = 0b_0000_0000;
/// 01:方式2:如果所有的Numerical都是整数但是包含负数
const FIRST_BYTE_INTEGER: u8 = 0b_0000_0001;
/// 10:方式3:如果所有的Numerical都是正数但是包含小数
const FIRST_BYTE_UNSIGNED_DECIMAL: u8 = 0b_0000_0010;
/// 11:方式4:所有的Numerical包含负数和小数
const FIRST_BYTE_DECIMAL: u8 = 0b_0000_0011;

const NEGATIVE_FLAG: u8 = 0b_0000_0001;
const DECIMAL_FLAG: u8 = 0b_0000_0010;

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

    out.push(first_byte);
    let mut last_index = 0;
    blocks.into_iter().for_each(|block| {
        let len = match block {
            Block::Space(start, size) => {
                println!("space: {start:?}, size: {size:?}");
                if start > last_index {
                    out.extend_from_slice(&src[last_index..start]);
                }
                crate::all_ascii::space::compress_space(size, out);
                size
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
                if start > last_index {
                    out.extend_from_slice(&src[last_index..start]);
                }
                numerical_compressor(base, negative, decimal_places as u8, out);
                size
            }
        };
        last_index += len
    });

    if last_index < src.len() {
        out.extend_from_slice(&src[last_index + 1..]);
    }
}

pub fn decompress(input: &[u8], out: &mut Vec<u8>) {
    if input.is_empty() {
        return;
    }
    let first_byte = input[0];
    let numerical_decompressor = match first_byte {
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
        } else if b & crate::all_ascii::SPACE_HOLDER_FLAG != 0 {
            crate::all_ascii::space::decompress_space(b, out);
            index += 1;
        } else if b & crate::all_ascii::NUMERICAL_HOLDER_FLAG != 0 {
            index += numerical_decompressor(input, out);
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
}
