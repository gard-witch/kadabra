mod layout;

use clap::Parser;

use crate::layout::{Layout, LayoutKind};

fn main() {
    let cli = Cli::parse();
    let user_text = selection::get_text();

    let result = convert(&user_text, cli.from.into(), cli.to.into());
    println!("{result}");
}

fn convert(user_text: &str, from: &Layout, to: &Layout) -> String {
    let mut result = String::with_capacity(user_text.len());
    for ch in user_text.trim_end().chars() {
        let new_ch = if let Some(key) = from.get_by_char(ch) {
            to.get_by_key(key)
        } else {
            ch
        };
        result.push(new_ch);
    }
    result
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, value_enum)]
    from: LayoutKind,

    #[arg(short, long, value_enum)]
    to: LayoutKind,
}
