use crate::misc::Species;
use glib::object::Cast;
use gtk::{
    BoxExt, ButtonExt, ContainerExt, EntryExt, GtkWindowExt, IconSize, Image, Label, LabelExt,
    ListBox, ListBoxExt, ListBoxRow, MenuButton, MenuButtonExt, Orientation, Popover, PopoverExt,
    ScrolledWindow, ScrolledWindowExt, SearchEntry, SearchEntryExt, ShadowType, WidgetExt, Window,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct SpeciesInput {
    widget: MenuButton,
    listbox: ListBox,
}

impl SpeciesInput {
    pub fn new(species: Vec<Species>) -> Self {
        let widget = MenuButton::new();
        let popover = Popover::new(&widget);
        let vbox = gtk::Box::new(Orientation::Vertical, 6);
        let search_entry = SearchEntry::new();
        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let listbox = ListBox::new();

        let hbox = gtk::Box::new(Orientation::Horizontal, 3);
        let sprite = Image::new_from_file([Self::get_path_to_sprite(species[0]).as_path()][0]);
        let label = Label::new(species[0].to_str());
        let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
        hbox.add(&sprite);
        hbox.add(&label);
        hbox.pack_end(&icon, false, false, 0);
        widget.add(&hbox);

        for current_species in species {
            let row = gtk::Box::new(Orientation::Horizontal, 3);
            let icon =
                Image::new_from_file([Self::get_path_to_sprite(current_species).as_path()][0]);
            let label = Label::new(current_species.to_str());
            row.add(&icon);
            row.add(&label);
            listbox.add(&row);
        }

        popover.set_border_width(6);
        scrolled_window.set_shadow_type(ShadowType::In);
        scrolled_window.set_propagate_natural_height(true);
        listbox.connect_row_activated({
            let widget = widget.clone();
            let popover = popover.clone();
            move |_, row| {
                popover.popdown();
                widget.get_children()[0].destroy();
                let species_str = row.get_children()[0]
                    .clone()
                    .downcast::<gtk::Box>()
                    .unwrap()
                    .get_children()[1]
                    .clone()
                    .downcast::<Label>()
                    .unwrap()
                    .get_text()
                    .unwrap();
                let species = Self::parse_species_str(&species_str);
                let hbox = gtk::Box::new(Orientation::Horizontal, 3);
                let sprite = Image::new_from_file([Self::get_path_to_sprite(species).as_path()][0]);
                let label = Label::new(species.to_str());
                let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
                hbox.add(&sprite);
                hbox.add(&label);
                hbox.pack_end(&icon, false, false, 0);
                hbox.show_all();
                widget.add(&hbox);
            }
        });
        popover.connect_hide({
            let search_entry = search_entry.clone();
            move |_| {
                search_entry.set_text("");
            }
        });
        widget.connect_clicked({
            let scrolled_window = scrolled_window.clone();
            move |widget| {
                let window = widget.get_toplevel().unwrap().downcast::<Window>().unwrap();
                let height_limit = (window.get_size().1 as f32 * 0.9).ceil();
                scrolled_window.set_min_content_height(height_limit as i32);
                scrolled_window.set_max_content_height(height_limit as i32);
            }
        });
        search_entry.connect_search_changed({
            let listbox = listbox.clone();
            move |search_entry| {
                listbox.set_filter_func({
                    let search_entry = search_entry.clone();
                    Some(Box::new(move |row| {
                        row.get_children()[0]
                            .clone()
                            .downcast::<gtk::Box>()
                            .unwrap()
                            .get_children()[1]
                            .clone()
                            .downcast::<Label>()
                            .unwrap()
                            .get_text()
                            .unwrap()
                            .to_lowercase()
                            .contains(search_entry.get_text().unwrap().as_str())
                    }))
                });
                listbox.invalidate_filter();
            }
        });

        scrolled_window.add(&listbox);
        vbox.add(&search_entry);
        vbox.add(&scrolled_window);
        vbox.show_all();
        popover.add(&vbox);
        widget.set_popover(&popover);

        Self { widget, listbox }
    }

    pub fn set_species(&self, species: Vec<Species>) {
        for row in self.listbox.get_children() {
            row.destroy();
        }
        for current_species in species {
            let row = gtk::Box::new(Orientation::Horizontal, 3);
            let icon =
                Image::new_from_file([Self::get_path_to_sprite(current_species).as_path()][0]);
            let label = Label::new(current_species.to_str());
            row.add(&icon);
            row.add(&label);
            self.listbox.add(&row);
        }
        self.reset();
    }

    pub fn get_current_species(&self) -> Species {
        let species_str = self.widget.get_children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .get_children()[1]
            .clone()
            .downcast::<Label>()
            .unwrap()
            .get_text()
            .unwrap();
        Self::parse_species_str(&species_str)
    }

    pub fn reset(&self) {
        self.widget.get_children()[0].destroy();
        let species_str = self.listbox.get_children()[0]
            .clone()
            .downcast::<ListBoxRow>()
            .unwrap()
            .get_children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .get_children()[1]
            .clone()
            .downcast::<Label>()
            .unwrap()
            .get_text()
            .unwrap();
        let species = Self::parse_species_str(&species_str);
        let hbox = gtk::Box::new(Orientation::Horizontal, 3);
        let sprite = Image::new_from_file([Self::get_path_to_sprite(species).as_path()][0]);
        let label = Label::new(species.to_str());
        let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
        hbox.add(&sprite);
        hbox.add(&label);
        hbox.pack_end(&icon, false, false, 0);
        hbox.show_all();
        self.widget.add(&hbox);
    }

    pub fn widget(&self) -> &MenuButton {
        &self.widget
    }

    fn parse_species_str(species_str: &str) -> Species {
        use Species::*;
        match species_str {
            "Abra" => Abra,
            "Absol" => Absol,
            "Aerodactyl" => Aerodactyl,
            "Aipom" => Aipom,
            "Anorith" => Anorith,
            "Aron" => Aron,
            "Azurill" => Azurill,
            "Bagon" => Bagon,
            "Baltoy" => Baltoy,
            "Barboach" => Barboach,
            "Beldum" => Beldum,
            "Bellsprout" => Bellsprout,
            "Bulbasaur" => Bulbasaur,
            "Cacnea" => Cacnea,
            "Carvanha" => Carvanha,
            "Castform" => Castform,
            "Caterpie" => Caterpie,
            "Chansey" => Chansey,
            "Charmander" => Charmander,
            "Chikorita" => Chikorita,
            "Chimecho" => Chimecho,
            "Chinchou" => Chinchou,
            "Clamperl" => Clamperl,
            "Cleffa" => Cleffa,
            "Corphish" => Corphish,
            "Corsola" => Corsola,
            "Cubone" => Cubone,
            "Cyndaquil" => Cyndaquil,
            "Delibird" => Delibird,
            "Deoxys" => Deoxys,
            "Diglett" => Diglett,
            "Doduo" => Doduo,
            "Dratini" => Dratini,
            "Drowzee" => Drowzee,
            "Dunsparce" => Dunsparce,
            "Duskull" => Duskull,
            "Eevee" => Eevee,
            "Ekans" => Ekans,
            "Electrike" => Electrike,
            "Electrode" => Electrode,
            "Elekid" => Elekid,
            "Exeggcute" => Exeggcute,
            "Farfetch'd" => Farfetchd,
            "Feebas" => Feebas,
            "Gastly" => Gastly,
            "Geodude" => Geodude,
            "Girafarig" => Girafarig,
            "Gligar" => Gligar,
            "Goldeen" => Goldeen,
            "Grimer" => Grimer,
            "Groudon" => Groudon,
            "Growlithe" => Growlithe,
            "Gulpin" => Gulpin,
            "Heracross" => Heracross,
            "Ho-Oh" => HoOh,
            "Hoothoot" => Hoothoot,
            "Hoppip" => Hoppip,
            "Horsea" => Horsea,
            "Houndour" => Houndour,
            "Igglybuff" => Igglybuff,
            "Illumise" => Illumise,
            "Kabuto" => Kabuto,
            "Kangaskhan" => Kangaskhan,
            "Kecleon" => Kecleon,
            "Koffing" => Koffing,
            "Krabby" => Krabby,
            "Kyogre" => Kyogre,
            "Lapras" => Lapras,
            "Larvitar" => Larvitar,
            "Latias" => Latias,
            "Latios" => Latios,
            "Ledyba" => Ledyba,
            "Lickitung" => Lickitung,
            "Lileep" => Lileep,
            "Lotad" => Lotad,
            "Lugia" => Lugia,
            "Lunatone" => Lunatone,
            "Luvdisc" => Luvdisc,
            "Machop" => Machop,
            "Magby" => Magby,
            "Magikarp" => Magikarp,
            "Magnemite" => Magnemite,
            "Makuhita" => Makuhita,
            "Mankey" => Mankey,
            "Mantine" => Mantine,
            "Mareep" => Mareep,
            "Marill" => Marill,
            "Mawile" => Mawile,
            "Meditite" => Meditite,
            "Meowth" => Meowth,
            "Mew" => Mew,
            "Miltank" => Miltank,
            "Minun" => Minun,
            "Misdreavus" => Misdreavus,
            "Mr. Mime" => MrMime,
            "Mudkip" => Mudkip,
            "Murkrow" => Murkrow,
            "Natu" => Natu,
            "Nidoran♀" => NidoranFemale,
            "Nidoran♂" => NidoranMale,
            "Nincada" => Nincada,
            "Nosepass" => Nosepass,
            "Numel" => Numel,
            "Oddish" => Oddish,
            "Omanyte" => Omanyte,
            "Onix" => Onix,
            "Paras" => Paras,
            "Phanpy" => Phanpy,
            "Pichu" => Pichu,
            "Pidgey" => Pidgey,
            "Pineco" => Pineco,
            "Pinsir" => Pinsir,
            "Plusle" => Plusle,
            "Poliwag" => Poliwag,
            "Ponyta" => Ponyta,
            "Poochyena" => Poochyena,
            "Porygon" => Porygon,
            "Psyduck" => Psyduck,
            "Qwilfish" => Qwilfish,
            "Ralts" => Ralts,
            "Rattata" => Rattata,
            "Rayquaza" => Rayquaza,
            "Regice" => Regice,
            "Regirock" => Regirock,
            "Registeel" => Registeel,
            "Relicanth" => Relicanth,
            "Remoraid" => Remoraid,
            "Rhyhorn" => Rhyhorn,
            "Roselia" => Roselia,
            "Sableye" => Sableye,
            "Sandshrew" => Sandshrew,
            "Scyther" => Scyther,
            "Seedot" => Seedot,
            "Seel" => Seel,
            "Sentret" => Sentret,
            "Seviper" => Seviper,
            "Shellder" => Shellder,
            "Shroomish" => Shroomish,
            "Shuckle" => Shuckle,
            "Shuppet" => Shuppet,
            "Skarmory" => Skarmory,
            "Skitty" => Skitty,
            "Slakoth" => Slakoth,
            "Slowpoke" => Slowpoke,
            "Slugma" => Slugma,
            "Smeargle" => Smeargle,
            "Smoochum" => Smoochum,
            "Sneasel" => Sneasel,
            "Snorlax" => Snorlax,
            "Snorunt" => Snorunt,
            "Snubbull" => Snubbull,
            "Solrock" => Solrock,
            "Spearow" => Spearow,
            "Spheal" => Spheal,
            "Spinarak" => Spinarak,
            "Spinda" => Spinda,
            "Spoink" => Spoink,
            "Squirtle" => Squirtle,
            "Stantler" => Stantler,
            "Staryu" => Staryu,
            "Sudowoodo" => Sudowoodo,
            "Sunkern" => Sunkern,
            "Surskit" => Surskit,
            "Swablu" => Swablu,
            "Swinub" => Swinub,
            "Taillow" => Taillow,
            "Tangela" => Tangela,
            "Tauros" => Tauros,
            "Teddiursa" => Teddiursa,
            "Tentacool" => Tentacool,
            "Togepi" => Togepi,
            "Torchic" => Torchic,
            "Torkoal" => Torkoal,
            "Totodile" => Totodile,
            "Trapinch" => Trapinch,
            "Treecko" => Treecko,
            "Tropius" => Tropius,
            "Tyrogue" => Tyrogue,
            "Venonat" => Venonat,
            "Volbeat" => Volbeat,
            "Voltorb" => Voltorb,
            "Vulpix" => Vulpix,
            "Wailmer" => Wailmer,
            "Weedle" => Weedle,
            "Whismur" => Whismur,
            "Wingull" => Wingull,
            "Wobbuffet" => Wobbuffet,
            "Wooper" => Wooper,
            "Wurmple" => Wurmple,
            "Wynaut" => Wynaut,
            "Yanma" => Yanma,
            "Zangoose" => Zangoose,
            "Zigzagoon" => Zigzagoon,
            "Zubat" => Zubat,
            _ => unreachable!(),
        }
    }

    fn get_path_to_sprite(species: Species) -> PathBuf {
        let mut path;
        if cfg!(debug_assertions) {
            path = std::env::current_dir().unwrap();
            path.push("res");
        } else {
            path = std::env::current_exe().unwrap();
            path.pop();
            path.pop();
            path.push("share");
        }
        path.push("sprites");
        if species == Species::MrMime {
            path.push("Mr Mime")
        } else {
            path.push(species.to_str());
        }
        path.set_extension("png");
        path
    }
}
