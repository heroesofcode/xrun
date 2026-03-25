use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use std::process::exit;

#[derive(Parser)]
#[command(name = "xrun")]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

/// Shared arguments for `project` and `workspace` subcommands.
#[derive(Args)]
struct RunArgs {
	/// Path to the .xcodeproj or .xcworkspace file
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
}

#[derive(Subcommand)]
enum Command {
	/// Run tests for an Xcode project (.xcodeproj)
	Project {
		#[command(flatten)]
		run: RunArgs,
	},
	/// Run tests for an Xcode workspace (.xcworkspace)
	Workspace {
		#[command(flatten)]
		run: RunArgs,
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

		let (build_type, RunArgs {
			path,
			scheme,
			platform,
			device,
			fail,
			generate_file,
		}) = match cli.command {
			Command::Project { run } => ("-project".to_string(), run),
			Command::Workspace { run } => ("-workspace".to_string(), run),
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
