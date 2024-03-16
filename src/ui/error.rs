use std::fmt::Display;

use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextView;

pub fn _error_short_window<E: Display>(scr: &mut Cursive, err: E) {
    let text = TextView::new(format!("{err}"));
    let win = Dialog::around(text)
        .title("Error")
        .button("Close", |s| {
            s.pop_layer();
        })
        .button("Quit", |s| super::is_quit(s));
    scr.add_layer(win);
}

pub fn error_full_window<E: Display>(scr: &mut Cursive, short: &str, err: E) {
    let text = TextView::new(short);
    let err_text = TextView::new(format!("{err}"));

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(err_text).title("Details"));
    let win = Dialog::around(layout)
        .title("Error")
        .button("Close", |s| {
            s.pop_layer();
        })
        .button("Quit", |s| super::is_quit(s));
    scr.add_layer(win);
}
