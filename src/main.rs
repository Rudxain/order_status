#![warn(
	unused,
	future_incompatible,
	clippy::cargo,
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
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

#[allow(clippy::enum_glob_use)]
use core::cmp::Ordering::*;
use order_status::get_order;

fn main() {
	println!(
		"{}",
		match get_order(
			&std::env::args()
				.skip(1)
				.map(|s| s.parse::<f64>().unwrap())
				.collect::<Vec<f64>>()
		) {
			Some(Equal) => "equal",
			Some(Greater) => "ascending",
			Some(Less) => "descending",
			None => "unordered",
		}
	);
}
