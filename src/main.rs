slint::include_modules!();
mod layout;

use clap::Parser;
use arboard::Clipboard;

use crate::layout::{Layout, LayoutKind};

fn main() {
    
    //let cli = Cli::parse();
    let user_text = selection::get_text();
    let ui = MainWindow::new().unwrap();
    let mut clipboard = Clipboard::new().unwrap();

    // let result = convert(&user_text, cli.from.into(), cli.to.into());
    let result = convert(&user_text, LayoutKind::En.into(), LayoutKind::Ru.into());

    // println!("{result}");
    ui.set_note(result.into());

    ui.on_exit_requested({
        let ui_weak = ui.as_weak();
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                let _ = ui.hide();
            }
            slint::quit_event_loop().unwrap();
        }
    });

    ui.on_copy_requested({
         let current = ui.get_note();
        move || {
            //  println!("{current}");
            clipboard.set_text(&*current);
        }
    });

    ui.run().unwrap();

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
