use cli_status::{ProgressBar, ProgressBarStyle};
use std::io::Write;

fn main() {
    let mut pg_bar = ProgressBar::new();
    pg_bar.set_title(String::from("Download..."));
    pg_bar.set_length(10);
    pg_bar.set_style(ProgressBarStyle::DoubleArrow);
    pg_bar.set_show_progress(true);
    
    for i in 0..11 {
        pg_bar.set_progress(i * 10);
        pg_bar.print();

        std::thread::sleep(std::time::Duration::from_millis(300));
        std::io::stdout().flush().unwrap();
    }

    println!();
}