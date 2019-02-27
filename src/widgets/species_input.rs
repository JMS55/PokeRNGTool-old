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
            "Anorith" => Anorith,
            "Beldum" => Beldum,
            "Castform" => Castform,
            "Chikorita" => Chikorita,
            "Cyndaquil" => Cyndaquil,
            "Deoxys" => Deoxys,
            "Electrode" => Electrode,
            "Groudon" => Groudon,
            "Ho-Oh" => HoOh,
            "Kecleon" => Kecleon,
            "Kyogre" => Kyogre,
            "Latias" => Latias,
            "Latios" => Latios,
            "Lileep" => Lileep,
            "Lugia" => Lugia,
            "Mew" => Mew,
            "Mudkip" => Mudkip,
            "Rayquaza" => Rayquaza,
            "Regice" => Regice,
            "Regirock" => Regirock,
            "Registeel" => Registeel,
            "Sudowoodo" => Sudowoodo,
            "Torchic" => Torchic,
            "Totodile" => Totodile,
            "Treecko" => Treecko,
            "Voltorb" => Voltorb,
            "Wynaut" => Wynaut,
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
        path.push(species.to_str());
        path.set_extension("png");
        path
    }
}
