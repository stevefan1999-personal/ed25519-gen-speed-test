use atomic_counter::{AtomicCounter, RelaxedCounter};
use ed25519_dalek::{SecretKey, SigningKey};
use quanta::Clock;
use std::{mem::MaybeUninit, sync::Arc, thread::sleep, time::Duration};
use thread_priority::ThreadPriority;
use fastrand::Rng;

fn main() {
    let counter = Arc::new(RelaxedCounter::new(0));
    let clock = Clock::new().now();
    for _ in 0..std::thread::available_parallelism().unwrap().get() {
        thread_priority::spawn_careless(ThreadPriority::Max, {
            let counter = counter.clone();
            move || {
                let mut rng = Rng::new();
                let mut bytes: SecretKey = unsafe { MaybeUninit::uninit().assume_init() };
                loop {
                    rng.shuffle(&mut bytes);
                    let _key = SigningKey::from_bytes(&bytes);
                    counter.inc();
                }
            }
        });
    }

    loop {
        let counter = counter.get();
        println!(
            "{} keys per second on average",
            counter / (clock.elapsed().as_secs().max(1) as usize)
        );

        sleep(Duration::from_secs(1));
    }
}
