use cursive::Cursive;
use std::env::set_var;

use cursive::traits::Scrollable;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::views::TextView;

use crate::conf::MBoardSettings;
use crate::conf::MBoardsAll;
use crate::traits::TomlConfig;
use crate::ui::help::help_window;

pub fn mboards_select_window(scr: &mut Cursive, mboards: &MBoardsAll) {
    let text = TextView::new(
        "Please select the motherboard/computer for which you will be building your LFA system.",
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
        .child(Panel::new(boards.scrollable()));

    let win = Dialog::around(layout)
        .title("Select your motherboard")
        .button("Cancel", |s| super::is_quit(s))
        .button("Help", |s| help_window(s, "select_mboard"));

    scr.add_layer(win);
}

pub fn mboard_select_kernel_window(scr: &mut Cursive, mboard_settings: &MBoardSettings) {
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

fn on_selected_mboard(scr: &mut Cursive, board: &String) {
    scr.pop_layer();
    set_var("LFA_BS_MBOARD", board);

    let mboard_config = MBoardSettings::parse(format!("data/{}/kernel.toml", board)).unwrap();
    mboard_select_kernel_window(scr, &mboard_config);
}

fn on_selected_kernel(scr: &mut Cursive, kernel: &String) {
    set_var("LFA_BS_KERNEL", kernel);
    scr.quit();
}
