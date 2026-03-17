use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Wraps a job in a progress spinner that runs for the duration of the job.
pub fn run_with_spinner<F: FnOnce()>(message: &str, job: F) {
	let progress_bar = ProgressBar::new_spinner();
	progress_bar.set_style(
		ProgressStyle::default_spinner()
			.template(&format!(
				"{{spinner:.blue}} {} [{{elapsed_precise}}]",
				message
			))
			.expect("Failed to set progress bar template"),
	);
	progress_bar.enable_steady_tick(Duration::from_millis(100));
	job();
	progress_bar.finish_and_clear();
}
