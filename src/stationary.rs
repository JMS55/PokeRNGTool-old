use crate::lcrng::LCRNG;
use crate::misc::Species::*;
use crate::misc::{Ability, Gender, GenderRatio, IVComparator, Nature, Species};
use crate::widgets::{
    AbilityInput, DelayInput, FrameInput, GenderInput, IDInput, IVWidgets, Label, NatureInput,
    SeedInput, ShinyOnlyInput, SpeciesInput,
};
use gdk::SELECTION_CLIPBOARD;
use gtk::{
    Button, ButtonExt, CellLayoutExt, CellRendererExt, CellRendererText, Clipboard, ContainerExt,
    Grid, GridExt, GtkListStoreExtManual, GtkMenuExt, Inhibit, ListStore, Menu, MenuItem,
    MenuShellExt, Orientation, ScrolledWindow, ScrolledWindowExt, ShadowType, Stack, StackExt,
    StaticType, TreeModelExt, TreeSelectionExt, TreeView, TreeViewColumn, TreeViewColumnExt,
    TreeViewExt, TreeViewGridLines, WidgetExt,
};

pub fn ui(search_button: &Button, reset_button: &Button, stack: &Stack) -> gtk::Box {
    let seed_input = SeedInput::new();
    let max_frame_input = FrameInput::new();
    let delay_input = DelayInput::new();
    let tid_input = IDInput::new();
    let sid_input = IDInput::new();
    let shiny_only_input = ShinyOnlyInput::new();
    let species_input = SpeciesInput::new(vec![
        Anorith, Beldum, Castform, Chikorita, Cyndaquil, Deoxys, Electrode, Groudon, HoOh, Kecleon,
        Kyogre, Latias, Latios, Lileep, Lugia, Mew, Mudkip, Rayquaza, Regice, Regirock, Registeel,
        Sudowoodo, Torchic, Totodile, Treecko, Voltorb, Wynaut,
    ]);
    let nature_input = NatureInput::new();
    let ability_input = AbilityInput::new(&species_input);
    let gender_input = GenderInput::new(&species_input);
    let hp_iv_widgets = IVWidgets::new("HP");
    let attack_iv_widgets = IVWidgets::new("Atk");
    let defense_iv_widgets = IVWidgets::new("Def");
    let special_attack_iv_widgets = IVWidgets::new("SpA");
    let special_defense_iv_widgets = IVWidgets::new("SpD");
    let speed_iv_widgets = IVWidgets::new("Spe");
    let results_table = ResultsTable::new();
    let seed_label = Label::new("Seed");
    let max_frame_label = Label::new("Max Frame");
    let delay_label = Label::new("Delay");
    let tid_label = Label::new("TID");
    let sid_label = Label::new("SID");
    let species_label = Label::new("Species");
    let nature_label = Label::new("Nature");
    let ability_label = Label::new("Ability");
    let gender_label = Label::new("Gender");

    search_button.connect_clicked({
        let stack = stack.clone();
        let seed_input = seed_input.clone();
        let max_frame_input = max_frame_input.clone();
        let delay_input = delay_input.clone();
        let tid_input = tid_input.clone();
        let sid_input = sid_input.clone();
        let shiny_only_input = shiny_only_input.clone();
        let species_input = species_input.clone();
        let nature_input = nature_input.clone();
        let ability_input = ability_input.clone();
        let gender_input = gender_input.clone();
        let hp_iv_widgets = hp_iv_widgets.clone();
        let attack_iv_widgets = attack_iv_widgets.clone();
        let defense_iv_widgets = defense_iv_widgets.clone();
        let special_attack_iv_widgets = special_attack_iv_widgets.clone();
        let special_defense_iv_widgets = special_defense_iv_widgets.clone();
        let speed_iv_widgets = speed_iv_widgets.clone();
        let results_table = results_table.clone();
        move |_| {
            if stack.get_visible_child_name().unwrap() == "stationary" {
                let matching_frames = Frame::generate(
                    seed_input.get_seed(),
                    max_frame_input.get_max_frame(),
                    delay_input.get_delay(),
                    species_input.get_current_species(),
                )
                .into_iter()
                .filter(|frame| {
                    frame.passes_filter(
                        shiny_only_input.get_shiny_only(),
                        tid_input.get_id(),
                        sid_input.get_id(),
                        nature_input.get_nature(),
                        ability_input.get_ability(),
                        gender_input.get_gender(),
                        hp_iv_widgets.get_filters().0,
                        attack_iv_widgets.get_filters().0,
                        defense_iv_widgets.get_filters().0,
                        special_attack_iv_widgets.get_filters().0,
                        special_defense_iv_widgets.get_filters().0,
                        speed_iv_widgets.get_filters().0,
                        hp_iv_widgets.get_filters().1,
                        attack_iv_widgets.get_filters().1,
                        defense_iv_widgets.get_filters().1,
                        special_attack_iv_widgets.get_filters().1,
                        special_defense_iv_widgets.get_filters().1,
                        speed_iv_widgets.get_filters().1,
                    )
                })
                .collect();
                results_table.set_frames(matching_frames, tid_input.get_id(), sid_input.get_id());
            }
        }
    });

    reset_button.connect_clicked({
        let stack = stack.clone();
        let seed_input = seed_input.clone();
        let max_frame_input = max_frame_input.clone();
        let delay_input = delay_input.clone();
        let tid_input = tid_input.clone();
        let sid_input = sid_input.clone();
        let shiny_only_input = shiny_only_input.clone();
        let species_input = species_input.clone();
        let nature_input = nature_input.clone();
        let ability_input = ability_input.clone();
        let gender_input = gender_input.clone();
        let hp_iv_widgets = hp_iv_widgets.clone();
        let attack_iv_widgets = attack_iv_widgets.clone();
        let defense_iv_widgets = defense_iv_widgets.clone();
        let special_attack_iv_widgets = special_attack_iv_widgets.clone();
        let special_defense_iv_widgets = special_defense_iv_widgets.clone();
        let speed_iv_widgets = speed_iv_widgets.clone();
        let results_table = results_table.clone();
        move |_| {
            if stack.get_visible_child_name().unwrap() == "stationary" {
                seed_input.reset();
                max_frame_input.reset();
                delay_input.reset();
                tid_input.reset();
                sid_input.reset();
                shiny_only_input.reset();
                species_input.reset();
                nature_input.reset();
                ability_input.reset();
                gender_input.reset();
                hp_iv_widgets.reset();
                attack_iv_widgets.reset();
                defense_iv_widgets.reset();
                special_attack_iv_widgets.reset();
                special_defense_iv_widgets.reset();
                speed_iv_widgets.reset();
                results_table.reset();
            }
        }
    });

    delay_input.widget().set_hexpand(true);
    shiny_only_input.widget().set_hexpand(true);
    gender_input.widget().set_hexpand(true);
    speed_iv_widgets.spinner_widget().set_hexpand(true);
    results_table.widget().set_hexpand(true);
    results_table.widget().set_vexpand(true);

    let grid = Grid::new();
    grid.set_row_spacing(6);
    grid.set_column_spacing(12);
    grid.attach(seed_label.widget(), 0, 0, 1, 1);
    grid.attach(max_frame_label.widget(), 0, 1, 1, 1);
    grid.attach(delay_label.widget(), 0, 2, 1, 1);
    grid.attach(seed_input.widget(), 1, 0, 1, 1);
    grid.attach(max_frame_input.widget(), 1, 1, 1, 1);
    grid.attach(delay_input.widget(), 1, 2, 1, 1);
    grid.attach(tid_label.widget(), 2, 0, 1, 1);
    grid.attach(sid_label.widget(), 2, 1, 1, 1);
    grid.attach(tid_input.widget(), 3, 0, 1, 1);
    grid.attach(sid_input.widget(), 3, 1, 1, 1);
    grid.attach(shiny_only_input.widget(), 3, 2, 1, 1);
    grid.attach(species_label.widget(), 4, 0, 1, 1);
    grid.attach(nature_label.widget(), 4, 1, 1, 1);
    grid.attach(ability_label.widget(), 4, 2, 1, 1);
    grid.attach(gender_label.widget(), 4, 3, 1, 1);
    grid.attach(species_input.widget(), 5, 0, 1, 1);
    grid.attach(nature_input.widget(), 5, 1, 1, 1);
    grid.attach(ability_input.widget(), 5, 2, 1, 1);
    grid.attach(gender_input.widget(), 5, 3, 1, 1);
    grid.attach(hp_iv_widgets.label_widget(), 6, 0, 1, 1);
    grid.attach(attack_iv_widgets.label_widget(), 6, 1, 1, 1);
    grid.attach(defense_iv_widgets.label_widget(), 6, 2, 1, 1);
    grid.attach(special_attack_iv_widgets.label_widget(), 6, 3, 1, 1);
    grid.attach(special_defense_iv_widgets.label_widget(), 6, 4, 1, 1);
    grid.attach(speed_iv_widgets.label_widget(), 6, 5, 1, 1);
    grid.attach(hp_iv_widgets.comparator_widget(), 7, 0, 1, 1);
    grid.attach(attack_iv_widgets.comparator_widget(), 7, 1, 1, 1);
    grid.attach(defense_iv_widgets.comparator_widget(), 7, 2, 1, 1);
    grid.attach(special_attack_iv_widgets.comparator_widget(), 7, 3, 1, 1);
    grid.attach(special_defense_iv_widgets.comparator_widget(), 7, 4, 1, 1);
    grid.attach(speed_iv_widgets.comparator_widget(), 7, 5, 1, 1);
    grid.attach(hp_iv_widgets.spinner_widget(), 8, 0, 1, 1);
    grid.attach(attack_iv_widgets.spinner_widget(), 8, 1, 1, 1);
    grid.attach(defense_iv_widgets.spinner_widget(), 8, 2, 1, 1);
    grid.attach(special_attack_iv_widgets.spinner_widget(), 8, 3, 1, 1);
    grid.attach(special_defense_iv_widgets.spinner_widget(), 8, 4, 1, 1);
    grid.attach(speed_iv_widgets.spinner_widget(), 8, 5, 1, 1);

    let vbox = gtk::Box::new(Orientation::Vertical, 12);
    vbox.add(&grid);
    vbox.add(results_table.widget());
    vbox
}

