/* 22.03.2024. Крокус-Сити Холл. Помним, любим, скорбим. */

mod consts;
mod conf;
mod run;
mod traits;

mod cli;
mod ui;

use std::env::var;
use clap::Parser;

use cli::WorkMode;

fn main() {
    // ui::menuconfig_window();
    let cli = cli::Cli::parse();
    match cli.work_mode {
        WorkMode::Menuconfig => {
            ui::menuconfig_window();
            println!(
                "{}/{} ({})",
                var("LFA_BS_MBOARD").unwrap(),
                var("LFA_BS_KERNEL").unwrap(),
                var("LFA_BS_KERNEL_CONF_MODE").unwrap(),
            );
        },
        WorkMode::ConfigureHost => todo!(),
        WorkMode::Build => todo!(),
    }
}
