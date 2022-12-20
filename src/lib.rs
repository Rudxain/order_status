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
#![forbid(
	unsafe_code,
	clippy::unwrap_used,
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
pub enum Order {
	/// `Ascending && Descending`
	Equal,
	Ascending,
	Descending,
	/// unsorted or unordered
	None,
}
impl fmt::Display for Order {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Equal => write!(f, "Equal"),
			Self::Ascending => write!(f, "Ascending"),
			Self::Descending => write!(f, "Descending"),
			Self::None => write!(f, "None")
		}
	}
}

/// returns `Order::Equal` if `a.len() < 2`
pub fn get_order<T: core::cmp::PartialOrd>(a: &[T]) -> Order {
	// to-do: optimize 3n to n iters
	if a.windows(2).all(|w| w[0] == w[1]) {
		return Order::Equal;
	}
	if a.windows(2).all(|w| w[0] <= w[1]) {
		return Order::Ascending;
	}
	if a.windows(2).all(|w| w[0] >= w[1]) {
		return Order::Descending;
	}
	Order::None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eq_works() {
		assert_eq!(get_order::<u8>(&[]), Order::Equal);
		assert_eq!(get_order(&[0]), Order::Equal);
		assert_eq!(get_order(&[69; 69]), Order::Equal);
	}
	#[test]
	fn asc_works() {
		assert_eq!(get_order(&[-2, -1, 0, 1, 2]), Order::Ascending);
	}
	#[test]
	fn des_works() {
		assert_eq!(get_order(&[2, 1, 0, -1, -2]), Order::Descending);
	}
	#[test]
	fn none_works() {
		assert_eq!(get_order(&[0, 1, 0]), Order::None);
		assert_eq!(get_order(&[-6, 1, -9, 42]), Order::None);
	}
}
