const MAX_COUNT_PER_BYTE: u8 = 0b_0011_1111; // 63
const SPACE_OUNT_MASK: u8 = 0b_0011_1111; // 63

/// 最高两位是10, 代表这个位置是空格，剩余的6个bit代表空格个数，如果空格数量超过 0b_0011_1111, 那么就再用一个字节来表示,
/// 例如: 64个空格会用0b_1011_1111和0b_1000_0001两个字节表示。
///
pub fn compress_space(count: usize, out: &mut Vec<u8>) {
    if count == 1 {
        out.push(b' ');
    }
    let current_out_len = out.len();

    // encode the rest
    let num_full_chunks = count / MAX_COUNT_PER_BYTE as usize;
    let remainder_count = count % MAX_COUNT_PER_BYTE as usize;
    if num_full_chunks > 0 {
        out.resize(
            num_full_chunks + current_out_len,
            MAX_COUNT_PER_BYTE | super::SPACE_HOLDER_FLAG,
        );
    }
    if remainder_count > 0 {
        out.push(remainder_count as u8 | super::SPACE_HOLDER_FLAG);
    }
}

#[inline]
pub fn decompress_space(input: u8, out: &mut Vec<u8>) -> usize {
    // MSB is 11, it's a space count byte
    // there is a space
    let space_count = input & SPACE_OUNT_MASK;
    out.resize(space_count as usize + out.len(), b' ');
    space_count as usize
}

#[cfg(test)]
mod test {
    #[test]
    fn test_space() {
        let x = " ".repeat(65);

        let mut out = Vec::new();
        super::compress_space(x.len(), &mut out);
        assert_eq!(out, vec![0b_1011_1111, 0b_1000_0010]);

        let mut final_out = Vec::new();
        for i in out {
            println!("out: {:08b}", i);
            super::decompress_space(i, &mut final_out);
        }

        assert_eq!(final_out, x.as_bytes());
    }
}
