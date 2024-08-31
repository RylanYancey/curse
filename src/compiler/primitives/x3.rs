
use std::ops::*;

use super::float::{bf16, f16};

macro_rules! impl_op_x3 {
	($name:tt, $tra:tt, $fun:ident, $op:tt) => {
		impl $tra for $name {
			type Output = Self;

			fn $fun(self, rhs: Self) -> Self::Output {
				Self(
					self.0 $op rhs.0,
					self.1 $op rhs.1,
					self.2 $op rhs.2
				)
			}
		}
	}
}

macro_rules! impl_ass_x3 {
	($name:tt, $tra:tt, $fun:ident, $op:tt) => {
		impl $tra for $name {
			fn $fun(&mut self, rhs: Self) {
				self.0 $op rhs.0;
				self.1 $op rhs.1;
				self.2 $op rhs.2;
			} 
		}
	}
}

macro_rules! impl_x3 {
	($name:tt, $typ:tt) => {
		#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
		pub struct $name(pub $typ, pub $typ, pub $typ);

		impl_op_x3!($name, Add, add, +);
		impl_op_x3!($name, Sub, sub, -);
		impl_op_x3!($name, Mul, mul, *);
		impl_op_x3!($name, Div, div, /);
		impl_op_x3!($name, Rem, rem, %);

		impl_ass_x3!($name, AddAssign, add_assign, +=);
		impl_ass_x3!($name, SubAssign, sub_assign, -=);
		impl_ass_x3!($name, MulAssign, mul_assign, *=);
		impl_ass_x3!($name, DivAssign, div_assign, /=);
		impl_ass_x3!($name, RemAssign, rem_assign, %=);
	}
}

impl_x3!(i8x3, i8);
impl_x3!(i16x3, i16);
impl_x3!(i32x3, i32);
impl_x3!(i64x3, i64);

impl_x3!(u8x3, u8);
impl_x3!(u16x3, u16);
impl_x3!(u32x3, u32);
impl_x3!(u64x3, u64);

impl_x3!(bf16x3, bf16);
impl_x3!(f16x3, f16);
impl_x3!(f32x3, f32);
impl_x3!(f64x3, f64);