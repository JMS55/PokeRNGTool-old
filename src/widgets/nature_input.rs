use crate::misc::Nature;
use glib::object::Cast;
use gtk::{
    BoxExt, ButtonExt, ContainerExt, EntryExt, GtkWindowExt, IconSize, Image, Label, LabelExt,
    ListBox, ListBoxExt, ListBoxRow, MenuButton, MenuButtonExt, Orientation, Popover, PopoverExt,
    ScrolledWindow, ScrolledWindowExt, SearchEntry, SearchEntryExt, ShadowType, WidgetExt, Window,
};

#[derive(Clone)]
pub struct NatureInput {
    widget: MenuButton,
    listbox: ListBox,
}

impl NatureInput {
    pub fn new() -> Self {
        let widget = MenuButton::new();
        let popover = Popover::new(&widget);
        let vbox = gtk::Box::new(Orientation::Vertical, 6);
        let search_entry = SearchEntry::new();
        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let listbox = ListBox::new();

        let hbox = gtk::Box::new(Orientation::Horizontal, 3);
        let label = Label::new("Any");
        let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
        hbox.add(&label);
        hbox.pack_end(&icon, false, false, 0);
        widget.add(&hbox);

        for nature in &[
            "Any", "Adamant", "Bashful", "Bold", "Brave", "Calm", "Careful", "Docile", "Gentle",
            "Hardy", "Hasty", "Impish", "Jolly", "Lax", "Lonely", "Mild", "Modest", "Naive",
            "Naughty", "Quiet", "Quirky", "Rash", "Relaxed", "Sassy", "Serious", "Timid",
        ] {
            let label = Label::new(*nature);
            listbox.add(&label);
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
                let nature = row.clone().get_children()[0]
                    .clone()
                    .downcast::<Label>()
                    .unwrap()
                    .get_text()
                    .unwrap();
                let hbox = gtk::Box::new(Orientation::Horizontal, 3);
                let label = Label::new(nature.as_str());
                let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
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

    pub fn get_nature(&self) -> Option<Nature> {
        use Nature::*;
        match self.widget.get_children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .get_children()[0]
            .clone()
            .downcast::<Label>()
            .unwrap()
            .get_text()
            .unwrap()
            .as_str()
        {
            "Any" => None,
            "Adamant" => Some(Adamant),
            "Bashful" => Some(Bashful),
            "Bold" => Some(Bold),
            "Brave" => Some(Brave),
            "Calm" => Some(Calm),
            "Careful" => Some(Careful),
            "Docile" => Some(Docile),
            "Gentle" => Some(Gentle),
            "Hardy" => Some(Hardy),
            "Hasty" => Some(Hasty),
            "Impish" => Some(Impish),
            "Jolly" => Some(Jolly),
            "Lax" => Some(Lax),
            "Lonely" => Some(Lonely),
            "Mild" => Some(Mild),
            "Modest" => Some(Modest),
            "Naive" => Some(Naive),
            "Naughty" => Some(Naughty),
            "Quiet" => Some(Quiet),
            "Quirky" => Some(Quirky),
            "Rash" => Some(Rash),
            "Relaxed" => Some(Relaxed),
            "Sassy" => Some(Sassy),
            "Serious" => Some(Serious),
            "Timid" => Some(Timid),
            _ => unreachable!(),
        }
    }

    pub fn reset(&self) {
        self.widget.get_children()[0].destroy();
        let nature = self.listbox.get_children()[0]
            .clone()
            .downcast::<ListBoxRow>()
            .unwrap()
            .get_children()[0]
            .clone()
            .downcast::<Label>()
            .unwrap()
            .get_text()
            .unwrap();
        let hbox = gtk::Box::new(Orientation::Horizontal, 3);
        let label = Label::new(nature.as_str());
        let icon = Image::new_from_icon_name("pan-down-symbolic", IconSize::Button);
        hbox.add(&label);
        hbox.pack_end(&icon, false, false, 0);
        hbox.show_all();
        self.widget.add(&hbox);
    }

    pub fn widget(&self) -> &MenuButton {
        &self.widget
    }
}
