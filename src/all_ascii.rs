pub mod decimal;
pub mod integer;
pub mod space;
pub mod unsigned_decimal;
pub mod unsigned_integer;

/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
pub(super) const SPACE_HOLDER_FLAG: u8 = 0b_1000_0000;
/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
pub(super) const NUMERICAL_HOLDER_FLAG: u8 = 0b_1100_0000;

/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
pub(super) const HOLDER_MASK: u8 = 0b_1100_0000;

#[inline]
pub(super) fn is_space(b: u8) -> bool {
    b & HOLDER_MASK == SPACE_HOLDER_FLAG
}

#[inline]
pub(super) fn is_numerical(b: u8) -> bool {
    b & HOLDER_MASK == NUMERICAL_HOLDER_FLAG
}
