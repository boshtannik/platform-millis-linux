pub use platform_millis::{ms, PlatformTime};
use std::sync::Mutex;
use std::sync::Once;
use std::time::Instant;

lazy_static::lazy_static! {
  static ref START: Mutex<Option<Instant>> = Mutex::new(None);
  static ref INIT_ONCE: Once = Once::new();
}

pub struct LinuxTime;

impl PlatformTime for LinuxTime {
    fn millis() -> ms {
        INIT_ONCE.call_once(|| {
            let mut start = START.lock().unwrap();
            *start = Some(Instant::now());
        });

        let start = START.lock().unwrap();
        match *start {
            Some(instant) => {
                let elapsed = Instant::now().duration_since(instant);
                elapsed.as_millis() as ms
            }
            _ => 0, // Return 0 if start time is not initialized
        }
    }
}
