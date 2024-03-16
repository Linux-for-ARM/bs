mod conf;
mod run;
mod traits;

mod ui;

use std::env::var;

fn main() {
    ui::menuconfig_window();
    println!(
        "{}/{}",
        var("LFA_BS_MBOARD").unwrap(),
        var("LFA_BS_KERNEL").unwrap()
    );
}
