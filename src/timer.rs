use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: None,
            elapsed: Duration::new(0, 0),
            running: false,
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.start_time = Some(Instant::now());
            self.running = true;
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            if let Some(start) = self.start_time {
                self.elapsed += start.elapsed();
            }
            self.start_time = None;
            self.running = false;
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::new(0, 0);
        self.running = false;
    }

    pub fn get_elapsed(&self) -> Duration {
        if self.running {
            if let Some(start) = self.start_time {
                return self.elapsed + start.elapsed();
            }
        }
        self.elapsed
    }

    pub fn get_elapsed_seconds(&self) -> u64 {
        self.get_elapsed().as_secs()
    }
}
