use philox::Philox4x32_10;

/*
Record some intermediate values from the stream to cross check after refactoring.
*/
#[test] fn record_values() {
  let record_every = 0x8ff;
  use std::io::{Read, Write};
  let mut f1 = vec![];
  let mut ph = Philox4x32_10::from_key([2, 6].into()).set_ctr(Default::default());
  loop {
    let r = ph.next();
    let ctr = ph.ctr();
    if ctr.0[0] & record_every == 0 {
      // be safe
      assert_eq!(r.len(), 4);
      // note: adding a scope between ref and deref would cause copy
      let a = unsafe { &*(&r[0] as *const _ as *const [u8; 4 * 4]) };
      // make sure that we did not copy
      assert_eq!(&a[0] as *const _ as *const u8, &r[0] as *const _ as *const u8);
      f1.write(a).unwrap();
      if ctr.0[0] & 0xfffff == 0 {
        break;
      }
    }
  }
  let mut expect = vec![0u8; 32768];
  std::fs::File::open("2-6-8ff-01").unwrap().read_exact(&mut expect).unwrap();
  assert_eq!(&f1, &expect);
}

#[test] fn speed() {
  let mut ph = Philox4x32_10::from_key([2, 6].into()).set_ctr(Default::default());
  let now = std::time::Instant::now;
  let t1 = now();
  loop {
    ph.next();
    let ctr = ph.ctr();
    if ctr.0[0] & 0x7fff == 0 {
      if now() - t1 > std::time::Duration::from_millis(1500) {
        break;
      }
    }
  }
  let dt = now() - t1;
  let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
  let ctr = ph.ctr();
  let nn = ((ctr.0[1] as u64) << 32) + ctr.0[0] as u64;
  eprintln!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0);
}
