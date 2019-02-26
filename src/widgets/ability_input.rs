use crate::misc::Ability;
use crate::widgets::SpeciesInput;
use gtk::{ComboBoxExtManual, ComboBoxText, ComboBoxTextExt, ContainerExt, WidgetExt};

#[derive(Clone)]
pub struct AbilityInput {
    widget: ComboBoxText,
}

impl AbilityInput {
    pub fn new(species_input: &SpeciesInput) -> Self {
        let widget = ComboBoxText::new();
        match species_input.get_current_species().get_abilities() {
            (one, None) => {
                widget.append_text(one.to_str());
                widget.set_sensitive(false);
            }
            (one, Some(two)) => {
                widget.append_text("Any");
                widget.append_text(one.to_str());
                widget.append_text(two.to_str());
            }
        }
        widget.set_active(0);
        species_input.widget().connect_add({
            let species_input = species_input.clone();
            let widget = widget.clone();
            move |_, _| {
                widget.remove_all();
                match species_input.get_current_species().get_abilities() {
                    (one, None) => {
                        widget.append_text(one.to_str());
                        widget.set_sensitive(false);
                    }
                    (one, Some(two)) => {
                        widget.append_text("Any");
                        widget.append_text(one.to_str());
                        widget.append_text(two.to_str());
                        widget.set_sensitive(true);
                    }
                }
                widget.set_active(0);
            }
        });
        Self { widget }
    }

    pub fn get_ability(&self) -> Option<Ability> {
        use Ability::*;
        match self.widget.get_active_text().unwrap().as_ref() {
            "Any" => None,
            "Air Lock" => Some(AirLock),
            "Battle Armor" => Some(BattleArmor),
            "Blaze" => Some(Blaze),
            "Clear Body" => Some(ClearBody),
            "Color Change" => Some(ColorChange),
            "Drizzle" => Some(Drizzle),
            "Drought" => Some(Drought),
            "Forecast" => Some(Forecast),
            "Levitate" => Some(Levitate),
            "Overgrow" => Some(Overgrow),
            "Pressure" => Some(Pressure),
            "Rock Head" => Some(RockHead),
            "Shadow Tag" => Some(ShadowTag),
            "Soundproof" => Some(Soundproof),
            "Static" => Some(Static),
            "Sturdy" => Some(Sturdy),
            "Suction Cups" => Some(SuctionCups),
            "Synchronize" => Some(Synchronize),
            "Torrent" => Some(Torrent),
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
