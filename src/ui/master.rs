use cursive::views::RadioGroup;
use cursive::Cursive;
use std::env::set_var;

use cursive::traits::Resizable;
use cursive::traits::Scrollable;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::views::TextView;

use crate::conf::MBoardSettings;
use crate::conf::MBoardsAll;
use crate::traits::TomlConfig;

use crate::ui::error::error_full_window;
use crate::ui::help::help_window;

pub fn mboards_select_window(scr: &mut Cursive, mboards: &MBoardsAll) {
    let text = TextView::new(
        "Please select the motherboard/computer for which\n\
                  you will be building your LFA system.",
    );

    let mut mboards = mboards.mboard.clone();

    // Список материнок должен быть отсортирован для удобного доступа к ним
    // в меню. Кроме того, меню поддерживает autojump.
    mboards.sort();

    let mut boards = SelectView::new().autojump();
    boards.set_on_submit(on_selected_mboard);

    for board in &mboards {
        boards.add_item(&board.pretty, board.name.clone());
    }

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(boards.scrollable().max_height(15)));

    let win = Dialog::around(layout)
        .title("Select your motherboard")
        .button("Cancel", |s| super::is_quit(s))
        .button("Help", |s| help_window(s, "select_mboard"));

    scr.add_layer(win);
}

pub fn mboard_select_kernel_window(scr: &mut Cursive, mboard_settings: &MBoardSettings) {
    if mboard_settings.kernel.len() == 1 {
        on_selected_kernel(scr, &mboard_settings.kernel[0].archive);
        return ();
    }

    let text = TextView::new("There are several versions of the Linux kernel\n\
                                                 available for your motherboard. Select the desired\n\
                                                 kernel from the list below.");

    let mut kernels = SelectView::new();
    kernels.set_on_submit(on_selected_kernel);

    for kernel in &mboard_settings.kernel {
        kernels.add_item(&kernel.description, kernel.archive.clone());
    }

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(kernels));

    let win = Dialog::around(layout)
        .title("Select Linux kernel version")
        .button("Cancel", |s| super::is_quit(s))
        .button("Help", |s| help_window(s, "select_kernel"));

    scr.add_layer(win);
}

pub fn kernel_configuration_window(scr: &mut Cursive) {
    let text = TextView::new("Select a kernel configuration option:");
    let mut config = RadioGroup::new();
    config.set_on_change(on_change_kernel_conf);

    let config_mode = LinearLayout::vertical()
        .child(config.button(
            "default_lfa".to_string(),
            "Use the ready-made `.config' from the LFA developers",
        ))
        .child(config.button(
            "menuconfig".to_string(),
            "Run `make menuconfig' before compiling the kernel",
        ))
        .child(config.button(
            "defconfig".to_string(),
            "Run `make defconfig' before compiling the kernel",
        ));

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(config_mode));

    let win = Dialog::around(layout)
        .title("Kernel configuration mode")
        .button("OK", |s| s.quit())
        .button("Cancel", |s| super::is_quit(s))
        .button("Help", |s| help_window(s, "kernel_configuration"));

    scr.add_layer(win);
}

fn on_selected_mboard(scr: &mut Cursive, board: &String) {
    set_var("LFA_BS_MBOARD", board);

    match MBoardSettings::parse(format!("data/{}/kernel.toml", board)) {
        Ok(conf) => {
            scr.pop_layer();
            mboard_select_kernel_window(scr, &conf);
        }
        Err(why) => error_full_window(
            scr,
            "Error parsing the selected motherboard config file!",
            why,
        ),
    }
}

fn on_selected_kernel(scr: &mut Cursive, kernel: &String) {
    set_var("LFA_BS_KERNEL", kernel);
    scr.pop_layer();
    kernel_configuration_window(scr);
}

fn on_change_kernel_conf(_scr: &mut Cursive, conf_mode: &String) {
    set_var("LFA_BS_KERNEL_CONF_MODE", conf_mode);
}
