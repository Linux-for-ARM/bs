//! Text User Interface

pub mod error;
pub mod help;
pub mod master;
pub mod service;

use cursive::event::Key;
use cursive::menu::Tree;
use cursive::view::Resizable;
use cursive::Cursive;

use cursive::align::Align;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextView;

pub fn main_window() {
    let mut scr = cursive::default();
    menubar(&mut scr);
    keybinds(&mut scr);

    scr.run();
}

fn menubar(scr: &mut Cursive) {
    scr.menubar()
        .add_subtree("File", Tree::new().leaf("Quit", |s| is_exit(s)))
        .add_subtree(
            "Help",
            Tree::new()
                .leaf("About program", about_win)
                .leaf("Please donate me", donate_me_win),
        );
    scr.set_autohide_menu(false);
}

fn keybinds(scr: &mut Cursive) {
    scr.add_global_callback(Key::F1, |s| s.select_menubar());
}

fn is_exit(scr: &mut Cursive) {
    let text = TextView::new("Really quit?");
    let win = Dialog::around(text)
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Quit", |s| s.quit());
    scr.add_layer(win);
}

fn about_win(scr: &mut Cursive) {
    let text = TextView::new(format!(
        "{}-{} (Linux for ARM project)\nCopyright (C) 2024 Pelmen Zharenny <t.me/brezhnev_zhiv>",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
    let win = Dialog::around(text)
        .title("About program")
        .button("OK", |s| {
            s.pop_layer();
        })
        .button("Please donate me", donate_me_win);
    scr.add_layer(win);
}

fn donate_me_win(scr: &mut Cursive) {
    let header = TextView::new("Над программой работает один человек в одиночку. Вы \
                                                   можете отблагодарить его, отправив донат на карту. \
                                                   Это покажет, что проект LFA нужен людям и даст силы \
                                                   продолжать работу над ним дальше.");
    let footer = TextView::new("Проект LFA является свободным ПО, за которое не \
                                                   платят его разработчику. Обратная связь от \
                                                   пользователей и добровольные пожертвования \
                                                   помогают сохранить мотивацию работать над LFA \
                                                   дальше. Также мне (разработчику LFA) нужен новый \
                                                   ноутбук, на котором я смог бы работать над проектом \
                                                   ну и по мелочи всякие разные одноплатники для \
                                                   тестирования LFA.");
    let card =
        TextView::new("2202 2062 5233 5406 (Сбербанк)\nМихаил Сергеевич К.").align(Align::center());

    let layout = LinearLayout::vertical()
        .child(header)
        .child(Panel::new(card))
        .child(footer)
        .max_width(55);

    let win = Dialog::around(layout)
        .title("Please donate me")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}
