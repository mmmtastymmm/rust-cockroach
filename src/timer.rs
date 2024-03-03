use std::time::{Instant};

pub(crate) struct Timer {
    start: Instant,
}

impl Timer {
    // Creates a new timer object and starts the timer immediately
    pub fn start() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    // Returns the elapsed time in seconds since the timer was started
    pub fn elapsed(&mut self) -> f64 {
        let seconds = self.start.elapsed().as_millis();
        self.reset();
        seconds as f64
    }

    // Optionally, a method to reset the timer
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}