use colored::*;

pub fn report_error(message: &'static str) {
    println!("{} {}", "error:".bright_red().bold(), message.bold());
}