pub struct Frame {
    pub species: Species,
    pub number: u32,
    pub pid: u32,
    pub hp_iv: u8,
    pub attack_iv: u8,
    pub defense_iv: u8,
    pub special_attack_iv: u8,
    pub special_defense_iv: u8,
    pub speed_iv: u8,
}

impl Frame {
    pub fn generate(seed: u32, max_frame: u32, delay: u32, species: Species) -> Vec<Frame> {
        let mut lcrng = LCRNG::new_emerald(seed);
        let mut rng = [
            lcrng.next_u16(),
            lcrng.next_u16(),
            lcrng.next_u16(),
            lcrng.next_u16(),
        ];
        let mut results = Vec::with_capacity(max_frame as usize + 1);
        for frame_number in 0..=max_frame {
            if frame_number.wrapping_sub(delay) > max_frame {
                continue;
            }
            let frame = Frame {
                species,
                number: frame_number.wrapping_sub(delay),
                pid: ((rng[1] as u32) << 16) | rng[0] as u32,
                hp_iv: (rng[2] & 0x1F) as u8,
                attack_iv: ((rng[2] & 0x3FF) >> 5) as u8,
                defense_iv: ((rng[2] & 0x7FFF) >> 10) as u8,
                special_attack_iv: ((rng[3] & 0x3FF) >> 5) as u8,
                special_defense_iv: ((rng[3] & 0x7FFF) >> 10) as u8,
                speed_iv: (rng[3] & 0x1F) as u8,
            };
            results.push(frame);
            rng = [rng[1], rng[2], rng[3], lcrng.next_u16()];
        }
        results
    }

