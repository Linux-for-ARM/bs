use cursive::align::Align;
use cursive::traits::Resizable;
use cursive::traits::Scrollable;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextView;
use cursive::Cursive;

pub fn about_window(scr: &mut Cursive) {
    let text = TextView::new(format!(
        "LFA Automated Build System ver. {}\n\
        Copyright (C) 2024 Pelmen Zharenny <t.me/brezhnev_zhiv>",
        env!("CARGO_PKG_VERSION")
    ));
    let win = Dialog::around(text)
        .title("About program")
        .button("OK", |s| {
            s.pop_layer();
        });

    scr.add_layer(win);
}

pub fn donate_window(scr: &mut Cursive) {
    let hdr_text = TextView::new("There is one person working on the entire LFA, \n\
                                                     including this build system. You can send me a \n\
                                                     donation so that I can continue working on the LFA:");
    let card_text = TextView::new("2202 2062 5233 5406 Sberbank (Russia)").align(Align::center());

    let layout = LinearLayout::vertical()
        .child(hdr_text)
        .child(Panel::new(card_text));

    let win = Dialog::around(layout)
        .title("Please donate me ;)")
        .button("OK", |s| {
            s.pop_layer();
        });

    scr.add_layer(win);
}

pub fn help_window(scr: &mut Cursive, help_file: &str) {
    let help_file_pth = format!("Docs/menuconfig/{help_file}.txt");
    let help_file_content =
        std::fs::read_to_string(help_file_pth).unwrap_or("File doesn't exist!".to_string());

    let text = TextView::new(help_file_content)
        .scrollable()
        .max_width(60)
        .max_height(20);

    let win = Dialog::around(text).title("Help").button("OK", |s| {
        s.pop_layer();
    });
    scr.add_layer(win);
}
