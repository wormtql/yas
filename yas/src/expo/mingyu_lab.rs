use crate::item::genshin_artifact::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, GenshinArtifact,
};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::fs::File;
use std::io::prelude::*;

struct MingyuLabArtifact<'a> {
    artifact: &'a GenshinArtifact,
}

impl<'a> Serialize for MingyuLabArtifact<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let extract_stat_name = |maybe_stat: &Option<ArtifactStat>| match maybe_stat {
            None => "flatATK",
            Some(stat) => stat.name.to_mingyu_lab(),
        };

        let extract_stat_value = |maybe_stat: &Option<ArtifactStat>| match maybe_stat {
            None => 0.0,
            Some(stat) => match stat.name {
                ArtifactStatName::Atk
                | ArtifactStatName::ElementalMastery
                | ArtifactStatName::Hp
                | ArtifactStatName::Def => stat.value,
                _ => stat.value * 100.0,
            },
        };

        let artifact = &self.artifact;
        let mut root = serializer.serialize_map(Some(13))?;
        root.serialize_entry("asKey", artifact.set_name.to_mingyu_lab())?;
        root.serialize_entry("rarity", &artifact.star)?;
        root.serialize_entry("slot", artifact.slot.to_mingyu_lab())?;
        root.serialize_entry("level", &artifact.level)?;
        root.serialize_entry("mainStat", artifact.main_stat.name.to_mingyu_lab())?;
        root.serialize_entry("subStat1Type", &extract_stat_name(&artifact.sub_stat_1))?;
        root.serialize_entry("subStat1Value", &extract_stat_value(&artifact.sub_stat_1))?;
        root.serialize_entry("subStat2Type", &extract_stat_name(&artifact.sub_stat_2))?;
        root.serialize_entry("subStat2Value", &extract_stat_value(&artifact.sub_stat_2))?;
        root.serialize_entry("subStat3Type", &extract_stat_name(&artifact.sub_stat_3))?;
        root.serialize_entry("subStat3Value", &extract_stat_value(&artifact.sub_stat_3))?;
        root.serialize_entry("subStat4Type", &extract_stat_name(&artifact.sub_stat_4))?;
        root.serialize_entry("subStat4Value", &extract_stat_value(&artifact.sub_stat_4))?;
        root.end()
    }
}

impl ArtifactStatName {
    pub fn to_mingyu_lab(&self) -> &'static str {
        match self {
            ArtifactStatName::HealingBonus => "healing",
            ArtifactStatName::CriticalDamage => "critDamage",
            ArtifactStatName::Critical => "critRate",
            ArtifactStatName::Atk => "flatATK",
            ArtifactStatName::AtkPercentage => "percentATK",
            ArtifactStatName::ElementalMastery => "elementalMastery",
            ArtifactStatName::Recharge => "energyRecharge",
            ArtifactStatName::HpPercentage => "percentHP",
            ArtifactStatName::Hp => "flatHP",
            ArtifactStatName::DefPercentage => "percentDEF",
            ArtifactStatName::Def => "flatDEF",
            ArtifactStatName::ElectroBonus => "electroDamage",
            ArtifactStatName::PyroBonus => "pyroDamage",
            ArtifactStatName::HydroBonus => "hydroDamage",
            ArtifactStatName::CryoBonus => "cryoDamage",
            ArtifactStatName::AnemoBonus => "anemoDamage",
            ArtifactStatName::GeoBonus => "geoDamage",
            ArtifactStatName::PhysicalBonus => "physicalDamage",
            ArtifactStatName::DendroBonus => "dendroDamage",
        }
    }
}

impl ArtifactSlot {
    pub fn to_mingyu_lab(&self) -> &'static str {
        match self {
            ArtifactSlot::Flower => "flower",
            ArtifactSlot::Feather => "plume",
            ArtifactSlot::Sand => "eon",
            ArtifactSlot::Goblet => "goblet",
            ArtifactSlot::Head => "circlet",
        }
    }
}

