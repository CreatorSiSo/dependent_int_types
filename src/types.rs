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

	pub fn from_data(data: u128) -> Self {
		Self { data, length: 0 }.resize()
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

impl std::ops::Add for ArbitraryInt {
	type Output = ArbitraryInt;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.data += rhs.data;
		self.resize()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	type Aint = ArbitraryInt;

	#[test]
	fn aint_from_data() {
		let expect = [
			(Aint::from_data(1), Aint::new(1, 1)),
			(Aint::from_data(3), Aint::new(3, 2)),
			(Aint::from_data(7), Aint::new(7, 3)),
			(Aint::from_data(15), Aint::new(15, 4)),
			(Aint::from_data(31), Aint::new(31, 5)),
			(Aint::from_data(63), Aint::new(63, 6)),
			(Aint::from_data(127), Aint::new(127, 7)),
			(Aint::from_data(255), Aint::new(255, 8)),
			(Aint::from_data(511), Aint::new(511, 9)),
			(Aint::from_data(1023), Aint::new(1023, 10)),
			(Aint::from_data(2047), Aint::new(2047, 11)),
			(Aint::from_data(4095), Aint::new(4095, 12)),
			(
				Aint::from_data(u16::MAX as u128),
				Aint::new(u16::MAX as u128, 16),
			),
			(
				Aint::from_data(u32::MAX as u128),
				Aint::new(u32::MAX as u128, 32),
			),
			(
				Aint::from_data(u64::MAX as u128),
				Aint::new(u64::MAX as u128, 64),
			),
			(Aint::from_data(u128::MAX), Aint::new(u128::MAX, 128)),
		];

		for (int, expect) in expect {
			assert_eq!(int, expect)
		}
	}

	#[test]
	fn resize_aint() {
		let expect = [
			(Aint::new(1, 128), Aint::new(1, 1)),
			(Aint::new(3, 128), Aint::new(3, 2)),
			(Aint::new(7, 128), Aint::new(7, 3)),
			(Aint::new(15, 128), Aint::new(15, 4)),
			(Aint::new(31, 128), Aint::new(31, 5)),
			(Aint::new(63, 128), Aint::new(63, 6)),
			(Aint::new(127, 128), Aint::new(127, 7)),
			(Aint::new(255, 128), Aint::new(255, 8)),
			(Aint::new(511, 128), Aint::new(511, 9)),
			(Aint::new(1023, 128), Aint::new(1023, 10)),
			(Aint::new(2047, 128), Aint::new(2047, 11)),
			(Aint::new(4095, 128), Aint::new(4095, 12)),
			(
				Aint::new(u16::MAX as u128, 128),
				Aint::new(u16::MAX as u128, 16),
			),
			(
				Aint::new(u32::MAX as u128, 128),
				Aint::new(u32::MAX as u128, 32),
			),
			(
				Aint::new(u64::MAX as u128, 128),
				Aint::new(u64::MAX as u128, 64),
			),
			(Aint::new(u128::MAX, 128), Aint::new(u128::MAX, 128)),
		];

		for (int, expect) in expect {
			assert_eq!(int.resize(), expect)
		}
	}

	#[test]
	fn aint_add() {
		let expect = [
			(Aint::from_data(1) + Aint::from_data(1), Aint::new(2, 2)),
			(
				Aint::from_data(123) + Aint::from_data(132),
				Aint::new(255, 8),
			),
		];

		for (int, expect) in expect {
			assert_eq!(int, expect)
		}
	}
}
