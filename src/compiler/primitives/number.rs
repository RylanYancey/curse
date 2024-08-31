
use std::ops::*;

use super::float::{bf16, f16};

pub trait Number :
	Sized + Copy + 
	Add + AddAssign +
	Sub + SubAssign +
	Mul + MulAssign +
	Div + DivAssign +
	Rem + RemAssign
{
	
}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}

impl Number for bf16 {}
impl Number for f16 {}
impl Number for f32 {}
impl Number for f64 {}