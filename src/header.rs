use clap::Command;
use rascii_art::{render_to, RenderOptions};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Header;

impl Header {
    pub fn show_header() {
        Self::show_helper();
        Self::show_banner();
    }

    fn show_banner() {
        if std::env::var("XRUN_NO_HEADER").is_ok() {
            return;
        }

        let mut buffer = String::new();
        let rendered = render_to(
            "xcode.png",
            &mut buffer,
            &RenderOptions::new()
                .width(35)
                .height(15)
                .colored(true)
                .charset(&[".", ",", "-", "*", "£", "$", "#"]),
        );

        if rendered.is_ok() {
            println!("{buffer}");
        } else {
            println!("{NAME} v{VERSION}");
        }
        println!("⚙️  {NAME} v{VERSION}");
        println!("💻 https://github.com/heroesofcode/xrun");
        println!("📚 Run `{NAME} --help` to see available options.\n");
    }

    fn show_helper() {
        let _ = Command::new(NAME)
            .version(VERSION)
            .about("Run iOS and macOS unit tests from the terminal or CI with clean, readable output.")
            .override_help(
"USAGE:
    xrun <extension> <path> <scheme> <platform_or_version> [device] [fail] [generate-file]

ARGS:
    <extension>              Test entry type. One of:
                             - project    (.xcodeproj)
                             - workspace  (.xcworkspace)

    <path>                   Path to the Xcode project or workspace
                             (e.g. DeliveryApp.xcodeproj, DeliveryApp.xcworkspace)

    <scheme>                 Xcode scheme to be tested

    <platform_or_version>    For iOS: Xcode version (e.g. 17.4)
                             For macOS: literal \"macOS\"

    <device>                 (iOS only) iOS simulator runtime version
                             (e.g. 15 for iOS 15)

    fail                     When present, xrun will exit with a non-zero
                             status code if any test fails

    generate-file            When present and tests fail, generates
                             results-xrun.pdf with a table of failures

OPTIONS:
    -h, --help               Print help information
    -V, --version            Print version information
"
            )
            // keep this so clap handles `--help`/`--version` but doesn't complain
            // about your positional args (since you parse them yourself).
            .ignore_errors(true)
            .get_matches();
    }
}