impl ArtifactSetName {
    pub fn to_mingyu_lab(&self) -> &'static str {
        match self {
            ArtifactSetName::ArchaicPetra => "archaic_petra",
            ArtifactSetName::HeartOfDepth => "heart_of_depth",
            ArtifactSetName::BlizzardStrayer => "blizzard_walker",
            ArtifactSetName::RetracingBolide => "retracing_bolide",
            ArtifactSetName::NoblesseOblige => "noblesse_oblige",
            ArtifactSetName::GladiatorFinale => "gladiators_finale",
            ArtifactSetName::MaidenBeloved => "maiden_beloved",
            ArtifactSetName::ViridescentVenerer => "viridescent_venerer",
            ArtifactSetName::LavaWalker => "lavawalker",
            ArtifactSetName::CrimsonWitch => "crimson_witch_of_flames",
            ArtifactSetName::ThunderSmoother => "thundersoother",
            ArtifactSetName::ThunderingFury => "thundering_fury",
            ArtifactSetName::BloodstainedChivalry => "bloodstained_chivalry",
            ArtifactSetName::WandererTroupe => "wanderers_troupe",
            ArtifactSetName::Scholar => "scholar",
            ArtifactSetName::Gambler => "gambler",
            ArtifactSetName::TinyMiracle => "tiny_miracle",
            ArtifactSetName::MartialArtist => "martial_artist",
            ArtifactSetName::BraveHeart => "brave_heart",
            ArtifactSetName::ResolutionOfSojourner => "resolution_of_sojourner",
            ArtifactSetName::DefenderWill => "defenders_will",
            ArtifactSetName::Berserker => "berserker",
            ArtifactSetName::Instructor => "instructor",
            ArtifactSetName::Exile => "the_exile",
            ArtifactSetName::PrayersForWisdom => "prayers_of_wisdom",
            ArtifactSetName::PrayersToSpringtime => "prayers_of_springtime",
            ArtifactSetName::PrayersForIllumination => "prayers_of_illumination",
            ArtifactSetName::PrayersForDestiny => "prayers_of_destiny",
            ArtifactSetName::PaleFlame => "pale_flame",
            ArtifactSetName::TenacityOfTheMillelith => "tenacity_of_the_millelith",
            ArtifactSetName::EmblemOfSeveredFate => "seal_of_insulation",
            ArtifactSetName::ShimenawaReminiscence => "reminiscence_of_shime",
            ArtifactSetName::HuskOfOpulentDreams => "husk_of_opulent_dreams",
            ArtifactSetName::OceanHuedClam => "divine_chorus",
            ArtifactSetName::VermillionHereafter => "vermillion_hereafter",
            ArtifactSetName::EchoesOfAnOffering => "echoes_of_an_offering",
            ArtifactSetName::DeepwoodMemories => "deepwood_memories",
            ArtifactSetName::GildedDreams => "gilded_dreams",
            ArtifactSetName::FlowerOfParadiseLost => "flower_of_paradise_list",
            ArtifactSetName::DesertPavilionChronicle => "desert_pavilion_chronicle",
            ArtifactSetName::NymphsDream => "nymphs_dream",
            ArtifactSetName::VourukashasGlow => "vourukashas_glow",
            ArtifactSetName::MarechausseeHunter => "marechaussee_hunter",
            ArtifactSetName::GoldenTroupe => "golden_troupe",

            // Not supported by Mingyulab
            ArtifactSetName::Adventurer => unreachable!(),
            ArtifactSetName::LuckyDog => unreachable!(),
            ArtifactSetName::TravelingDoctor => unreachable!(),
        }
    }
}

pub struct MingyuLabFormat<'a> {
    artifacts: Vec<MingyuLabArtifact<'a>>,
}

impl<'a> MingyuLabFormat<'a> {
    pub fn new(results: &'a Vec<GenshinArtifact>) -> MingyuLabFormat {
        let artifacts: Vec<MingyuLabArtifact<'a>> = results
            .into_iter()
            .filter(|artifact| {
                artifact.set_name != ArtifactSetName::Adventurer
                    && artifact.set_name != ArtifactSetName::LuckyDog
                    && artifact.set_name != ArtifactSetName::TravelingDoctor
            })
            .map(|artifact| MingyuLabArtifact { artifact })
            .collect();
        MingyuLabFormat { artifacts }
    }

    pub fn save(&self, path: String) {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let s = serde_json::to_string(&self.artifacts).unwrap();
        match file.write_all(s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path, why),
            _ => {},
        }
    }
}
