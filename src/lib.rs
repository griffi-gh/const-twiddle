//! # const-twiddle
//! Designed to be similar to `twiddle`, but also works in `const` contexts.\
//! Created for use in my gba emulator\
//! Supports `#![no_std]` by default

#![no_std]

use core::ops::RangeInclusive;

/// A trait for bit-twiddling utility functions.
pub trait Twiddle<T> {
	/// Creates a bitmask from a range of bits.
	#[must_use] 
	fn mask(range: RangeInclusive<u32>) -> T;

	/// Returns a set of bits.
	#[must_use] 
	fn bits(self, range: RangeInclusive<u32>) -> T;

	/// Returns a given bit as a boolean.
	#[must_use] 
	fn bit(self, bit: u32) -> bool;
	
	/// Replace a set of bits
	#[must_use] 
	fn set_bits(self, range: RangeInclusive<u32>, value: T) -> T;

	/// Set a single bit
	#[must_use] 
	fn set_bit(self, bit: u32, value: bool) -> T;
}

macro_rules! impl_twiddle {
	($tt: tt) => {
		::paste::paste! {
			#[inline]
			const fn [<$tt _unwrap_or>](option: Option<$tt>, or: $tt) -> $tt {
				match option {
					Some(x) => x,
					None => or
				}
			}
	
			#[inline]
			const fn [<$tt _cshl>](x: $tt, n: u32) -> $tt {
				[<$tt _unwrap_or>](x.checked_shl(n), 0)
			}
	
			#[inline]
			const fn [<$tt _mask>](start: u32, end: u32) -> $tt {
				let top = [<$tt _cshl>](1, 1 + start - end);
				top.wrapping_sub(1) << end
			}
	
			#[inline]
			const fn [<$tt _bits>](x: $tt, start: u32, end: u32) -> $tt {
				(x & [<$tt _mask>](start, end)) >> end
			}

			#[inline]
			const fn [<$tt _bit>](x: $tt, bit: u32) -> bool {
				((x >> bit) & 1) != 0
			}

			#[inline]
			const fn [<$tt _set_bits>](x: $tt, start: u32, end: u32, bits: $tt) -> $tt {
				let mask = [<$tt _mask>](start, end);
        (x & !mask) | ((bits << end) & mask)
			}

			#[inline]
			const fn [<$tt _set_bit>](x: $tt, bit: u32, value: bool) -> $tt {
				let mask = [<$tt _mask>](bit, bit);
				(x & !mask) | ((value as $tt) << bit)
			}
			
			/// Allows usage of [`Twiddle`] functions in const contexts 
			#[repr(transparent)]
			#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
			pub struct [<Const $tt:upper>](pub $tt);

			impl From<$tt> for [<Const $tt:upper>] {
				#[inline]
				#[must_use]
				fn from(x: $tt) -> Self {
					Self(x)
				}
			}

			impl From<[<Const $tt:upper>]> for $tt {
				#[inline]
				#[must_use]
				fn from(x: [<Const $tt:upper>]) -> Self {
					x.0
				}
			}

			impl [<Const $tt:upper>] {
				/// Creates a bitmask from a range of bits.
				#[inline]
				#[must_use]
				pub const fn mask(range: RangeInclusive<u32>) -> $tt {
					[<$tt _mask>](*range.start(), *range.end())
				}

				/// Returns a set of bits.
				#[inline]
				#[must_use]
				pub const fn bits(self, range: RangeInclusive<u32>) -> $tt {
					[<$tt _bits>](self.0, *range.start(), *range.end())
				}
				
				/// Returns a given bit as a boolean.
				#[inline]
				#[must_use]
				pub const fn bit(self, bit: u32) -> bool {
					[<$tt _bit>](self.0, bit)
				}

				/// Replace a set of bits
				#[inline]
				#[must_use] 
				pub const fn set_bits(self, range: RangeInclusive<u32>, value: $tt) -> $tt {
					[<$tt _set_bits>](self.0, *range.start(), *range.end(), value)
				}

				/// Set a single bit
				#[inline]
				#[must_use] 
				pub const fn set_bit(self, bit: u32, value: bool) -> $tt {
					[<$tt _set_bit>](self.0, bit, value)
				}
			}

      impl Twiddle<$tt> for [<Const $tt:upper>] {
				#[inline]
				#[must_use]
				fn mask(range: RangeInclusive<u32>) -> $tt {
					Self::mask(range)
				}

				#[inline]
				#[must_use]
				fn bits(self, range: RangeInclusive<u32>) -> $tt {
					self.bits(range)
				}
				
				#[inline]
				#[must_use]
				fn bit(self, bit: u32) -> bool {
					self.bit(bit)
				}

				#[inline]
				#[must_use] 
				fn set_bits(self, range: RangeInclusive<u32>, value: $tt) -> $tt {
					self.set_bits(range, value)
				}

				#[inline]
				#[must_use] 
				fn set_bit(self, bit: u32, value: bool) -> $tt {
					self.set_bit(bit, value)
				}
			}

			impl Twiddle<$tt> for $tt {
				#[inline]
				#[must_use]
				fn mask(range: RangeInclusive<u32>) -> $tt {
					[<$tt _mask>](*range.start(), *range.end())
				}

				#[inline]
				#[must_use]
				fn bits(self, range: RangeInclusive<u32>) -> $tt {
					[<$tt _bits>](self, *range.start(), *range.end())
				}

				#[inline]
				#[must_use]
				fn bit(self, bit: u32) -> bool {
					[<$tt _bit>](self, bit)
				}

				#[inline]
				#[must_use] 
				fn set_bits(self, range: RangeInclusive<u32>, value: $tt) -> $tt {
					[<$tt _set_bits>](self, *range.start(), *range.end(), value)
				}

				#[inline]
				#[must_use] 
				fn set_bit(self, bit: u32, value: bool) -> $tt {
					[<$tt _set_bit>](self, bit, value)
				}
			}
		}
	};
	( $( $tt: tt ),* ) => {
		$(
			impl_twiddle!($tt);
		)*
	}
}

impl_twiddle! {
	u8, u16, u32, u64, u128, 
	i8, i16, i32, i64, i128
}