    pub fn is_shiny(&self, tid: u16, sid: u16) -> bool {
        let hid = (self.pid >> 16) as u16;
        let lid = (self.pid & 0xFFFF) as u16;
        (tid ^ sid ^ hid ^ lid) < 8
    }

    pub fn get_nature(&self) -> Nature {
        use Nature::*;
        match self.pid % 25 {
            0 => Hardy,
            1 => Lonely,
            2 => Brave,
            3 => Adamant,
            4 => Naughty,
            5 => Bold,
            6 => Docile,
            7 => Relaxed,
            8 => Impish,
            9 => Lax,
            10 => Timid,
            11 => Hasty,
            12 => Serious,
            13 => Jolly,
            14 => Naive,
            15 => Modest,
            16 => Mild,
            17 => Quiet,
            18 => Bashful,
            19 => Rash,
            20 => Calm,
            21 => Gentle,
            22 => Sassy,
            23 => Careful,
            24 => Quirky,
            _ => unreachable!(),
        }
    }

    pub fn get_ability(&self) -> Ability {
        match self.species.get_abilities() {
            (a, None) => a,
            (a, Some(_)) if (0x1 & self.pid) == 0 => a,
            (_, Some(b)) if (0x1 & self.pid) == 1 => b,
            _ => unreachable!(),
        }
    }

