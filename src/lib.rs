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

/// All possible sorting states of a sequence of values.
///
/// - `Undefined` should be used when no comparisons are possible
/// - `Equal` must be used if all values are considered "the same"
/// - `Ascending` and `Descending` are self-explanatory
/// - `Unsorted` should be used if the comparison fn couldn't recognize an ordering,
/// or when the sequence has an arbitrary or random order
#[derive(Debug, PartialEq, Eq)]
pub enum Ordering {
	/// unknown
	Undefined,
	/// (`Ascending` && `Descending`) || same || uniform || homogeneous
	Equal,
	/// increasing || previous <= next
	Ascending,
	/// decreasing || previous >= next
	Descending,
	/// unordered
	Unsorted, // avoid collision with `Option::None`
}
impl fmt::Display for Ordering {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Undefined => write!(f, "undefined"),
			Self::Equal => write!(f, "equal"),
			Self::Ascending => write!(f, "ascending"),
			Self::Descending => write!(f, "descending"),
			Self::Unsorted => write!(f, "unsorted"),
		}
	}
}

/// Uses `PartialOrd` trait to determine the ordering of a generic slice.
///
/// Returns `Ordering::Undefined` if `a.len() < 2`.
pub fn get_ordering<T: core::cmp::PartialOrd>(a: &[T]) -> Ordering {
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
	Ordering::Unsorted
}

/// Uses `PartialOrd` trait to determine the ordering of a generic iterator.
///
/// Returns `Ordering::Undefined` if `it.count() < 2`.
fn get_ordering_iter<T: core::cmp::PartialOrd>(
	it: &mut /* is there some way of avoiding `mut`? */ dyn Iterator<Item = T>,
) -> Ordering {
	let mut out = Ordering::Undefined;
	let mut previous: Option<T> = None;
	for current in it {
		if previous.is_none() {
			previous = Some(current);
			continue;
		}
		if previous == Some(current) {
			out = Ordering::Equal;
		}
	}
	out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
	use super::*;

	#[test]
	fn undef_works() {
		assert_eq!(get_ordering::<u8>(&[]), Ordering::Undefined);
		assert_eq!(get_ordering(&[0]), Ordering::Undefined);
	}

	#[test]
	fn eq_works() {
		assert_eq!(get_ordering(&[69; 69]), Ordering::Equal);
	}
	#[test]
	fn asc_works() {
		assert_eq!(get_ordering(&[-2, -1, 0, 1, 2]), Ordering::Ascending);
	}
	#[test]
	fn des_works() {
		assert_eq!(get_ordering(&[2, 1, 0, -1, -2]), Ordering::Descending);
	}
	#[test]
	fn none_works() {
		assert_eq!(get_ordering(&[0, 1, 0]), Ordering::Unsorted);
		assert_eq!(get_ordering(&[-6, 1, -9, 42]), Ordering::Unsorted);
	}
}
