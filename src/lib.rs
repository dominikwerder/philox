#[allow(unused)]
use typenum::{Unsigned, U2, U4, U8, U10, U32, U64, Same, UInt};
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Mul, Div};

type Ph<N, W, R> = Philox<N, W, R, <N as Div<U2>>::Output>;

pub struct Philox<N: Unsigned + ArrayLength<u32>, W: Unsigned, R: Unsigned, KN: ArrayLength<u32>> {
	_m0: std::marker::PhantomData<N>,
	_m1: std::marker::PhantomData<KN>,
	_m2: std::marker::PhantomData<W>,
	_m3: std::marker::PhantomData<R>,
}

struct HiLo(u32, u32);

fn mulhilo(a: u32, b: u32) -> HiLo {
	let p = a as u64 * b as u64;
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
	fn round(&mut self, key: &mut GenericArray<u32, KN>, mut ctr: &mut GenericArray<u32, N>) {
		let c0 = ctr.clone();
		// These constants were chosen just because the random numbers look statistically best
		#[allow(non_upper_case_globals)] const PHILOX_M4x32_0: u32 = 0xD2511F53;
		#[allow(non_upper_case_globals)] const PHILOX_M4x32_1: u32 = 0xCD9E8D57;
		let HiLo(hi0, lo0) = mulhilo(PHILOX_M4x32_0, c0[0]);
		let HiLo(hi1, lo1) = mulhilo(PHILOX_M4x32_1, c0[2]);
		let c1 = &mut ctr;
		c1[0] = hi1 ^ c0[1] ^ key[0];
		c1[1] = lo1;
		c1[2] = hi0 ^ c0[3] ^ key[1];
		c1[3] = lo0;
	}
	fn update_key(&mut self, key: &mut GenericArray<u32, KN>) {
		const C0: u32 = 0x9E3779B9;
		const C1: u32 = 0xBB67AE85;
		key[0] = key[0].overflowing_add(C0).0; // golden ratio
		key[1] = key[1].overflowing_add(C1).0; // sqrt(3) - 1
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

#[test] fn a() {
	assert_eq!(U64::to_u32(), 64);
	assert_eq!(<U64 as Div<U8>>::Output::to_u32(), 8);
	assert_eq!(<U64 as Mul<U2>>::Output::to_u32(), 128);
	assert_eq!(U64::U32, 64);
}

#[test] fn arithmetic() {
	assert_eq!((0xffffffffffffffff_u64 >> 32) as u32, 0xffffffff_u32);
	assert_eq!(0x12345678cafe1337_u64 as u32, 0xcafe1337_u32);
}

fn parse_test_vector(s: &str) -> (GenericArray<u32, U2>, GenericArray<u32, U4>, GenericArray<u32, U4>) {
	let a: Vec<_> = s.split(" ").map(|x| x.trim()).collect();
	eprintln!("{:?}", a);
	type T = u32;
	fn p(s: &str) -> T {
		T::from_str_radix(s, 16).unwrap()
	}
	(
		[p(a[6]), p(a[7])].into(),
		[p(a[2]), p(a[3]), p(a[4]), p(a[5])].into(),
		[p(a[8]), p(a[9]), p(a[10]), p(a[11])].into(),
	)
}

#[test] fn test_parse_test_vector() {
	/*
	Test vectors contain first the counter, then the key, then the result.
	*/
	let s = "philox4x32 10 243f6a88 85a308d3 13198a2e 03707344 a4093822 299f31d0 d16cfe09 94fdcceb 5001e420 24126ea1";
	let v = parse_test_vector(s);
	assert_eq!(v.0.as_slice(), &[0xa4093822, 0x299f31d0]);
}

#[test] fn generate_key_zeros() {
	let vectors = "
	philox4x32 10 00000000 00000000 00000000 00000000 00000000 00000000 6627e8d5 e169c58d bc57ac4c 9b00dbd8
	philox4x32 10 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff 408f276d 41c83b0e a20bc7c6 6d5451fd
	philox4x32 10 243f6a88 85a308d3 13198a2e 03707344 a4093822 299f31d0 d16cfe09 94fdcceb 5001e420 24126ea1
	".trim().split("\n").map(|x|x.trim()).collect::<Vec<_>>();
	for v in &vectors[..] {
		let v = parse_test_vector(v);
		let mut ph = Ph::<U4, U32, U10>::default();
		let r = ph.next(v.0, v.1);
		assert_eq!(r.as_slice(), v.2.as_slice());
	}
}
