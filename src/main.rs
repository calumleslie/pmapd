extern crate simd;

mod vector4;

use simd::f32x4;

fn main() {
	let x = f32x4::new(1.0, 2.0, 3.0, 4.0);

    println!("Hello, world! {:?}", x*x);
}
