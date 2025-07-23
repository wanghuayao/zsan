pub(super) const VLE_VALIDATE_BITS: u8 = 7_u8;
pub(super) const VLE_CONTINUOUS_FLAG: u8 = 1_u8 << VLE_VALIDATE_BITS;
pub(super) const VLE_VALIDATE_BITS_MASK: u8 = VLE_CONTINUOUS_FLAG - 1_u8;

// Macro to encapsulate the VLE encoding loop logic
#[macro_export]
macro_rules! vle_encode_loop {
    ($value:ident, $out:ident) => {
        loop {
            let val = ($value & super::VLE_VALIDATE_BITS_MASK as u64) as u8;
            $value = $value >> super::VLE_VALIDATE_BITS;
            if $value > 0 {
                $out.push(val | super::VLE_CONTINUOUS_FLAG);
            } else {
                $out.push(val);
                break;
            }
        }
    };
}

// Macro to encapsulate the VLE decoding loop logic
#[macro_export]
macro_rules! vle_decode_loop {
    ($input:expr, $result:ident, $bits_cnt:ident, $index:ident) => {
        loop {
            let byte = $input[$index];
            let val = (byte & super::VLE_VALIDATE_BITS_MASK) as u64;
            $result = $result | (val << $bits_cnt);
            if (byte & super::VLE_CONTINUOUS_FLAG) == 0 {
                break;
            }
            $bits_cnt += 7;
            $index += 1;
        }
    };
}

mod vle_n;
mod vle_one;
mod vle_zero;

pub(crate) use vle_n::*;
pub(crate) use vle_one::*;
pub(crate) use vle_zero::*;
