#[allow(unused)]
use typenum::{Unsigned, U2, U4, U8, U10, U32, U64, Same, UInt};
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Mul, Div};

type Ph<N, W, R> = Philox<N, W, R, <N as Div<U8>>::Output>;

pub struct Philox<N: Unsigned, W: Unsigned, R: Unsigned, NA: ArrayLength<u32>> {
	_key: GenericArray<u32, NA>,
	_m1: std::marker::PhantomData<N>,
	_m2: std::marker::PhantomData<W>,
	_m3: std::marker::PhantomData<R>,
}

impl<N: Unsigned, W: Unsigned, R: Unsigned, NA: ArrayLength<u32>> Default for Philox<N, W, R, NA> {
	fn default() -> Self {
		assert_eq!(W::to_usize(), 32);
		Self {
			_key: Default::default(),
			_m1: Default::default(),
			_m2: Default::default(),
			_m3: Default::default(),
		}
	}
}

#[test] fn a() {
	assert_eq!(U64::to_u32(), 64);
	assert_eq!(<U64 as Div<U8>>::Output::to_u32(), 8);
	assert_eq!(<U64 as Mul<U2>>::Output::to_u32(), 128);
	assert_eq!(U64::U32, 64);
}

#[test] fn b() {
	Ph::<U4, U32, U10>::default();
}
