//! Displaying help information

use crate::error::Error;
use crate::utils::read_to_string;

use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::Cursive;

use cursive::traits::Resizable;
use cursive::traits::Scrollable;

pub fn help_window(scr: &mut Cursive, help: &str) {
    let help_file_pth = format!("Docs/menuconfig/{help}.txt");
    let help_content = match read_to_string(help_file_pth) {
        Ok(content) => content,
        Err(why) => format!("Read error: {why}"),
    };

    let text = TextView::new(help_content)
        .scrollable()
        .max_width(60)
        .max_height(20);

    let win = Dialog::around(text).title("Help").button("OK", |s| {
        s.pop_layer();
    });

    scr.add_layer(win);
}
