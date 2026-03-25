use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::exit;

#[derive(Parser)]
#[command(name = "xrun")]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	/// Run tests for an Xcode project (.xcodeproj)
	Project {
		/// Path to the .xcodeproj file
		path: String,
		/// Xcode scheme to test
		scheme: String,
		/// Target platform: `macOS` or an iOS version string (e.g. `17.4`)
		platform: String,
		/// iPhone model number — required for iOS (e.g. `15` → `name=iPhone 15`)
		device: Option<String>,
		/// Exit non-zero on test failures
		#[arg(long)]
		fail: bool,
		/// Write results-xrun.pdf on test failure
		#[arg(long = "generate-file")]
		generate_file: bool,
	},
	/// Run tests for an Xcode workspace (.xcworkspace)
	Workspace {
		/// Path to the .xcworkspace file
		path: String,
		/// Xcode scheme to test
		scheme: String,
		/// Target platform: `macOS` or an iOS version string (e.g. `17.4`)
		platform: String,
		/// iPhone model number — required for iOS (e.g. `15` → `name=iPhone 15`)
		device: Option<String>,
		/// Exit non-zero on test failures
		#[arg(long)]
		fail: bool,
		/// Write results-xrun.pdf on test failure
		#[arg(long = "generate-file")]
		generate_file: bool,
	},
}

pub struct AppArgs {
	pub build_type: String,
	pub project_path: String,
	pub scheme: String,
	pub platform: String,
	pub device: Option<String>,
	pub fail: bool,
	pub generate_file: bool,
}

impl AppArgs {
	pub fn parse() -> Self {
		let cli = Cli::parse();

		let (build_type, path, scheme, platform, device, fail, generate_file) = match cli.command {
			Command::Project {
				path,
				scheme,
				platform,
				device,
				fail,
				generate_file,
			} => (
				"-project".to_string(),
				path,
				scheme,
				platform,
				device,
				fail,
				generate_file,
			),
			Command::Workspace {
				path,
				scheme,
				platform,
				device,
				fail,
				generate_file,
			} => (
				"-workspace".to_string(),
				path,
				scheme,
				platform,
				device,
				fail,
				generate_file,
			),
		};

		if platform != "macOS" && device.is_none() {
			eprintln!(
				"{}",
				"Error: Device argument is required for iOS testing".red()
			);
			exit(1);
		}

		AppArgs {
			build_type,
			project_path: path,
			scheme,
			platform,
			device,
			fail,
			generate_file,
		}
	}
}
