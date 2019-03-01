use typenum::{Unsigned, U2, U4, U8, U10, U32, U64};
use generic_array::GenericArray;
use super::{Ph, Philox4x32_10};

struct Counter(GenericArray<u32, U4>);

impl Counter {
	fn inc(&mut self) {
		let ctr = &mut self.0;
		ctr[0] = ctr[0].wrapping_add(1);
		if ctr[0] == 0 {
			let x = ctr[1].overflowing_add(1);
			ctr[1] = x.0;
			if x.1 {
				let x = ctr[2].overflowing_add(1);
				ctr[2] = x.0;
				if x.1 {
					let x = ctr[3].overflowing_add(1);
					ctr[3] = x.0;
				}
			}
		}
	}
}

#[test] fn typenum() {
	assert_eq!(U64::to_u32(), 64);
	assert_eq!(<U64 as std::ops::Div<U8>>::Output::to_u32(), 8);
	assert_eq!(<U64 as std::ops::Mul<U2>>::Output::to_u32(), 128);
	assert_eq!(U64::U32, 64);
}

#[test] fn arithmetic() {
	assert_eq!((0xffffffffffffffff_u64 >> 32) as u32, 0xffffffff_u32);
	assert_eq!(0x12345678cafe1337_u64 as u32, 0xcafe1337_u32);
}

fn parse_test_vector(s: &str) -> (GenericArray<u32, U2>, GenericArray<u32, U4>, GenericArray<u32, U4>) {
	let a: Vec<_> = s.split(" ").map(|x| x.trim()).collect();
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

#[test] fn check_test_vectors() {
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

#[test] fn speed() {
	let t1 = std::time::Instant::now();
	let mut ph = Philox4x32_10::default();
	let key = GenericArray::from_slice(&[2, 6]);
	let mut ctr = Counter(GenericArray::default());
	let nn = 10000000;
	for _ in 0..nn {
		ph.next(key.clone(), ctr.0.clone());
		ctr.inc();
	}
	let t2 = std::time::Instant::now();
	let dt = t2 - t1;
	let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
	//assert_eq!("", format!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0));
	assert!(secs < 2.0);
}

