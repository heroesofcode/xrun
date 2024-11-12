use colored::Colorize;
use crate::results::*;

pub struct Utils;

impl Utils {
    pub fn show_message_success_with_file(test_errors: Vec<(String, String)>) {
        Results::validation_show_errors(test_errors, true);
        println!("{}", "results-xrun.pdf file generated successfully".green());
    }
}