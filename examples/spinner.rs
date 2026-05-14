use cli_status::{Spinner, SpinnerStyle};
use std::io::Write;

fn main() {
    let mut spinner = Spinner::new();
    spinner.set_title(String::from("Download..."));
    spinner.set_style(SpinnerStyle::BrailleSpinnerSmooth);
    
    for _ in 0..50 {
        spinner.tick();

        std::thread::sleep(std::time::Duration::from_millis(100));
        std::io::stdout().flush().unwrap();
    }

    spinner.finish();

    println!();
}