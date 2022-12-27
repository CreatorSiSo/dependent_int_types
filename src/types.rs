use crate::constants::uint_max_lut;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Int {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArbitraryInt {
	data: u128,
	length: u8,
}

impl ArbitraryInt {
	pub fn new(data: u128, length: u8) -> Self {
		Self { data, length }
	}

	pub fn resize(mut self) -> Self {
		for (index, max) in uint_max_lut().iter().enumerate() {
			if self.data <= *max {
				self.length = index as u8 + 1;
				break;
			}
		}
		self
	}
}

#[test]
fn test_shrink() {
	let expect = [
		(ArbitraryInt::new(1, 128), ArbitraryInt::new(1, 1)),
		(ArbitraryInt::new(3, 128), ArbitraryInt::new(3, 2)),
		(ArbitraryInt::new(7, 128), ArbitraryInt::new(7, 3)),
		(ArbitraryInt::new(15, 128), ArbitraryInt::new(15, 4)),
		(ArbitraryInt::new(31, 128), ArbitraryInt::new(31, 5)),
		(ArbitraryInt::new(63, 128), ArbitraryInt::new(63, 6)),
		(ArbitraryInt::new(127, 128), ArbitraryInt::new(127, 7)),
		(ArbitraryInt::new(255, 128), ArbitraryInt::new(255, 8)),
		(ArbitraryInt::new(511, 128), ArbitraryInt::new(511, 9)),
		(ArbitraryInt::new(1023, 128), ArbitraryInt::new(1023, 10)),
		(ArbitraryInt::new(2047, 128), ArbitraryInt::new(2047, 11)),
		(ArbitraryInt::new(4095, 128), ArbitraryInt::new(4095, 12)),
		(
			ArbitraryInt::new(u16::MAX as u128, 128),
			ArbitraryInt::new(u16::MAX as u128, 16),
		),
		(
			ArbitraryInt::new(u32::MAX as u128, 128),
			ArbitraryInt::new(u32::MAX as u128, 32),
		),
		(
			ArbitraryInt::new(u64::MAX as u128, 128),
			ArbitraryInt::new(u64::MAX as u128, 64),
		),
		(
			ArbitraryInt::new(u128::MAX, 128),
			ArbitraryInt::new(u128::MAX, 128),
		),
	];

	for (int, expect) in expect {
		assert_eq!(int.resize(), expect)
	}
}
