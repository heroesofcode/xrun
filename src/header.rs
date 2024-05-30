use colored::Colorize;

pub fn header() {
    let text = r#"
    __  __    ____      _   _   _   _     
    \ \/"/ U |  _"\ uU |"|u| | | \ |"|    
    /\  /\  \| |_) |/ \| |\| |<|  \| |>   
   U /  \ u  |  _ <    | |_| |U| |\  |u   
    /_/\_\   |_| \_\  <<\___/  |_| \_|    
  ,-,>> \\_  //   \\_(__) )(   ||   \\,-. 
   \_)  (__)(__)  (__)   (__)  (_")  (_/  (0.6.0)
    "#;

    println!("{}", text);

    println!("💻 https://github.com/heroesofcode/xrun");
    println!("===================================================\n");
    println!("{}", "📋 Processing.......\n".blue());
}