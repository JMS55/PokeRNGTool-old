use crate::misc::IVComparator;
use crate::widgets::Label;
use gtk::{
    ComboBoxExt, ComboBoxExtManual, ComboBoxText, ComboBoxTextExt, SpinButton, SpinButtonExt,
    WidgetExt,
};

#[derive(Clone)]
pub struct IVWidgets {
    label: Label,
    spinner: SpinButton,
    combobox: ComboBoxText,
}

impl IVWidgets {
    pub fn new(label_text: &'static str) -> Self {
        let label = Label::new(label_text);
        let spinner = SpinButton::new_with_range(0.0, 31.0, 1.0);
        spinner.set_sensitive(false);
        let combobox = ComboBoxText::new();
        combobox.append_text("Any");
        combobox.append_text("==");
        combobox.append_text(">=");
        combobox.append_text("<=");
        combobox.set_active(0);
        combobox.connect_changed({
            let spinner = spinner.clone();
            move |combobox| {
                let sensitivity = match combobox.get_active_text().unwrap().as_ref() {
                    "Any" => false,
                    _ => true,
                };
                spinner.set_sensitive(sensitivity);
            }
        });
        Self {
            label,
            spinner,
            combobox,
        }
    }

    pub fn get_filters(&self) -> (IVComparator, u8) {
        use IVComparator::*;
        let comparator = match self.combobox.get_active_text().unwrap().as_ref() {
            "Any" => Any,
            "==" => Equal,
            ">=" => GreaterThanOrEqual,
            "<=" => LessThanOrEqual,
            _ => unreachable!(),
        };
        (comparator, self.spinner.get_value() as u8)
    }

    pub fn reset(&self) {
        self.spinner.set_value(0.0);
        self.combobox.set_active(0);
    }

    pub fn label_widget(&self) -> &gtk::Label {
        self.label.widget()
    }

    pub fn spinner_widget(&self) -> &SpinButton {
        &self.spinner
    }

    pub fn comparator_widget(&self) -> &ComboBoxText {
        &self.combobox
    }
}
