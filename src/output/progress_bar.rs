use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::options::OutputStyleOption;

#[cfg(not(windows))]
const TICK_SETTINGS: (&str, u64) = ("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ", 80);

#[cfg(windows)]
const TICK_SETTINGS: (&str, u64) = (r"+-x| ", 200);

/// Return a pre-configured progress bar
pub fn get_progress_bar(length: u64, msg: &str, option: OutputStyleOption) -> ProgressBar {
    let is_basic_or_color = matches!(option, OutputStyleOption::Basic | OutputStyleOption::Color);

    let progressbar_style = if is_basic_or_color {
        ProgressStyle::default_bar()
    } else {
        ProgressStyle::default_spinner()
            .tick_chars(TICK_SETTINGS.0)
            .template(" {spinner} {msg:<30} {wide_bar} ETA {eta_precise} ")
            .expect("no template error")
    };

    let progress_bar = if is_basic_or_color {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(length)
    };
    progress_bar.set_style(progressbar_style);
    progress_bar.enable_steady_tick(Duration::from_millis(TICK_SETTINGS.1));
    progress_bar.set_message(msg.to_owned());

    progress_bar
}
