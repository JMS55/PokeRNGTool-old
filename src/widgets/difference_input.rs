use gtk::{SpinButton, SpinButtonExt};

#[derive(Clone)]
pub struct DifferenceInput {
    widget: SpinButton,
}

impl DifferenceInput {
    pub fn new() -> Self {
      let widget = SpinButton::new_with_range(0.0, 200.0, 1.0);
        widget.set_value(18.0);
        Self { widget }
    }

    pub fn get_difference(&self) -> u32 {
        self.widget.get_value() as u32
    }

    pub fn reset(&self) {
        self.widget.set_value(18.0);
    }

    pub fn widget(&self) -> &SpinButton {
        &self.widget
    }
}
