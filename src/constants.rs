#[macro_export]
macro_rules! array_from_fn {
	($element_fn:expr; $lenght:expr) => {{
		let mut array = [Default::default(); $lenght];

		let mut i = 0;
		while i < $lenght {
			array[i] = $element_fn(i);
			i += 1;
		}

		array
	}};
}

// Size in bits of unsinged integers with lengths 1..127
// Size of u128 is u128::MAX + 1 so it does not fit into a u128
pub const fn uint_length_lut() -> [u128; 127] {
	const fn index_to_lenght(i: usize) -> u128 {
		u128::pow(2, i as u32 + 1)
	}
	array_from_fn![index_to_lenght; 127]
}

// Maximum value of unsinged integers with lengths 1..128
pub const fn uint_max_lut() -> [u128; 128] {
	const fn index_to_max(i: usize) -> u128 {
		match i {
			0..=126 => uint_length_lut()[i] - 1,
			_ => u128::MAX,
		}
	}
	array_from_fn![index_to_max; 128]
}
