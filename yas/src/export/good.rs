use crate::core::genshin::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, GenshinArtifact,
};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

struct GOODArtifact<'a> {
    artifact: &'a GenshinArtifact,
}

impl<'a> Serialize for GOODArtifact<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let artifact = &self.artifact;

        let mut substats = Vec::new();
        if let Some(stat) = &artifact.sub_stat_1 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_2 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_3 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_4 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }

        let mut root = serializer.serialize_map(Some(8))?;
        root.serialize_entry("setKey", artifact.set_name.to_good())?;
        root.serialize_entry("slotKey", artifact.slot.to_good())?;
        root.serialize_entry("level", &artifact.level)?;
        root.serialize_entry("rarity", &artifact.star)?;
        root.serialize_entry("mainStatKey", artifact.main_stat.name.to_good())?;
        root.serialize_entry("location", "")?;
        root.serialize_entry("lock", &false)?;
        root.serialize_entry("substats", &substats)?;
        root.end()
    }
}

#[derive(Serialize)]
struct GOODStat<'a> {
    key: &'a str,
    value: f64,
}

impl<'a> GOODStat<'a> {
    fn new(stat: &ArtifactStat) -> GOODStat {
        GOODStat {
            key: stat.name.to_good(),
            value: match stat.name {
                ArtifactStatName::Atk
                | ArtifactStatName::ElementalMastery
                | ArtifactStatName::Hp
                | ArtifactStatName::Def => stat.value,
                _ => stat.value * 100.0,
            },
        }
    }
}

impl ArtifactStatName {
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactStatName::HealingBonus => "heal_",
            ArtifactStatName::CriticalDamage => "critDMG_",
            ArtifactStatName::Critical => "critRate_",
            ArtifactStatName::Atk => "atk",
            ArtifactStatName::AtkPercentage => "atk_",
            ArtifactStatName::ElementalMastery => "eleMas",
            ArtifactStatName::Recharge => "enerRech_",
            ArtifactStatName::HpPercentage => "hp_",
            ArtifactStatName::Hp => "hp",
            ArtifactStatName::DefPercentage => "def_",
            ArtifactStatName::Def => "def",
            ArtifactStatName::ElectroBonus => "electro_dmg_",
            ArtifactStatName::PyroBonus => "pyro_dmg_",
            ArtifactStatName::HydroBonus => "hydro_dmg_",
            ArtifactStatName::CryoBonus => "cryo_dmg_",
            ArtifactStatName::AnemoBonus => "anemo_dmg_",
            ArtifactStatName::GeoBonus => "geo_dmg_",
            ArtifactStatName::PhysicalBonus => "physical_dmg_",
            ArtifactStatName::DendroBonus => "dendro_dmg_",
        }
    }
}

impl ArtifactSlot {
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactSlot::Flower => "flower",
            ArtifactSlot::Feather => "plume",
            ArtifactSlot::Sand => "sands",
            ArtifactSlot::Goblet => "goblet",
            ArtifactSlot::Head => "circlet",
        }
    }
}

impl ArtifactSetName {
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactSetName::ArchaicPetra => "ArchaicPetra",
            ArtifactSetName::HeartOfDepth => "HeartOfDepth",
            ArtifactSetName::BlizzardStrayer => "BlizzardStrayer",
            ArtifactSetName::RetracingBolide => "RetracingBolide",
            ArtifactSetName::NoblesseOblige => "NoblesseOblige",
            ArtifactSetName::GladiatorFinale => "GladiatorsFinale",
            ArtifactSetName::MaidenBeloved => "MaidenBeloved",
            ArtifactSetName::ViridescentVenerer => "ViridescentVenerer",
            ArtifactSetName::LavaWalker => "Lavawalker",
            ArtifactSetName::CrimsonWitch => "CrimsonWitchOfFlames",
            ArtifactSetName::ThunderSmoother => "Thundersoother",
            ArtifactSetName::ThunderingFury => "ThunderingFury",
            ArtifactSetName::BloodstainedChivalry => "BloodstainedChivalry",
            ArtifactSetName::WandererTroupe => "WanderersTroupe",
            ArtifactSetName::Scholar => "Scholar",
            ArtifactSetName::Gambler => "Gambler",
            ArtifactSetName::TinyMiracle => "TinyMiracle",
            ArtifactSetName::MartialArtist => "MartialArtist",
            ArtifactSetName::BraveHeart => "BraveHeart",
            ArtifactSetName::ResolutionOfSojourner => "ResolutionOfSojourner",
            ArtifactSetName::DefenderWill => "DefendersWill",
            ArtifactSetName::Berserker => "Berserker",
            ArtifactSetName::Instructor => "Instructor",
            ArtifactSetName::Exile => "TheExile",
            ArtifactSetName::Adventurer => "Adventurer",
            ArtifactSetName::LuckyDog => "LuckyDog",
            ArtifactSetName::TravelingDoctor => "TravelingDoctor",
            ArtifactSetName::PrayersForWisdom => "PrayersForWisdom",
            ArtifactSetName::PrayersToSpringtime => "PrayersToSpringtime",
            ArtifactSetName::PrayersForIllumination => "PrayersForIllumination",
            ArtifactSetName::PrayersForDestiny => "PrayersForDestiny",
            ArtifactSetName::PaleFlame => "PaleFlame",
            ArtifactSetName::TenacityOfTheMillelith => "TenacityOfTheMillelith",
            ArtifactSetName::EmblemOfSeveredFate => "EmblemOfSeveredFate",
            ArtifactSetName::ShimenawaReminiscence => "ShimenawasReminiscence",
            ArtifactSetName::HuskOfOpulentDreams => "HuskOfOpulentDreams",
            ArtifactSetName::OceanHuedClam => "OceanHuedClam",
            ArtifactSetName::VermillionHereafter => "VermillionHereafter",
            ArtifactSetName::EchoesOfAnOffering => "EchoesOfAnOffering",
            ArtifactSetName::DeepwoodMemories => "DeepwoodMemories",
            ArtifactSetName::GildedDreams => "GildedDreams",
            ArtifactSetName::FlowerOfParadiseLost => "FlowerOfParadiseLost",
            ArtifactSetName::DesertPavilionChronicle => "DesertPavilionChronicle",
            ArtifactSetName::NymphsDream => "NymphsDream",
            ArtifactSetName::VourukashasGlow => "VourukashasGlow",
            ArtifactSetName::MarechausseeHunter => "MarechausseeHunter",
            ArtifactSetName::GoldenTroupe => "GoldenTroupe",
        }
    }
}

#[derive(Serialize)]
pub struct GOODFormat<'a> {
    format: &'a str,
    version: u32,
    source: &'a str,
    artifacts: Vec<GOODArtifact<'a>>,
}

impl<'a> GOODFormat<'a> {
    pub fn new(results: &'a [GenshinArtifact]) -> GOODFormat {
        let artifacts: Vec<GOODArtifact<'a>> = results
            .iter()
            .map(|artifact| GOODArtifact { artifact })
            .collect();
        GOODFormat {
            format: "GOOD",
            version: 1,
            source: "yas",
            artifacts,
        }
    }

    pub fn save(&self, path: String) {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let s = serde_json::to_string(&self).unwrap();

        if let Err(why) = file.write_all(s.as_bytes()) {
            panic!("couldn't write to {}: {}", path, why)
        }
    }
}