    pub fn get_gender(&self) -> Gender {
        use Gender::*;
        use GenderRatio::*;
        let last_two_digits = self.pid & 0xFF;
        match self.species.get_gender_ratio() {
            GenderRatio::Genderless => Gender::Genderless,
            AlwaysMale => Male,
            AlwaysFemale => Female,
            Twentyfive_Seventyfive => {
                if last_two_digits <= 190 {
                    Female
                } else {
                    Male
                }
            }
            Fifty_Fifty => {
                if last_two_digits <= 126 {
                    Female
                } else {
                    Male
                }
            }
            Seventyfive_Twentyfive => {
                if last_two_digits <= 63 {
                    Female
                } else {
                    Male
                }
            }
            Eightysevenpointfive_Twelvepointfive => {
                if last_two_digits <= 30 {
                    Female
                } else {
                    Male
                }
            }
        }
    }

    pub fn passes_filter(
        &self,
        shiny_only: bool,
        tid: u16,
        sid: u16,
        nature: Option<Nature>,
        ability: Option<Ability>,
        gender: Option<Gender>,
        hp_iv_comparator: IVComparator,
        attack_iv_comparator: IVComparator,
        defense_iv_comparator: IVComparator,
        special_attack_iv_comparator: IVComparator,
        special_defense_iv_comparator: IVComparator,
        speed_iv_comparator: IVComparator,
        hp_iv: u8,
        attack_iv: u8,
        defense_iv: u8,
        special_attack_iv: u8,
        special_defense_iv: u8,
        speed_iv: u8,
    ) -> bool {
        if shiny_only {
            if !self.is_shiny(tid, sid) {
                return false;
            }
        }
        if let Some(nature) = nature {
            if self.get_nature() != nature {
                return false;
            }
        }
        if let Some(ability) = ability {
            if self.get_ability() != ability {
                return false;
            }
        }
        if let Some(gender) = gender {
            if self.get_gender() != gender {
                return false;
            }
        }
        if !hp_iv_comparator.test(self.hp_iv, hp_iv) {
            return false;
        }
        if !attack_iv_comparator.test(self.attack_iv, attack_iv) {
            return false;
        }
        if !defense_iv_comparator.test(self.defense_iv, defense_iv) {
            return false;
        }
        if !special_attack_iv_comparator.test(self.special_attack_iv, special_attack_iv) {
            return false;
        }
        if !special_defense_iv_comparator.test(self.special_defense_iv, special_defense_iv) {
            return false;
        }
        if !speed_iv_comparator.test(self.speed_iv, speed_iv) {
            return false;
        }
        true
    }
}

