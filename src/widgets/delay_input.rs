use gtk::{SpinButton, SpinButtonExt};

#[derive(Clone)]
pub struct DelayInput {
    widget: SpinButton,
}

impl DelayInput {
    pub fn new() -> Self {
        Self {
            widget: SpinButton::new_with_range(0.0, 10000.0, 1.0),
        }
    }

    pub fn get_delay(&self) -> u32 {
        self.widget.get_value() as u32
    }

    pub fn reset(&self) {
        self.widget.set_value(0.0);
    }

    pub fn widget(&self) -> &SpinButton {
        &self.widget
    }
}
