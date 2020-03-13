use std::ops::Sub;
use std::time::Duration;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind")))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(std::time::Instant);

#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind")))]
impl Instant {
    pub fn now() -> Self {
        Self(std::time::Instant::now())
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind"))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind"))]
impl Instant {
    pub fn now() -> Self {
        let now = js_sys::Date::new_0();
        let millisecs_since_unix_epoch: u64 = now.get_time() as u64;

        Self(millisecs_since_unix_epoch as u64)
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_millis(self.0 - earlier.0)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}
