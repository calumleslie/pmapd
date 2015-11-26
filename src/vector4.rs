use simd::f32x4;
use std::cmp::Eq;
use std::ops::Sub;

#[derive(Debug,Clone,Copy)]
struct Vector4 {
	value: f32x4
}

impl PartialEq for Vector4 {
	fn eq(&self, other: &Vector4) -> bool {
		self.value.eq( other.value ).all()
	}
}

// Equality is total
impl Eq for Vector4 {}

impl Vector4 {
	fn new(x: f32, y: f32, z: f32, t: f32) -> Vector4 {
		Vector4::wrapping( f32x4::new( x, y, z,t ) )
	}
	fn wrapping( value: f32x4 ) -> Vector4 {
		Vector4 { value: value }
	}
	fn x(self) -> f32 {
		self.value.extract(0)
	}
	fn y(self) -> f32 {
		self.value.extract(1)
	}
	fn z(self) -> f32 {
		self.value.extract(2)
	}
	fn t(self) -> f32 {
		self.value.extract(3)
	}
	fn min(self, other: Vector4) -> Vector4 {
		Vector4 { value: self.value.min( other.value ) }
	}
	fn max(self, other: Vector4) -> Vector4 {
		Vector4 { value: self.value.max( other.value ) }
	}
	fn dot(self, other: Vector4) -> f32 {
		// Paul's implementation uses an intrinsic for this but 
		// the simd crate doesn't seem to support it.
		let multiplied = self.value * other.value;

		multiplied.extract(0) + 
			multiplied.extract(1) + 
			multiplied.extract(2) + 
			multiplied.extract(3)
	}
	fn magnitude_squared(self) -> f32 {
		self.dot(self)
	}
	fn distance_squared(self, other: Vector4) -> f32 {
		(self - other).magnitude_squared()
	}
	fn distance_squared_to_bounding_box(self, mins: Vector4, maxs: Vector4) -> f32 {
		assert!(Vector4::min(mins, maxs) == mins);
		assert!(Vector4::max(mins, maxs) == maxs);
		
		self.distance_squared(Vector4::min(maxs, Vector4::max(mins, self)))
	}
}

impl Sub for Vector4 {
	type Output = Self;
	fn sub(self, x: Self) -> Self {
		Vector4::wrapping( self.value - x.value )
	}
}

#[test]
fn accessors_work() {
	let v4 = Vector4::new( 1.0, 2.0, 3.0, 4.0 );
	assert_eq!( v4.x(), 1.0 );
	assert_eq!( v4.y(), 2.0 );
	assert_eq!( v4.z(), 3.0 );
	assert_eq!( v4.t(), 4.0 );
}

#[test]
fn magnitude_squared_test() {
	let v4 = Vector4::new( 2.0, 2.0, 2.0, 2.0 );

	assert_eq!(v4.magnitude_squared(), 16.0);
}

#[test]
fn distance_squared_test() {
	let src = Vector4::new( 1.0, 1.0, 4.0, 1.0 );
	let dst = Vector4::new( 1.0, 1.0, 9.0, 1.0 );

	assert_eq!(src.distance_squared(dst), 25.0 );
}

#[test]
fn distance_squared_to_bounding_box_test() {
	let mins = Vector4::new( 0.0, 0.0, 0.0, 0.0 );
	let maxs = Vector4::new( 8.0, 8.0, 8.0, 8.0 );
	let point = Vector4::new( 4.0, 4.0, 4.0, 11.0 );

	assert_eq!(point.distance_squared_to_bounding_box(mins, maxs), 9.0);

}