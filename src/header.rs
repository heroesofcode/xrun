use colored::Colorize;
use clap::Command;

pub fn header() {
    validation_helper();

    let text = r#"
    __  __    ____      _   _   _   _     
    \ \/"/ U |  _"\ uU |"|u| | | \ |"|    
    /\  /\  \| |_) |/ \| |\| |<|  \| |>   
   U /  \ u  |  _ <    | |_| |U| |\  |u   
    /_/\_\   |_| \_\  <<\___/  |_| \_|    
  ,-,>> \\_  //   \\_(__) )(   ||   \\,-. 
   \_)  (__)(__)  (__)   (__)  (_")  (_/  (0.11.0)
    "#;

    println!("{}", text);

    println!("💻 https://github.com/heroesofcode/xrun");
    println!("===================================================\n");
    println!("{}", "📋 Processing.......\n".blue());
}

fn validation_helper() {
    let _app = Command::new("xrun")
        .version("0.11.0")
        .ignore_errors(true)
        .get_matches();
}
