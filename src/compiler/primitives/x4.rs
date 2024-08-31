
use std::ops::*;

use super::float::{bf16, f16};

macro_rules! impl_op_x4 {
	($name:tt, $tra:tt, $fun:ident, $op:tt) => {
		impl $tra for $name {
			type Output = Self;

			fn $fun(self, rhs: Self) -> Self::Output {
				Self(
					self.0 $op rhs.0,
					self.1 $op rhs.1,
					self.2 $op rhs.2,
					self.3 $op rhs.3
				)
			}
		}
	}
}

macro_rules! impl_ass_x4 {
	($name:tt, $tra:tt, $fun:ident, $op:tt) => {
		impl $tra for $name {
			fn $fun(&mut self, rhs: Self) {
				self.0 $op rhs.0;
				self.1 $op rhs.1;
				self.2 $op rhs.2;
				self.3 $op rhs.3;
			} 
		}
	}
}

macro_rules! impl_x4 {
	($name:tt, $typ:tt) => {
		#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
		pub struct $name(pub $typ, pub $typ, pub $typ, pub $typ);

		impl_op_x4!($name, Add, add, +);
		impl_op_x4!($name, Sub, sub, -);
		impl_op_x4!($name, Mul, mul, *);
		impl_op_x4!($name, Div, div, /);
		impl_op_x4!($name, Rem, rem, %);

		impl_ass_x4!($name, AddAssign, add_assign, +=);
		impl_ass_x4!($name, SubAssign, sub_assign, -=);
		impl_ass_x4!($name, MulAssign, mul_assign, *=);
		impl_ass_x4!($name, DivAssign, div_assign, /=);
		impl_ass_x4!($name, RemAssign, rem_assign, %=);
	}
}

impl_x4!(i8x4, i8);
impl_x4!(i16x4, i16);
impl_x4!(i32x4, i32);
impl_x4!(i64x4, i64);

impl_x4!(u8x4, u8);
impl_x4!(u16x4, u16);
impl_x4!(u32x4, u32);
impl_x4!(u64x4, u64);

impl_x4!(bf16x4, bf16);
impl_x4!(f16x4, f16);
impl_x4!(f32x4, f32);
impl_x4!(f64x4, f64);