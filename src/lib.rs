use typenum::{Unsigned, U2, U4, U8, U10, U32, U64};
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Mul, Div};

type TA = u8;

type Ph<N, W, R> = Philox<N, W, R, <N as Div<U8>>::Output>;

#[derive(Default)]
pub struct Philox<N, W, R, NA: ArrayLength<TA>> {
	_key: GenericArray<TA, NA>,
	_m1: std::marker::PhantomData<N>,
	_m2: std::marker::PhantomData<W>,
	_m3: std::marker::PhantomData<R>,
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
