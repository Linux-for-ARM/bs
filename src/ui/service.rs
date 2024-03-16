use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::views::ListView;
use cursive::views::Panel;
use cursive::views::TextView;

use cursive::traits::Nameable;

use crate::conf::Defaults;
use crate::ui::help::help_window;

pub fn defaults_edit_win(scr: &mut Cursive, defs: &Defaults) {
    let text = TextView::new("These are the default settings for each LFA build session.");
    let defaults = ListView::new()
        .child(
            "System name:",
            EditView::new()
                .content(&defs.system_name)
                .with_name("defaults.system_name"),
        )
        .child(
            "System version:",
            EditView::new()
                .content(&defs.system_version)
                .with_name("defaults.system_version"),
        );

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(defaults));

    let win = Dialog::around(layout)
        .title("Edit defaults")
        .button("Save", |_| {})
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Help", |s| help_window(s, "defaults"));
    scr.add_layer(win);
}
