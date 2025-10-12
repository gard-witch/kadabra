slint::include_modules!();
mod layout;

use arboard::Clipboard;
use clap::Parser;

use crate::layout::{Layout, LayoutKind};

fn main() -> Result<(), slint::PlatformError> {
    let cli = Cli::parse();
    let user_text = selection::get_text();
    let ui = MainWindow::new()?;

    let result = convert(&user_text, cli.from.into(), cli.to.into());

    ui.set_text(result.into());

    ui.on_exit_requested(|| {
        let _ = slint::quit_event_loop();
    });

    let mut clipboard = Clipboard::new().unwrap();
    let text = ui.get_text();
    ui.on_copy_requested(move || {
        let _ = clipboard.set_text(&*text);

        let _ = slint::invoke_from_event_loop(|| {
            slint::quit_event_loop().unwrap();
        });
    });

    ui.run()
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
