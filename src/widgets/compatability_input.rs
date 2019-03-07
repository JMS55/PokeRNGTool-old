use gtk::{ComboBoxExtManual, ComboBoxText, ComboBoxTextExt};

#[derive(Clone)]
pub struct CompatabilityInput {
    widget: ComboBoxText,
}

impl CompatabilityInput {
    pub fn new() -> Self {
        let widget = ComboBoxText::new();
        widget.append_text("The two don't seem to like each other");
        widget.append_text("The two seem to get along");
        widget.append_text("The two seem to get along very well");
        widget.set_active(0);
        Self { widget }
    }

    pub fn get_compatability(&self) -> u32 {
        match self.widget.get_active_text().unwrap().as_ref() {
            "The two don't seem to like each other" => 20,
            "The two seem to get along" => 50,
            "The two seem to get along very well" => 70,
            _ => unreachable!(),
        }
    }

    pub fn reset(&self) {
        self.widget.set_active(0);
    }

    pub fn widget(&self) -> &ComboBoxText {
        &self.widget
    }
}
