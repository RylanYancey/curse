use super::float::bf16;
use super::float::f16;


pub trait Clamp {
	fn clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamp {
	($t:tt) => {
		impl Clamp for $t {
			fn clamp(self, min: Self, max: Self) -> Self {
				if self < min {
					min
				} else if self > max {
					max
				} else {
					self
				}
			}
		}
	}
}

impl_clamp!(i8);
impl_clamp!(i16);
impl_clamp!(i32);
impl_clamp!(i64);

impl_clamp!(u8);
impl_clamp!(u16);
impl_clamp!(u32);
impl_clamp!(u64);

impl_clamp!(bf16);
impl_clamp!(f16);
impl_clamp!(f32);
impl_clamp!(f64);