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
            "Arena Trap" => Some(ArenaTrap),
            "Battle Armor" => Some(BattleArmor),
            "Blaze" => Some(Blaze),
            "Chlorophyll" => Some(Chlorophyll),
            "Clear Body" => Some(ClearBody),
            "Cloud Nine" => Some(CloudNine),
            "Color Change" => Some(ColorChange),
            "Compound Eyes" => Some(CompoundEyes),
            "Cute Charm" => Some(CuteCharm),
            "Damp" => Some(Damp),
            "Drizzle" => Some(Drizzle),
            "Drought" => Some(Drought),
            "Early Bird" => Some(EarlyBird),
            "Effect Spore" => Some(EffectSpore),
            "Flame Body" => Some(FlameBody),
            "Flash Fire" => Some(FlashFire),
            "Forecast" => Some(Forecast),
            "Guts" => Some(Guts),
            "Huge Power" => Some(HugePower),
            "Hustle" => Some(Hustle),
            "Hyper Cutter" => Some(HyperCutter),
            "Illuminate" => Some(Illuminate),
            "Immunity" => Some(Immunity),
            "Inner Focus" => Some(InnerFocus),
            "Insomnia" => Some(Insomnia),
            "Intimidate" => Some(Intimidate),
            "Keen Eye" => Some(KeenEye),
            "Levitate" => Some(Levitate),
            "Lightning Rod" => Some(LightningRod),
            "Liquid Ooze" => Some(LiquidOoze),
            "Magma Armor" => Some(MagmaArmor),
            "Magnet Pull" => Some(MagnetPull),
            "Minus" => Some(Minus),
            "Natural Cure" => Some(NaturalCure),
            "Oblivious" => Some(Oblivious),
            "Overgrow" => Some(Overgrow),
            "Own Tempo" => Some(OwnTempo),
            "Pickup" => Some(Pickup),
            "Plus" => Some(Plus),
            "Poison Point" => Some(PoisonPoint),
            "Pressure" => Some(Pressure),
            "Pure Power" => Some(PurePower),
            "Rain Dish" => Some(RainDish),
            "Rock Head" => Some(RockHead),
            "Rough Skin" => Some(RoughSkin),
            "Run Away" => Some(RunAway),
            "Sand Veil" => Some(SandVeil),
            "Serene Grace" => Some(SereneGrace),
            "Shadow Tag" => Some(ShadowTag),
            "Shed Skin" => Some(ShedSkin),
            "Shell Armor" => Some(ShellArmor),
            "Shield Dust" => Some(ShieldDust),
            "Soundproof" => Some(Soundproof),
            "Speed Boost" => Some(SpeedBoost),
            "Static" => Some(Static),
            "Stench" => Some(Stench),
            "Sticky Hold" => Some(StickyHold),
            "Sturdy" => Some(Sturdy),
            "Suction Cups" => Some(SuctionCups),
            "Swarm" => Some(Swarm),
            "Swift Swim" => Some(SwiftSwim),
            "Synchronize" => Some(Synchronize),
            "Thick Fat" => Some(ThickFat),
            "Torrent" => Some(Torrent),
            "Trace" => Some(Trace),
            "Truant" => Some(Truant),
            "Vital Spirit" => Some(VitalSpirit),
            "Volt Absorb" => Some(VoltAbsorb),
            "Water Absorb" => Some(WaterAbsorb),
            "Water Veil" => Some(WaterVeil),
            "White Smoke" => Some(WhiteSmoke),
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
