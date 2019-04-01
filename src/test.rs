use typenum::{Unsigned, U2, U4, U8, U10, U32, U64};
use generic_array::GenericArray;
use super::Ph;

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
  for v in &vectors {
    let v = parse_test_vector(v);
    let mut ph = Ph::<U4, U32, U10>::from_key(v.0).set_ctr(v.1);
    let r = ph.next();
    assert_eq!(r.as_slice(), v.2.as_slice());
  }
}
