use gtk::{Entry, EntryExt, Inhibit, WidgetExt};
use regex::Regex;

#[derive(Clone)]
pub struct SeedInput {
    widget: Entry,
}

impl SeedInput {
    pub fn new() -> Self {
        let widget = Entry::new();
        widget.set_text("0");
        widget.connect_focus_out_event({
            move |widget, _| {
                let validation_regex = Regex::new("^[[:xdigit:]]{1,8}$").unwrap();
                if !validation_regex.is_match(widget.get_text().unwrap().trim()) {
                    widget.set_text("0");
                }
                Inhibit(false)
            }
        });
        Self { widget }
    }

    pub fn get_seed(&self) -> u32 {
        u32::from_str_radix(&self.widget.get_text().unwrap(), 16).unwrap()
    }

    pub fn reset(&self) {
        self.widget.set_text("0");
    }

    pub fn widget(&self) -> &Entry {
        &self.widget
    }
}
