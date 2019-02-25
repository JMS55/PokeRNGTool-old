#[derive(PartialEq, Copy, Clone)]
pub enum Species {
    Anorith,
    Beldum,
    Castform,
    Chikorita,
    Cyndaquil,
    Deoxys,
    Electrode,
    Groudon,
    HoOh,
    Kecleon,
    Kyogre,
    Latias,
    Latios,
    Lileep,
    Lugia,
    Mew,
    Mudkip,
    Rayquaza,
    Regice,
    Regirock,
    Registeel,
    Sudowoodo,
    Torchic,
    Totodile,
    Treecko,
    Voltorb,
    Wynaut,
}

impl Species {
    pub fn to_str(&self) -> &'static str {
        use Species::*;
        match self {
            Anorith => "Anorith",
            Beldum => "Beldum",
            Castform => "Castform",
            Chikorita => "Chikorita",
            Cyndaquil => "Cyndaquil",
            Deoxys => "Deoxys",
            Electrode => "Electrode",
            Groudon => "Groudon",
            HoOh => "Ho-Oh",
            Kecleon => "Kecleon",
            Kyogre => "Kyogre",
            Latias => "Latias",
            Latios => "Latios",
            Lileep => "Lileep",
            Lugia => "Lugia",
            Mew => "Mew",
            Mudkip => "Mudkip",
            Rayquaza => "Rayquaza",
            Regice => "Regice",
            Regirock => "Regirock",
            Registeel => "Registeel",
            Sudowoodo => "Sudowoodo",
            Torchic => "Torchic",
            Totodile => "Totodile",
            Treecko => "Treecko",
            Voltorb => "Voltorb",
            Wynaut => "Wynaut",
        }
    }

    pub fn get_abilities(&self) -> (Ability, Option<Ability>) {
        use Ability::*;
        use Species::*;
        match self {
            Anorith => (BattleArmor, None),
            Beldum => (ClearBody, None),
            Castform => (Forecast, None),
            Chikorita => (Overgrow, None),
            Cyndaquil => (Blaze, None),
            Deoxys => (Pressure, None),
            Electrode => (Soundproof, Some(Static)),
            Groudon => (Drought, None),
            HoOh => (Pressure, None),
            Kecleon => (ColorChange, None),
            Kyogre => (Drizzle, None),
            Latias => (Levitate, None),
            Latios => (Levitate, None),
            Lileep => (SuctionCups, None),
            Lugia => (Pressure, None),
            Mew => (Synchronize, None),
            Mudkip => (Torrent, None),
            Rayquaza => (AirLock, None),
            Regice => (ClearBody, None),
            Regirock => (ClearBody, None),
            Registeel => (ClearBody, None),
            Sudowoodo => (RockHead, Some(Sturdy)),
            Torchic => (Blaze, None),
            Totodile => (Torrent, None),
            Treecko => (Overgrow, None),
            Voltorb => (Soundproof, Some(Static)),
            Wynaut => (ShadowTag, None),
        }
    }

    pub fn get_gender_ratio(&self) -> GenderRatio {
        use GenderRatio::*;
        use Species::*;
        match self {
            Anorith => Eightysevenpointfive_Twelvepointfive,
            Beldum => Genderless,
            Castform => Fifty_Fifty,
            Chikorita => Eightysevenpointfive_Twelvepointfive,
            Cyndaquil => Eightysevenpointfive_Twelvepointfive,
            Deoxys => Genderless,
            Electrode => Genderless,
            Groudon => Genderless,
            HoOh => Genderless,
            Kecleon => Fifty_Fifty,
            Kyogre => Genderless,
            Latias => AlwaysFemale,
            Latios => AlwaysMale,
            Lileep => Eightysevenpointfive_Twelvepointfive,
            Lugia => Genderless,
            Mew => Genderless,
            Mudkip => Eightysevenpointfive_Twelvepointfive,
            Rayquaza => Genderless,
            Regice => Genderless,
            Regirock => Genderless,
            Registeel => Genderless,
            Sudowoodo => Fifty_Fifty,
            Torchic => Eightysevenpointfive_Twelvepointfive,
            Totodile => Eightysevenpointfive_Twelvepointfive,
            Treecko => Eightysevenpointfive_Twelvepointfive,
            Voltorb => Genderless,
            Wynaut => Fifty_Fifty,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Nature {
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

impl Nature {
    pub fn to_str(&self) -> &'static str {
        use Nature::*;
        match self {
            Adamant => "Adamant",
            Bashful => "Bashful",
            Bold => "Bold",
            Brave => "Brave",
            Calm => "Calm",
            Careful => "Careful",
            Docile => "Docile",
            Gentle => "Gentle",
            Hardy => "Hardy",
            Hasty => "Hasty",
            Impish => "Impish",
            Jolly => "Jolly",
            Lax => "Lax",
            Lonely => "Lonely",
            Mild => "Mild",
            Modest => "Modest",
            Naive => "Naive",
            Naughty => "Naughty",
            Quiet => "Quiet",
            Quirky => "Quirky",
            Rash => "Rash",
            Relaxed => "Relaxed",
            Sassy => "Sassy",
            Serious => "Serious",
            Timid => "Timid",
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Ability {
    AirLock,
    BattleArmor,
    Blaze,
    ClearBody,
    ColorChange,
    Drizzle,
    Drought,
    Forecast,
    Levitate,
    Overgrow,
    Pressure,
    RockHead,
    ShadowTag,
    Soundproof,
    Static,
    Sturdy,
    SuctionCups,
    Synchronize,
    Torrent,
}

impl Ability {
    pub fn to_str(&self) -> &'static str {
        use Ability::*;
        match self {
            AirLock => "Air Lock",
            BattleArmor => "Battle Armor",
            Blaze => "Blaze",
            ClearBody => "Clear Body",
            ColorChange => "Color Change",
            Drizzle => "Drizzle",
            Drought => "Drought",
            Forecast => "Forecast",
            Levitate => "Levitate",
            Overgrow => "Overgrow",
            Pressure => "Pressure",
            RockHead => "Rock Head",
            ShadowTag => "Shadow Tag",
            Soundproof => "Soundproof",
            Static => "Static",
            Sturdy => "Sturdy",
            SuctionCups => "Suction Cups",
            Synchronize => "Synchronize",
            Torrent => "Torrent",
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Gender {
    Genderless,
    Male,
    Female,
}

impl Gender {
    pub fn to_str(&self) -> &'static str {
        use Gender::*;
        match self {
            Genderless => "Genderless",
            Male => "Male",
            Female => "Female",
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
pub enum GenderRatio {
    Genderless,
    AlwaysMale,
    AlwaysFemale,
    // Male:Female % ratio
    Twentyfive_Seventyfive,
    Fifty_Fifty,
    Seventyfive_Twentyfive,
    Eightysevenpointfive_Twelvepointfive,
}

pub enum IVComparator {
    Any,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl IVComparator {
    pub fn test(&self, test_iv: u8, filter_iv: u8) -> bool {
        use IVComparator::*;
        match self {
            Any => true,
            Equal => test_iv == filter_iv,
            GreaterThanOrEqual => test_iv >= filter_iv,
            LessThanOrEqual => test_iv <= filter_iv,
        }
    }
}
