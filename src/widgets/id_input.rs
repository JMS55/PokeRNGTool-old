use gtk::{SpinButton, SpinButtonExt};

#[derive(Clone)]
pub struct IDInput {
    widget: SpinButton,
}

impl IDInput {
    pub fn new() -> Self {
        Self {
            widget: SpinButton::new_with_range(0.0, u16::max_value() as f64, 1.0),
        }
    }

    pub fn get_id(&self) -> u16 {
        self.widget.get_value() as u16
    }

    pub fn reset(&self) {
        self.widget.set_value(0.0);
    }

    pub fn widget(&self) -> &SpinButton {
        &self.widget
    }
}
