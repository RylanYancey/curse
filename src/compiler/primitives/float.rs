use num_traits::{Bounded, FromPrimitive, MulAdd, Num, One, ParseFloatError, Zero};
use std::ops::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct f16(pub half::f16);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct bf16(pub half::bf16);

pub trait Float {
	
}

impl Float for half::f16 {

}

impl Float for half::bf16 {

}

impl Float for f32 {

}

impl Float for f64 {

}

macro_rules! impl_op {
	($typ:tt, $tra:tt, $fun:ident, $tra_ass:tt, $fun_ass:ident) => {
		impl $tra for $typ {
			type Output = Self;

			fn $fun(self, rhs: Self::Output) -> Self {
				$typ(half::$typ::$fun(self.0, rhs.0))
			}
		}

		impl $tra_ass for $typ {
			fn $fun_ass(&mut self, rhs: Self) {
				half::$typ::$fun_ass(&mut self.0, rhs.0);
			}
		}
	}
}

impl_op!(bf16, Add, add, AddAssign, add_assign);
impl_op!(bf16, Sub, sub, SubAssign, sub_assign);
impl_op!(bf16, Mul, mul, MulAssign, mul_assign);
impl_op!(bf16, Div, div, DivAssign, div_assign);
impl_op!(bf16, Rem, rem, RemAssign, rem_assign);

impl Zero for bf16 {
	fn is_zero(&self) -> bool {
		self.0 == half::bf16::ZERO
	}

	fn zero() -> Self {
		bf16(half::bf16::ZERO)
	}

	fn set_zero(&mut self) {
		self.0 = half::bf16::ZERO
	}
}

impl One for bf16 {
	fn is_one(&self) -> bool
	where
		Self: PartialEq, 
	{
		self.0 == half::bf16::ONE	
	}

	fn one() -> Self {
		bf16(half::bf16::ONE)
	}

	fn set_one(&mut self) {
		self.0 = half::bf16::ONE
	}
}

impl Bounded for bf16 {
	fn max_value() -> Self {
		bf16(half::bf16::MAX)
	}

	fn min_value() -> Self {
		bf16(half::bf16::MIN)
	}
}

impl MulAdd for bf16 {
	type Output = Self;

	fn mul_add(self, a: Self, b: Self) -> Self::Output {
		bf16(half::bf16::add(half::bf16::mul(self.0, a.0), b.0))
	}
}

impl FromPrimitive for bf16 {
	fn from_i64(n: i64) -> Option<Self> {
		half::bf16::from_i64(n).map(|f| bf16(f))
	}

	fn from_u64(n: u64) -> Option<Self> {
		half::bf16::from_u64(n).map(|f| bf16(f))
	}

	fn from_f32(n: f32) -> Option<Self> {
		Some(bf16(half::bf16::from_f32(n)))
	}

	fn from_f64(n: f64) -> Option<Self> {
		Some(bf16(half::bf16::from_f64(n)))
	}
}

impl Num for bf16 {
	type FromStrRadixErr = ParseFloatError;

	fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
		f32::from_str_radix(str, radix)
			.map(|f| bf16(half::bf16::from_f32(f)))
	}
}

impl_op!(f16, Add, add, AddAssign, add_assign);
impl_op!(f16, Sub, sub, SubAssign, sub_assign);
impl_op!(f16, Mul, mul, MulAssign, mul_assign);
impl_op!(f16, Div, div, DivAssign, div_assign);
impl_op!(f16, Rem, rem, RemAssign, rem_assign);

impl Zero for f16 {
	fn is_zero(&self) -> bool {
		self.0 == half::f16::ZERO
	}

	fn zero() -> Self {
		f16(half::f16::ZERO)
	}

	fn set_zero(&mut self) {
		self.0 = half::f16::ZERO
	}
}

impl One for f16 {
	fn is_one(&self) -> bool
		where
			Self: PartialEq, 
	{
		self.0 == half::f16::ONE	
	}

	fn one() -> Self {
		f16(half::f16::ONE)
	}

	fn set_one(&mut self) {
		self.0 = half::f16::ONE
	}
}

impl Bounded for f16 {
	fn max_value() -> Self {
		f16(half::f16::MAX)
	}

	fn min_value() -> Self {
		f16(half::f16::MIN)
	}
}

impl MulAdd for f16 {
	type Output = Self;

	fn mul_add(self, a: Self, b: Self) -> Self::Output {
		f16(half::f16::add(half::f16::mul(self.0, a.0), b.0))
	}
}

impl FromPrimitive for f16 {
	fn from_i64(n: i64) -> Option<Self> {
		half::f16::from_i64(n).map(|f| f16(f))
	}

	fn from_u64(n: u64) -> Option<Self> {
		half::f16::from_u64(n).map(|f| f16(f))
	}

	fn from_f32(n: f32) -> Option<Self> {
		Some(f16(half::f16::from_f32(n)))
	}

	fn from_f64(n: f64) -> Option<Self> {
		Some(f16(half::f16::from_f64(n)))
	}
}

impl Num for f16 {
	type FromStrRadixErr = ParseFloatError;

	fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
		f32::from_str_radix(str, radix)
			.map(|f| f16(half::f16::from_f32(f)))
	}
}