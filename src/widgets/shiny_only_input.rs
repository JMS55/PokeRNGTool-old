use gtk::{CheckButton, ToggleButtonExt};

#[derive(Clone)]
pub struct ShinyOnlyInput {
    widget: CheckButton,
}

impl ShinyOnlyInput {
    pub fn new() -> Self {
        Self {
            widget: CheckButton::new_with_label("Shiny Only"),
        }
    }

    pub fn get_shiny_only(&self) -> bool {
        self.widget.get_active()
    }

    pub fn reset(&self) {
        self.widget.set_active(false);
    }

    pub fn widget(&self) -> &CheckButton {
        &self.widget
    }
}
