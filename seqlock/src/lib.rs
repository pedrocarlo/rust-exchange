//! Got from this very informative blog
//! https://louisponet.github.io/blog/posts/icc-1-seqlock/

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering, compiler_fence},
};

#[derive(Default)]
#[repr(align(64))]
pub struct Seqlock<T> {
    version: AtomicUsize,
    data: UnsafeCell<T>,
}
unsafe impl<T: Send> Send for Seqlock<T> {}
unsafe impl<T: Sync> Sync for Seqlock<T> {}

impl<T: Copy> Seqlock<T> {
    pub fn new(data: T) -> Self {
        Self {
            version: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }
    #[inline(never)]
    pub fn read(&self, result: &mut T) {
        loop {
            let v1 = self.version.load(Ordering::Acquire);
            compiler_fence(Ordering::AcqRel);
            *result = unsafe { *self.data.get() };
            compiler_fence(Ordering::AcqRel);
            let v2 = self.version.load(Ordering::Acquire);
            if v1 == v2 && v1 & 1 == 0 {
                return;
            }
        }
    }

    #[inline(never)]
    pub fn write(&self, val: &T) {
        let v = self.version.fetch_add(1, Ordering::Release);
        compiler_fence(Ordering::AcqRel);
        unsafe { *self.data.get() = *val };
        compiler_fence(Ordering::AcqRel);
        self.version.store(v.wrapping_add(2), Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::atomic::AtomicBool,
        time::{Duration, Instant},
    };

    fn consumer_loop<const N: usize>(lock: &Seqlock<[usize; N]>, done: &AtomicBool) {
        let mut msg = [0usize; N];
        while !done.load(Ordering::Relaxed) {
            lock.read(&mut msg);
            let first = msg[0];
            for i in msg {
                if first != i {
                    dbg!(&first, &msg);
                }
                assert_eq!(first, i);
            }
        }
    }

    fn producer_loop<const N: usize>(lock: &Seqlock<[usize; N]>, done: &AtomicBool) {
        let curt = Instant::now();
        let mut count = 0;
        let mut msg = [0usize; N];
        while curt.elapsed() < Duration::from_secs(1) {
            msg.fill(count);
            lock.write(&msg);
            count = count.wrapping_add(1);
        }
        done.store(true, Ordering::Relaxed);
    }

    fn read_test<const N: usize>() {
        let lock = Seqlock::new([0usize; N]);
        let done = AtomicBool::new(false);
        crossbeam::thread::scope(|s| {
            s.spawn(|_| {
                consumer_loop(&lock, &done);
            });
            s.spawn(|_| {
                producer_loop(&lock, &done);
            });
        })
        .unwrap();
    }

    #[test]
    fn read_1() {
        read_test::<1>()
    }

    #[test]
    fn read_16() {
        read_test::<16>()
    }
    #[test]
    fn read_32() {
        read_test::<32>()
    }
    #[test]
    fn read_64() {
        read_test::<64>()
    }
    #[test]
    fn read_128() {
        read_test::<128>()
    }

    #[test]
    fn read_256() {
        read_test::<256>()
    }

    #[test]
    fn read_512() {
        read_test::<512>()
    }

    // #[test]
    // fn read_large() {
    //     read_test::<65536>()
    // }
}
