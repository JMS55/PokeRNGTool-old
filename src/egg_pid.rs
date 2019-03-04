use crate::misc::Species::*;
use crate::widgets::{
    AbilityInput, CompatabilityInput, DifferenceInput, FrameInput, GenderInput, IDInput, Label,
    NatureInput, SeedInput, ShinyOnlyInput, SpeciesInput,
};
use gtk::{Button, ContainerExt, Grid, GridExt, Orientation, Stack, WidgetExt};

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
    // let results_table = ResultsTable::new();
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

    // search_button.connect_clicked({});

    // reset_button.connect_clicked({});

    compatability_input.widget().set_hexpand(true);
    shiny_only_input.widget().set_hexpand(true);
    gender_input.widget().set_hexpand(true);
    // results_table.widget().set_hexpand(true);
    // results_table.widget().set_vexpand(true);

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
    // vbox.add(results_table.widget());
    vbox
}
