use std::time::{Duration, Instant};

/// A simple timer for tracking elapsed time in the game.
pub struct Timer {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Timer {
    /// Creates a new timer instance.
    pub fn new() -> Self {
        Timer {
            start_time: None,
            elapsed: Duration::new(0, 0),
            running: false,
        }
    }

    /// Starts the timer.
    ///
    /// If the timer is already running, this does nothing.
    pub fn start(&mut self) {
        if !self.running {
            self.start_time = Some(Instant::now());
            self.running = true;
        }
    }

    /// Stops the timer and accumulates elapsed time.
    ///
    /// If the timer is not running, this does nothing.
    pub fn stop(&mut self) {
        if self.running {
            if let Some(start) = self.start_time {
                self.elapsed += start.elapsed();
            }
            self.start_time = None;
            self.running = false;
        }
    }

    /// Resets the timer to zero and stops it.
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::new(0, 0);
        self.running = false;
    }

    /// Returns the total elapsed time as a `Duration`.
    ///
    /// If the timer is running, includes the time since it was started.
    pub fn get_elapsed(&self) -> Duration {
        if self.running {
            if let Some(start) = self.start_time {
                return self.elapsed + start.elapsed();
            }
        }
        self.elapsed
    }

    /// Returns the total elapsed time in seconds.
    pub fn get_elapsed_seconds(&self) -> u64 {
        self.get_elapsed().as_secs()
    }
}
