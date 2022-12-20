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

use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Ordering {
	/// unknown
	Undefined,
	/// (`Ascending` && `Descending`) || same || uniform || homogeneous
	Equal,
	/// previous <= next
	Ascending,
	/// previous >= next
	Descending,
	/// unordered || unsorted
	None,
}
impl fmt::Display for Ordering {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Undefined => write!(f, "unknown ordering"),
			Self::Equal => write!(f, "equal"),
			Self::Ascending => write!(f, "ascending"),
			Self::Descending => write!(f, "descending"),
			Self::None => write!(f, "unordered")
		}
	}
}

pub fn get_order<T: core::cmp::PartialOrd>(a: &[T]) -> Ordering {
	if a.len() < 2 {
		return Ordering::Undefined;
	}
	if a.windows(2).all(|w| w[0] == w[1]) {
		return Ordering::Equal;
	}
	if a.windows(2).all(|w| w[0] <= w[1]) {
		return Ordering::Ascending;
	}
	if a.windows(2).all(|w| w[0] >= w[1]) {
		return Ordering::Descending;
	}
	Ordering::None
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
	use super::*;

	#[test]
	fn undef_works() {
		assert_eq!(get_order::<u8>(&[]), Ordering::Undefined);
		assert_eq!(get_order(&[0]), Ordering::Undefined);
	}

	#[test]
	fn eq_works() {
		assert_eq!(get_order(&[69; 69]), Ordering::Equal);
	}
	#[test]
	fn asc_works() {
		assert_eq!(get_order(&[-2, -1, 0, 1, 2]), Ordering::Ascending);
	}
	#[test]
	fn des_works() {
		assert_eq!(get_order(&[2, 1, 0, -1, -2]), Ordering::Descending);
	}
	#[test]
	fn none_works() {
		assert_eq!(get_order(&[0, 1, 0]), Ordering::None);
		assert_eq!(get_order(&[-6, 1, -9, 42]), Ordering::None);
	}
}
