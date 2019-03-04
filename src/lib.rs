/*
MIT License

Copyright (c) 2019 Dominik Werder <dominik.werder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use typenum::{Unsigned, U2, U4, U10, U32};
pub use generic_array::{ArrayLength, GenericArray};

type Ph<N, W, R> = Philox<N, W, R, <N as std::ops::Div<U2>>::Output>;
pub type Philox4x32_10 = Ph<U4, U32, U10>;

pub struct Philox<N: Unsigned + ArrayLength<u32>, W: Unsigned, R: Unsigned, KN: ArrayLength<u32>> {
	_m0: std::marker::PhantomData<N>,
	_m1: std::marker::PhantomData<KN>,
	_m2: std::marker::PhantomData<W>,
	_m3: std::marker::PhantomData<R>,
}

struct HiLo(u32, u32);

fn mulhilo(a: u32, b: u32) -> HiLo {
	let p = (a as u64).wrapping_mul(b as u64);
	HiLo((p >> 32) as u32, p as u32)
}

impl<N: Unsigned + ArrayLength<u32>, W: Unsigned, R: Unsigned, KN: ArrayLength<u32>> Philox<N, W, R, KN> {
	pub fn next(&mut self, mut key: GenericArray<u32, KN>, mut ctr: GenericArray<u32, N>) -> GenericArray<u32, N> {
		for _ in 0..R::USIZE {
			self.round(&mut key, &mut ctr);
			self.update_key(&mut key);
		}
		ctr
	}
	fn round(&mut self, key: &mut GenericArray<u32, KN>, ctr: &mut GenericArray<u32, N>) {
		// These constants were chosen just because the random numbers look statistically best
		#[allow(non_upper_case_globals)] const PHILOX_M4x32_0: u32 = 0xD2511F53;
		#[allow(non_upper_case_globals)] const PHILOX_M4x32_1: u32 = 0xCD9E8D57;
		let HiLo(hi0, lo0) = mulhilo(PHILOX_M4x32_0, ctr[0]);
		let HiLo(hi1, lo1) = mulhilo(PHILOX_M4x32_1, ctr[2]);
		ctr[0] = hi1 ^ ctr[1] ^ key[0];
		ctr[2] = hi0 ^ ctr[3] ^ key[1];
		ctr[1] = lo1;
		ctr[3] = lo0;
	}
	fn update_key(&mut self, key: &mut GenericArray<u32, KN>) {
		const C0: u32 = 0x9E3779B9;
		const C1: u32 = 0xBB67AE85;
		key[0] = key[0].wrapping_add(C0); // golden ratio
		key[1] = key[1].wrapping_add(C1); // sqrt(3) - 1
	}
}

impl<N: Unsigned + ArrayLength<u32>, W: Unsigned, R: Unsigned, KN: Unsigned + ArrayLength<u32>> Default for Philox<N, W, R, KN> {
	fn default() -> Self {
		assert_eq!(W::to_usize(), 32);
		Self {
			_m0: Default::default(),
			_m1: Default::default(),
			_m2: Default::default(),
			_m3: Default::default(),
		}
	}
}

#[cfg(test)] mod test;
