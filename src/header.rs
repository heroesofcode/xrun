use clap::Command;
use rascii_art::{render_to, RenderOptions};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Header;

impl Header {
    pub fn show_header() {
        Self::validation_helper();

        if std::env::var("XRUN_NO_HEADER").is_ok() {
            return;
        }

        let mut buffer = String::new();

        let rendered = render_to(
            "assets/xcode.png",
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

    fn validation_helper() {
        let _ = Command::new(NAME)
            .version(VERSION)
            .ignore_errors(true)
            .get_matches();
    }
}