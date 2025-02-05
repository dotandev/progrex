use progrex::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bar = ProgressBar::new(100);

    for i in 0..=100 {
        bar.set_progress(i);
        thread::sleep(Duration::from_millis(50));
    }

    bar.finish();
}
