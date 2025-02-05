use std::io::{self, Write};
// use std::thread;
use std::time::{
    // Duration, 
    Instant
};

pub struct ProgressBar {
    total: usize,
    current: usize,
    start_time: Instant,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            start_time: Instant::now(),
        }
    }

    pub fn set_progress(&mut self, value: usize) {
        self.current = value;
        self.display();
    }

    fn display(&self) {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let elapsed = self.start_time.elapsed().as_secs();
        let estimated_total_time = if self.current > 0 {
            (elapsed as f64 / self.current as f64) * self.total as f64
        } else {
            0.0
        };

        let eta = estimated_total_time - elapsed as f64;
        let progress_bar = "█".repeat(self.current * 50 / self.total)
            + &" ".repeat(50 - (self.current * 50 / self.total));

        print!("\r[{}] {:.2}% | ETA: {:.1}s", progress_bar, percentage, eta);
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        println!("\n✅ Progress Complete!");
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
