/* 22.03.2024. Крокус-Сити Холл. Помним, любим, скорбим. */

mod consts;
mod conf;
mod run;
mod traits;
mod reqs;

mod cli;
mod ui;

use std::{env::var, process::exit};
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
        WorkMode::Build => {
            if !check_reqs() {
                exit(1);
            }
        },
    }
}

fn check_reqs() -> bool {
    let hard = reqs::Hardware::new().unwrap();

    let ram = hard.ram_size;
    let processors = hard.cpu_cores;

    println!("{ram} {processors}");

    if ram < consts::MIN_REQ_RAM {
        eprintln!("Error: Your computer is not compatible with the requirements of the build system");
        eprintln!("Not enough RAM (minimum {} MB)", consts::MIN_REQ_RAM);

        return false;
    }

    if processors < consts::MIN_REQ_PROCESSORS {
        eprintln!("Error: Your computer is not compatible with the requirements of the build system");
        eprintln!("Not enough processor cores (minimum {})", consts::MIN_REQ_PROCESSORS);

        return false;
    }

    true
}