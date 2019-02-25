use gtk::{SpinButton, SpinButtonExt};

#[derive(Clone)]
pub struct FrameInput {
    widget: SpinButton,
}

impl FrameInput {
    pub fn new() -> Self {
        let widget = SpinButton::new_with_range(1.0, u32::max_value() as f64, 1.0);
        widget.set_value(50000.0);
        Self { widget }
    }

    pub fn get_max_frame(&self) -> u32 {
        self.widget.get_value() as u32
    }

    pub fn reset(&self) {
        self.widget.set_value(50000.0);
    }

    pub fn widget(&self) -> &SpinButton {
        &self.widget
    }
}
