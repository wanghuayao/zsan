#[derive(Debug, PartialEq)]
pub(crate) struct NumericalBlock {
    pub base: u64,
    pub negative: bool,
    pub decimal_places: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Block {
    Numerical(usize, usize, NumericalBlock),
    Space(usize, usize),
}

const MAX_DECIMAL_PLACES: usize = 15;

pub fn retrave_blocks(src: &[u8]) -> (bool, bool, Vec<Block>) {
    let mut has_negative = false;
    let mut has_decimal = false;

    let s = src;
    let n = s.len();
    let mut out = Vec::new();
    let mut idx = 0;
    const MAX: u64 = 999_999_999_999_999_999;

    // 空格
    macro_rules! flush_space {
        ($i:expr) => {{
            let mut j = $i;
            while j < n && s[j] == b' ' {
                j += 1;
            }
            if j > $i {
                out.push(Block::Space($i, j - $i));
            }
            j
        }};
    }

    while idx < n {
        println!("idx: {},{:?}", idx, out);

        idx = flush_space!(idx);
        if idx >= n {
            break;
        }

        let start = idx;
        let neg = s[idx] == b'-';
        idx += neg as usize;

        // ---------- 整数 ----------
        let mut base: u64 = 0;
        let mut integer_end = idx;

        if idx < n && s[idx] == b'0' {
            integer_end = idx + 1;
            base = 0;
        } else {
            while integer_end < n && s[integer_end].is_ascii_digit() {
                let d = (s[integer_end] - b'0') as u64;
                match base.checked_mul(10).and_then(|v| v.checked_add(d)) {
                    Some(v) if v <= MAX => base = v,
                    _ => break, // 溢出
                }
                integer_end += 1;
            }
            if integer_end == idx {
                idx += 1;
                continue;
            } // 无整数
        }
        idx = integer_end;

        // ---------- 小数 ----------
        let mut dec_pl = 0;
        if idx < n && s[idx] == b'.' {
            let mut tmp = base; // 备份，方便回滚
            idx += 1;
            let mut frac_idx = idx;
            while frac_idx < n && s[frac_idx].is_ascii_digit() {
                let d = (s[frac_idx] - b'0') as u64;
                match tmp.checked_mul(10).and_then(|v| v.checked_add(d)) {
                    Some(v) if v <= MAX => {
                        tmp = v;
                        frac_idx += 1;
                        dec_pl += 1;
                    }
                    _ => break, // 溢出
                }
            }
            base = tmp;
            idx = frac_idx;
        }

        // ---------- 推结果 ----------
        if base != 0 || dec_pl != 0 {
            if neg {
                has_negative = true;
            }
            out.push(Block::Numerical(
                start,
                if dec_pl == 0 {
                    integer_end - start
                } else {
                    has_decimal = true;
                    integer_end - start + dec_pl + 1
                },
                NumericalBlock {
                    base,
                    negative: neg,
                    decimal_places: dec_pl,
                },
            ));
        }
    }
    (has_negative, has_decimal, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_large() {
        let (has_negative, has_decimal, out) = retrave_blocks("99999999999999999988".as_bytes());
        assert_eq!(has_negative, false);
        assert_eq!(has_decimal, false);
        assert_eq!(
            out,
            vec![
                Block::Numerical(
                    0,
                    18,
                    NumericalBlock {
                        base: 999_999_999_999_999_999,
                        negative: false,
                        decimal_places: 0
                    }
                ),
                Block::Numerical(
                    18,
                    2,
                    NumericalBlock {
                        base: 88,
                        negative: false,
                        decimal_places: 0
                    }
                ),
            ]
        );
    }

    #[test]
    fn split_decimal() {
        let (has_negative, has_decimal, out) = retrave_blocks("99.999999999999999988".as_bytes());
        assert_eq!(has_negative, false);
        assert_eq!(has_decimal, true);
        assert_eq!(
            out,
            vec![
                Block::Numerical(
                    0,
                    19,
                    NumericalBlock {
                        base: 999_999_999_999_999_999,
                        negative: false,
                        decimal_places: 16
                    }
                ),
                Block::Numerical(
                    19,
                    2,
                    NumericalBlock {
                        base: 88,
                        negative: false,
                        decimal_places: 0
                    }
                ),
            ]
        );
    }

    #[test]
    fn mix() {
        let input = "  -12.050  007.8  +3.00  0.0  abc   ";
        let (has_negative, has_decimal, out) = retrave_blocks(input.as_bytes());
        assert_eq!(has_negative, true);
        assert_eq!(has_decimal, true);
        assert_eq!(
            out,
            vec![
                Block::Space(0, 2),
                Block::Numerical(
                    2,
                    7,
                    NumericalBlock {
                        base: 12050,
                        negative: true,
                        decimal_places: 3
                    }
                ),
                Block::Space(9, 2),
                Block::Numerical(
                    13,
                    3,
                    NumericalBlock {
                        base: 78,
                        negative: false,
                        decimal_places: 1
                    }
                ),
                Block::Space(16, 2),
                Block::Numerical(
                    19,
                    4,
                    NumericalBlock {
                        base: 300,
                        negative: false,
                        decimal_places: 2
                    }
                ),
                Block::Space(23, 2),
                Block::Numerical(
                    25,
                    3,
                    NumericalBlock {
                        base: 0,
                        negative: false,
                        decimal_places: 1
                    }
                ),
                Block::Space(28, 2),
                Block::Space(33, 3),
            ]
        );
    }
}
