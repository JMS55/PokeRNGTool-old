use crate::misc::{Gender, GenderRatio};
use crate::widgets::SpeciesInput;
use gtk::{ComboBoxExtManual, ComboBoxText, ComboBoxTextExt, ContainerExt, WidgetExt};

#[derive(Clone)]
pub struct GenderInput {
    widget: ComboBoxText,
}

impl GenderInput {
    pub fn new(species_input: &SpeciesInput) -> Self {
        let widget = ComboBoxText::new();
        match species_input.get_current_species().get_gender_ratio() {
            GenderRatio::Genderless => widget.append_text("Genderless"),
            GenderRatio::AlwaysMale => widget.append_text("Male"),
            GenderRatio::AlwaysFemale => widget.append_text("Female"),
            _ => {
                widget.append_text("Any");
                widget.append_text("Male");
                widget.append_text("Female");
            }
        }
        widget.set_active(0);
        match species_input.get_current_species().get_gender_ratio() {
            GenderRatio::Genderless | GenderRatio::AlwaysMale | GenderRatio::AlwaysFemale => {
                widget.set_sensitive(false)
            }
            _ => widget.set_sensitive(true),
        }

        species_input.widget().connect_add({
            let species_input = species_input.clone();
            let widget = widget.clone();
            move |_, _| {
                widget.remove_all();
                match species_input.get_current_species().get_gender_ratio() {
                    GenderRatio::Genderless => widget.append_text("Genderless"),
                    GenderRatio::AlwaysMale => widget.append_text("Male"),
                    GenderRatio::AlwaysFemale => widget.append_text("Female"),
                    _ => {
                        widget.append_text("Any");
                        widget.append_text("Male");
                        widget.append_text("Female");
                    }
                }
                widget.set_active(0);
                match species_input.get_current_species().get_gender_ratio() {
                    GenderRatio::Genderless
                    | GenderRatio::AlwaysMale
                    | GenderRatio::AlwaysFemale => widget.set_sensitive(false),
                    _ => widget.set_sensitive(true),
                }
            }
        });
        Self { widget }
    }

    pub fn get_gender(&self) -> Option<Gender> {
        use Gender::*;
        if !self.widget.get_sensitive() {
            return None;
        }
        match self.widget.get_active_text().unwrap().as_ref() {
            "Any" => None,
            "Male" => Some(Male),
            "Female" => Some(Female),
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
