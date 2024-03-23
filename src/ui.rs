pub mod error;
pub mod help;
pub mod master;
pub mod service;

use cursive::event::Key;
use cursive::menu::Tree;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::views::TextView;
use cursive::Cursive;

use crate::conf::Defaults;
use crate::conf::MBoardsAll;
use crate::traits::TomlConfig;

use crate::consts::MBOARDS_LIST;
use crate::consts::DEFAULTS;

pub fn is_quit(scr: &mut Cursive) {
    let win = Dialog::around(TextView::new(
        "Really quit? All your changes will not be saved.",
    ))
    .button("Cancel", |s| {
        s.pop_layer();
    })
    .button("Quit", |s| s.quit());

    scr.add_layer(win);
}

fn master_window(scr: &mut Cursive) {
    let text = TextView::new(
        "Arrow keys navigate the menu. <Enter> selects submenus.\n\
                                                 Press <Esc> key to exit, <F1> to select menubar.",
    );
    let copyright = TextView::new("LFA BS. Copyright (C) 2024 Pelmen Zharenny. Written in Rust.");

    let mut menu = SelectView::new().autojump();

    menu.add_item("Motherboard select", 1);
    menu.add_item("Kernel version select [OPTIONAL]", 2);
    menu.add_item("Kernel configuration", 3);
    menu.add_item("Default values (sys name, version, etc.)", 4);

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(menu))
        .child(copyright);

    let win = Dialog::around(layout)
        .title("LFA System Configuration")
        .button("Write", |_| {})
        .button("Exit", |s| is_quit(s));
    scr.add_layer(win);
}

pub fn menuconfig_window() {
    let mut scr = cursive::default();

    scr.menubar()
        .add_subtree("File", Tree::new().leaf("Quit", |s| is_quit(s)))
        .add_subtree(
            "Windows",
            Tree::new()
                .leaf("Configuration", |s| master_window(s))
                .leaf("Edit defaults", |s| {
                    match Defaults::parse(DEFAULTS) {
                        Ok(conf) => service::defaults_edit_win(s, &conf),
                        Err(why) => error::error_full_window(
                            s,
                            "Parsing error of the default settings file",
                            why,
                        ),
                    }
                }),
        )
        .add_subtree(
            "Help",
            Tree::new()
                .leaf("Please donate me ;)", |s| help::donate_window(s))
                .leaf("About program", |s| help::about_window(s)),
        );

    scr.set_autohide_menu(false);
    scr.set_global_callback(Key::F1, |s| s.select_menubar());

    match MBoardsAll::parse(MBOARDS_LIST) {
        Ok(conf) => master::mboards_select_window(&mut scr, &conf),
        Err(why) => error::error_full_window(
            &mut scr,
            "Error parsing a file with a list of motherboards!",
            why,
        ),
    }

    scr.run();
}
