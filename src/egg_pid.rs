use crate::lcrng::LCRNG;
use crate::misc::Species::*;
use crate::misc::{Ability, Gender, GenderRatio, Nature, Species};
use crate::widgets::{
    AbilityInput, CompatabilityInput, DifferenceInput, FrameInput, GenderInput, IDInput, Label,
    NatureInput, SeedInput, ShinyOnlyInput, SpeciesInput,
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
    let difference_input = DifferenceInput::new();
    let compatability_input = CompatabilityInput::new();
    let tid_input = IDInput::new();
    let sid_input = IDInput::new();
    let shiny_only_input = ShinyOnlyInput::new();
    let species_input = SpeciesInput::new(vec![
        Abra,
        Absol,
        Aerodactyl,
        Aipom,
        Anorith,
        Aron,
        Azurill,
        Bagon,
        Baltoy,
        Barboach,
        Beldum,
        Bellsprout,
        Bulbasaur,
        Cacnea,
        Carvanha,
        Castform,
        Caterpie,
        Chansey,
        Charmander,
        Chikorita,
        Chimecho,
        Chinchou,
        Clamperl,
        Cleffa,
        Corphish,
        Corsola,
        Cubone,
        Cyndaquil,
        Delibird,
        Diglett,
        Doduo,
        Dratini,
        Drowzee,
        Dunsparce,
        Duskull,
        Eevee,
        Ekans,
        Electrike,
        Elekid,
        Exeggcute,
        Farfetchd,
        Feebas,
        Gastly,
        Geodude,
        Girafarig,
        Gligar,
        Goldeen,
        Grimer,
        Growlithe,
        Gulpin,
        Heracross,
        Hoothoot,
        Hoppip,
        Horsea,
        Houndour,
        Igglybuff,
        Illumise,
        Kabuto,
        Kangaskhan,
        Kecleon,
        Koffing,
        Krabby,
        Lapras,
        Larvitar,
        Ledyba,
        Lickitung,
        Lileep,
        Lotad,
        Lunatone,
        Luvdisc,
        Machop,
        Magby,
        Magikarp,
        Magnemite,
        Makuhita,
        Mankey,
        Mantine,
        Mareep,
        Marill,
        Mawile,
        Meditite,
        Meowth,
        Miltank,
        Minun,
        Misdreavus,
        MrMime,
        Mudkip,
        Murkrow,
        Natu,
        NidoranFemale,
        NidoranMale,
        Nincada,
        Nosepass,
        Numel,
        Oddish,
        Omanyte,
        Onix,
        Paras,
        Phanpy,
        Pichu,
        Pidgey,
        Pineco,
        Pinsir,
        Plusle,
        Poliwag,
        Ponyta,
        Poochyena,
        Porygon,
        Psyduck,
        Qwilfish,
        Ralts,
        Rattata,
        Relicanth,
        Remoraid,
        Rhyhorn,
        Roselia,
        Sableye,
        Sandshrew,
        Scyther,
        Seedot,
        Seel,
        Sentret,
        Seviper,
        Shellder,
        Shroomish,
        Shuckle,
        Shuppet,
        Skarmory,
        Skitty,
        Slakoth,
        Slowpoke,
        Slugma,
        Smeargle,
        Smoochum,
        Sneasel,
        Snorlax,
        Snorunt,
        Snubbull,
        Solrock,
        Spearow,
        Spheal,
        Spinarak,
        Spinda,
        Spoink,
        Squirtle,
        Stantler,
        Staryu,
        Sudowoodo,
        Sunkern,
        Surskit,
        Swablu,
        Swinub,
        Taillow,
        Tangela,
        Tauros,
        Teddiursa,
        Tentacool,
        Togepi,
        Torchic,
        Torkoal,
        Totodile,
        Trapinch,
        Treecko,
        Tropius,
        Tyrogue,
        Venonat,
        Volbeat,
        Voltorb,
        Vulpix,
        Wailmer,
        Weedle,
        Whismur,
        Wingull,
        Wobbuffet,
        Wooper,
        Wurmple,
        Wynaut,
        Yanma,
        Zangoose,
        Zigzagoon,
        Zubat,
    ]);
    let nature_input = NatureInput::new();
    let ability_input = AbilityInput::new(&species_input);
    let gender_input = GenderInput::new(&species_input);
    let results_table = ResultsTable::new();
    let seed_label = Label::new("Seed");
    let max_frame_label = Label::new("Max Frame");
    let difference_label = Label::new("Difference");
    let compatability_label = Label::new("Compatability");
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
        let difference_input = difference_input.clone();
        let compatability_input = compatability_input.clone();
        let tid_input = tid_input.clone();
        let sid_input = sid_input.clone();
        let shiny_only_input = shiny_only_input.clone();
        let species_input = species_input.clone();
        let nature_input = nature_input.clone();
        let ability_input = ability_input.clone();
        let gender_input = gender_input.clone();
        let results_table = results_table.clone();
        move |_| {
            if stack.get_visible_child_name().unwrap() == "egg_pid" {
                let matching_frames = Frame::generate(
                    seed_input.get_seed(),
                    max_frame_input.get_max_frame(),
                    compatability_input.get_compatability(),
                    difference_input.get_difference(),
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
        let difference_input = difference_input.clone();
        let compatability_input = compatability_input.clone();
        let tid_input = tid_input.clone();
        let sid_input = sid_input.clone();
        let shiny_only_input = shiny_only_input.clone();
        let species_input = species_input.clone();
        let nature_input = nature_input.clone();
        let ability_input = ability_input.clone();
        let gender_input = gender_input.clone();
        let results_table = results_table.clone();
        move |_| {
            if stack.get_visible_child_name().unwrap() == "egg_pid" {
                seed_input.reset();
                max_frame_input.reset();
                difference_input.reset();
                compatability_input.reset();
                tid_input.reset();
                sid_input.reset();
                shiny_only_input.reset();
                species_input.reset();
                nature_input.reset();
                ability_input.reset();
                gender_input.reset();
                results_table.reset();
            }
        }
    });

    compatability_input.widget().set_hexpand(true);
    shiny_only_input.widget().set_hexpand(true);
    gender_input.widget().set_hexpand(true);
    results_table.widget().set_hexpand(true);
    results_table.widget().set_vexpand(true);

    let grid = Grid::new();
    grid.set_row_spacing(6);
    grid.set_column_spacing(12);
    grid.attach(seed_label.widget(), 0, 0, 1, 1);
    grid.attach(max_frame_label.widget(), 0, 1, 1, 1);
    grid.attach(difference_label.widget(), 0, 2, 1, 1);
    grid.attach(compatability_label.widget(), 0, 3, 1, 1);
    grid.attach(seed_input.widget(), 1, 0, 1, 1);
    grid.attach(max_frame_input.widget(), 1, 1, 1, 1);
    grid.attach(difference_input.widget(), 1, 2, 1, 1);
    grid.attach(compatability_input.widget(), 1, 3, 1, 1);
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

    let vbox = gtk::Box::new(Orientation::Vertical, 12);
    vbox.add(&grid);
    vbox.add(results_table.widget());
    vbox
}

pub struct Frame {
    pub species: Species,
    pub number: u32,
    pub menu_opens: u8,
    pub pid: u32,
}

impl Frame {
    pub fn generate(
        seed: u32,
        max_frame: u32,
        parent_compatability: u32,
        difference: u32,
        species: Species,
    ) -> Vec<Frame> {
        let mut results = Vec::with_capacity(max_frame as usize);
        let mut rng = LCRNG::new_emerald(seed);
        let mut rng_cache = [rng.next_u16(), rng.next_u16()];
        for frame_number in 0..=max_frame {
            for menu_opens in 0..=30 {
                if rng_cache[0] as u32 * 100 / 65535 >= parent_compatability {
                    // Egg not generated, skip this frame
                    continue;
                }
                let offset = difference + menu_opens * 3;
                let mut rng2 = LCRNG::new_emerald((frame_number.wrapping_sub(offset)) & 0xFFFF);
                let pid1 = (rng_cache[1] % 0xFFFE) + 1;
                let pid2 = rng2.next_u32() & 0xFFFF0000;
                let pid = pid1 as u32 | pid2;
                let frame = Frame {
                    species,
                    menu_opens: menu_opens as u8,
                    number: frame_number.wrapping_sub(offset).wrapping_sub(17),
                    pid,
                };
                results.push(frame);
            }
            rng_cache = [rng_cache[1], rng.next_u16()];
        }
        results.retain(|frame| frame.number <= max_frame);
        // Sort by Frame number and then Menu Opens
        results.sort_unstable_by(|frame1, frame2| {
            frame1
                .number
                .cmp(&frame2.number)
                .then_with(|| frame1.menu_opens.cmp(&frame2.menu_opens))
        });
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
        let gender_ratio = match self.species {
            NidoranFemale | NidoranMale | Illumise | Volbeat => Fifty_Fifty,
            _ => self.species.get_gender_ratio(),
        };
        match gender_ratio {
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
        if self.species == NidoranMale || self.species == Volbeat {
            if self.get_gender() != Gender::Male {
                return false;
            }
        }
        if self.species == NidoranFemale || self.species == Illumise {
            if self.get_gender() != Gender::Female {
                return false;
            }
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
                        let pid = result_model.get_value(&model_index, 2);
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
            "Menu Opens",
            "PID",
            "Shiny",
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
                &[0, 1, 2, 3, 4, 5, 6],
                &[
                    &frame.number,
                    &frame.menu_opens,
                    &format!("{:X}", frame.pid),
                    &shiny_text,
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
            u8::static_type(),  // Menu Opens
            str::static_type(), // PID
            str::static_type(), // Shiny
            str::static_type(), // Nature
            str::static_type(), // Ability
            str::static_type(), // Gender
        ])
    }
}
