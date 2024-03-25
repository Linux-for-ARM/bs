//! Error windows

use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::Panel;
use cursive::views::TextView;
use cursive::With;

/// Constructs and displays an error window in the program
pub struct Error {
    text: String,
    is_exit: bool,
}

impl Error {
    /// Creates a new instance of the window
    pub fn new<S: ToString>(txt: S) -> Self {
        Self {
            text: txt.to_string(),
            is_exit: false,
        }
    }

    /// Sets the behavior of the window
    ///
    /// If the error is uncorrectable, set `true`, in which case you can only
    /// abort the program when the error occurs.
    ///
    /// If the error is non-crititcal, then do not use this methor or set it
    /// to `false`, in which case you will still have the ability to close
    /// the error window and continue working.
    ///
    /// ## Example
    /// ```
    /// use bs::tui::error::Error;
    /// let error = Error::new("some message").set_exit(true);
    /// ```
    pub fn set_exit(mut self, exit: bool) -> Self {
        self.is_exit = exit;
        self
    }

    /// Displays window to the terminal
    pub fn show(&self, scr: &mut Cursive) {
        let text = TextView::new(&self.text).scrollable();
        let win = Dialog::around(text).title("Error").with(|s| {
            if self.is_exit {
                s.add_button("Exit", |s| s.quit());
            } else {
                s.add_button("OK", |s| {
                    s.pop_layer();
                });
            }
        });
        scr.add_layer(win);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_exit_test() {
        let error = Error::new("some message").set_exit(true);
        assert!(error.is_exit);
    }
}