#[derive(Clone)]
pub struct ResultsTable {
    table: TreeView,
    scrolled_window: ScrolledWindow,
}

impl ResultsTable {
    pub fn new() -> Self {
        let table = TreeView::new_with_model(&Self::create_model());
        table.set_grid_lines(TreeViewGridLines::Both);
        table.set_headers_clickable(false);
        table.set_show_expanders(false);
        table.set_enable_search(false);
        table.connect_button_press_event(|table, event| {
            if event.get_button() == 3 {
                if let Some((result_model, model_index)) = table.get_selection().get_selected() {
                    let pid_menu_item = MenuItem::new_with_label("Copy PID");
                    pid_menu_item.connect_button_press_event(move |_, _| {
                        let pid = result_model.get_value(&model_index, 1);
                        Clipboard::get(&SELECTION_CLIPBOARD).set_text(&pid.get::<&str>().unwrap());
                        Inhibit(false)
                    });
                    let pid_copy_menu = Menu::new();
                    pid_copy_menu.append(&pid_menu_item);
                    pid_copy_menu.show_all();
                    pid_copy_menu.popup_at_pointer(None);
                }
            }
            Inhibit(false)
        });
        let text_renderer = CellRendererText::new();
        text_renderer.set_alignment(0.5, 0.5);
        let titles = [
            "Frame Number",
            "PID",
            "Shiny",
            "HP",
            "Atk",
            "Def",
            "SpAtk",
            "SpDef",
            "Speed",
            "Nature",
            "Ability",
            "Gender",
        ];
        for (i, h) in titles.iter().enumerate() {
            let column = TreeViewColumn::new();
            column.set_title(h);
            column.set_alignment(0.5);
            column.set_expand(true);
            column.pack_start(&text_renderer, false);
            column.add_attribute(&text_renderer, "text", i as i32);
            table.append_column(&column);
        }
        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.set_shadow_type(ShadowType::In);
        scrolled_window.add(&table);
        Self {
            table,
            scrolled_window,
        }
    }

    pub fn set_frames(&self, frames: Vec<Frame>, tid: u16, sid: u16) {
        let model = Self::create_model();
        for frame in frames {
            let shiny_text = match frame.is_shiny(tid, sid) {
                false => "",
                true => "★",
            };
            model.insert_with_values(
                None,
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                &[
                    &frame.number,
                    &format!("{:X}", frame.pid),
                    &shiny_text,
                    &frame.hp_iv,
                    &frame.attack_iv,
                    &frame.defense_iv,
                    &frame.special_attack_iv,
                    &frame.special_defense_iv,
                    &frame.speed_iv,
                    &frame.get_nature().to_str(),
                    &frame.get_ability().to_str(),
                    &frame.get_gender().to_str(),
                ],
            );
            self.table.set_model(&model);
        }
    }

    pub fn reset(&self) {
        self.table.set_model(&Self::create_model());
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.scrolled_window
    }

    fn create_model() -> ListStore {
        ListStore::new(&[
            u32::static_type(), // Frame Number
            str::static_type(), // PID
            str::static_type(), // Shiny
            u8::static_type(),  // HP IV
            u8::static_type(),  // Attack IV
            u8::static_type(),  // Defense IV
            u8::static_type(),  // Special Attack IV
            u8::static_type(),  // Special Defense IV
            u8::static_type(),  // Speed IV
            str::static_type(), // Nature
            str::static_type(), // Ability
            str::static_type(), // Gender
        ])
    }
}
