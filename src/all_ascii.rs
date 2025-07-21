pub mod decimal;
pub mod integer;
pub mod space;
pub mod unsigned_decimal;
pub mod unsigned_integer;

/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
pub(super) const SPACE_HOLDER_FLAG: u8 = 0b_1000_0000;
/// The flag (MSB) applied to space count bytes in `HEADER_MODE_ALL_ASCII`.
pub(super) const NUMERICAL_HOLDER_FLAG: u8 = 0b_1100_0000;
