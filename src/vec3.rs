use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

macro_rules! impl_binop {
	($Trait:ident $fn:ident) => {
		impl $Trait for Vec3 {
			type Output = Vec3;

			fn $fn(self, rhs: Vec3) -> Self::Output {
				Vec3 {
					x: self.x.$fn(rhs.x),
					y: self.y.$fn(rhs.y),
					z: self.z.$fn(rhs.z),
				}
			}
		}

		impl $Trait<Vec3> for &Vec3 {
			type Output = Vec3;

			fn $fn(self, rhs: Vec3) -> Self::Output {
				Vec3 {
					x: self.x.$fn(rhs.x),
					y: self.y.$fn(rhs.y),
					z: self.z.$fn(rhs.z),
				}
			}
		}

		impl $Trait<&Vec3> for Vec3 {
			type Output = Vec3;

			fn $fn(self, rhs: &Vec3) -> Self::Output {
				Vec3 {
					x: self.x.$fn(rhs.x),
					y: self.y.$fn(rhs.y),
					z: self.z.$fn(rhs.z),
				}
			}
		}

		impl $Trait<&Vec3> for &Vec3 {
			type Output = Vec3;

			fn $fn(self, rhs: &Vec3) -> Self::Output {
				Vec3 {
					x: self.x.$fn(rhs.x),
					y: self.y.$fn(rhs.y),
					z: self.z.$fn(rhs.z),
				}
			}
		}
	};
}

impl_binop!(Add add);
impl_binop!(Sub sub);

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl Mul<f64> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl Mul<&f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: &f64) -> Self::Output {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl Mul<&f64> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: &f64) -> Self::Output {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}

impl From<(f64, f64, f64)> for Vec3 {
	fn from((x, y, z): (f64, f64, f64)) -> Self {
		Vec3 { x, y, z }
	}
}

impl From<[f64; 3]> for Vec3 {
	fn from([x, y, z]: [f64; 3]) -> Self {
		Vec3 { x, y, z }
	}
}
