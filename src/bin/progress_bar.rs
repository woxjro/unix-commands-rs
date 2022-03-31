use indicatif::ProgressBar;
use indicatif::{ProgressIterator, ProgressStyle};
use std::{thread, time};
fn main() {
    let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    for _ in 0..1000 {
        let duration = time::Duration::from_millis(2);
        thread::sleep(duration);

        bar.inc(1);
    }
    bar.finish();

    for _ in (0..1000).progress() {
        let duration = time::Duration::from_millis(2);
        thread::sleep(duration);
    }

    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        //.key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(time::Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
