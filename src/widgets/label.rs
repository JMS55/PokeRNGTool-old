use gtk::LabelExt;

#[derive(Clone)]
pub struct Label {
    widget: gtk::Label,
}

impl Label {
    pub fn new(text: &'static str) -> Self {
        let widget = gtk::Label::new(text);
        widget.set_xalign(1.0);
        Self { widget }
    }

    pub fn widget(&self) -> &gtk::Label {
        &self.widget
    }
}
