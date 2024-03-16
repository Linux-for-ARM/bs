pub mod error;
pub mod help;
pub mod master;
pub mod service;

use cursive::event::Key;
use cursive::menu::Tree;
use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::Cursive;

use crate::conf::Defaults;
use crate::conf::MBoardsAll;
use crate::traits::TomlConfig;

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

pub fn menuconfig_window() {
    let mut scr = cursive::default();

    scr.menubar()
        .add_subtree("File", Tree::new().leaf("Quit", |s| is_quit(s)))
        .add_subtree(
            "Edit",
            Tree::new().leaf("Edit defaults", |s| {
                match Defaults::parse("data/defaults.toml") {
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

    match MBoardsAll::parse("data/mboards.toml") {
        Ok(conf) => master::mboards_select_window(&mut scr, &conf),
        Err(why) => error::error_full_window(
            &mut scr,
            "Error parsing a file with a list of motherboards!",
            why,
        ),
    }

    scr.run();
}
