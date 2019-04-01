use typenum::{U4};
use philox::{GenericArray, Philox4x32_10};

struct Counter(GenericArray<u32, U4>);

impl Counter {
  fn inc(&mut self) {
    let ctr = &mut self.0;
    let x = ctr[0].overflowing_add(1);
    ctr[0] = x.0;
    if x.1 {
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

/*
Record some intermediate values from the stream to cross check after refactoring.
*/
#[test] fn record_values() {
  let record_every = 0x8ff;
  use std::io::Write;
  let mut f1 = std::fs::File::create(format!("2-6-{:x}-01", record_every)).unwrap();
  let mut ph = Philox4x32_10::default();
  let key = GenericArray::from_slice(&[2, 6]);
  let mut ctr = Counter(GenericArray::default());
  let now = std::time::Instant::now;
  let t1 = now();
  loop {
    let r = ph.next(key.clone(), ctr.0.clone());
    ctr.inc();
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
  let dt = now() - t1;
  let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
  let nn = ((ctr.0[1] as u64) << 32) + ctr.0[0] as u64;
  eprintln!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0);
}

#[test] fn speed() {
  let mut ph = Philox4x32_10::default();
  let key = GenericArray::from_slice(&[2, 6]);
  let mut ctr = Counter(GenericArray::default());
  let now = std::time::Instant::now;
  let t1 = now();
  loop {
    ph.next(key.clone(), ctr.0.clone());
    ctr.inc();
    if ctr.0[0] & 0x7fff == 0 {
      if now() - t1 > std::time::Duration::from_millis(1500) {
        break;
      }
    }
  }
  let dt = now() - t1;
  let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
  let nn = ((ctr.0[1] as u64) << 32) + ctr.0[0] as u64;
  eprintln!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0);
}
