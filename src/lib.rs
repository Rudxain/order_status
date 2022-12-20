#![no_std]
#![warn(
	unused,
	future_incompatible,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string
)]
#![deny(clippy::unwrap_used)]
#![forbid(
	unsafe_code,
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::excessive_precision,
	clippy::float_arithmetic,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

use core::cmp::Ordering;

/// Returns:
/// - `Some(Ordering::Equal)` if `a.len() < 2` or all values are the same
/// - `Some(Ordering::Greater)` if ascending
/// - `Some(Ordering::Less)` if descending
/// - `None` if unordered/unsorted
pub fn get_order<T: core::cmp::PartialOrd>(a: &[T]) -> Option<Ordering> {
	// to-do: optimize 3n to n iters
	if a.windows(2).all(|w| w[0] == w[1]) {
		return Some(Ordering::Equal);
	}
	if a.windows(2).all(|w| w[0] <= w[1]) {
		return Some(Ordering::Greater);
	}
	if a.windows(2).all(|w| w[0] >= w[1]) {
		return Some(Ordering::Less);
	}
	None
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
	use super::*;

	#[test]
	fn eq_works() {
		assert_eq!(get_order::<u8>(&[]).unwrap(), Ordering::Equal);
		assert_eq!(get_order(&[0]).unwrap(), Ordering::Equal);
		assert_eq!(get_order(&[69; 69]).unwrap(), Ordering::Equal);
	}
	#[test]
	fn asc_works() {
		assert_eq!(get_order(&[-2, -1, 0, 1, 2]).unwrap(), Ordering::Greater);
	}
	#[test]
	fn des_works() {
		assert_eq!(get_order(&[2, 1, 0, -1, -2]).unwrap(), Ordering::Less);
	}
	#[test]
	fn none_works() {
		assert_eq!(get_order(&[0, 1, 0]), None);
		assert_eq!(get_order(&[-6, 1, -9, 42]), None);
	}
}
