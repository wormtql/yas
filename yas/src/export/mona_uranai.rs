use std::convert::From;
use std::fs::File;
use std::io::prelude::*;

use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::core::genshin::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, GenshinArtifact,
};

type MonaArtifact = GenshinArtifact;

impl ArtifactStatName {
    pub fn to_mona(&self) -> String {
        let temp = match self {
            ArtifactStatName::HealingBonus => "cureEffect",
            ArtifactStatName::CriticalDamage => "criticalDamage",
            ArtifactStatName::Critical => "critical",
            ArtifactStatName::Atk => "attackStatic",
            ArtifactStatName::AtkPercentage => "attackPercentage",
            ArtifactStatName::ElementalMastery => "elementalMastery",
            ArtifactStatName::Recharge => "recharge",
            ArtifactStatName::HpPercentage => "lifePercentage",
            ArtifactStatName::Hp => "lifeStatic",
            ArtifactStatName::DefPercentage => "defendPercentage",
            ArtifactStatName::Def => "defendStatic",
            ArtifactStatName::ElectroBonus => "thunderBonus",
            ArtifactStatName::PyroBonus => "fireBonus",
            ArtifactStatName::HydroBonus => "waterBonus",
            ArtifactStatName::CryoBonus => "iceBonus",
            ArtifactStatName::AnemoBonus => "windBonus",
            ArtifactStatName::GeoBonus => "rockBonus",
            ArtifactStatName::PhysicalBonus => "physicalBonus",
            ArtifactStatName::DendroBonus => "dendroBonus",
        };
        String::from(temp)
    }
}

impl ArtifactSetName {
    pub fn to_mona(&self) -> String {
        let same = self.to_string();
        let temp = match self {
            ArtifactSetName::ArchaicPetra => "archaicPetra",
            ArtifactSetName::HeartOfDepth => "heartOfDepth",
            ArtifactSetName::BlizzardStrayer => "blizzardStrayer",
            ArtifactSetName::RetracingBolide => "retracingBolide",
            ArtifactSetName::NoblesseOblige => "noblesseOblige",
            ArtifactSetName::GladiatorFinale => "gladiatorFinale",
            ArtifactSetName::MaidenBeloved => "maidenBeloved",
            ArtifactSetName::ViridescentVenerer => "viridescentVenerer",
            ArtifactSetName::LavaWalker => "lavaWalker",
            ArtifactSetName::CrimsonWitch => "crimsonWitch",
            ArtifactSetName::ThunderSmoother => "thunderSmoother",
            ArtifactSetName::ThunderingFury => "thunderingFury",
            ArtifactSetName::BloodstainedChivalry => "bloodstainedChivalry",
            ArtifactSetName::WandererTroupe => "wandererTroupe",
            ArtifactSetName::Scholar => "scholar",
            ArtifactSetName::Gambler => "gambler",
            ArtifactSetName::TinyMiracle => "tinyMiracle",
            ArtifactSetName::MartialArtist => "martialArtist",
            ArtifactSetName::BraveHeart => "braveHeart",
            ArtifactSetName::ResolutionOfSojourner => "resolutionOfSojourner",
            ArtifactSetName::DefenderWill => "defenderWill",
            ArtifactSetName::Berserker => "berserker",
            ArtifactSetName::Instructor => "instructor",
            ArtifactSetName::Exile => "exile",
            ArtifactSetName::Adventurer => "adventurer",
            ArtifactSetName::LuckyDog => "luckyDog",
            ArtifactSetName::TravelingDoctor => "travelingDoctor",
            ArtifactSetName::PrayersForWisdom => "prayersForWisdom",
            ArtifactSetName::PrayersToSpringtime => "prayersToSpringtime",
            ArtifactSetName::PrayersForIllumination => "prayersForIllumination",
            ArtifactSetName::PrayersForDestiny => "prayersForDestiny",
            ArtifactSetName::PaleFlame => "paleFlame",
            ArtifactSetName::TenacityOfTheMillelith => "tenacityOfTheMillelith",
            ArtifactSetName::EmblemOfSeveredFate => "emblemOfSeveredFate",
            ArtifactSetName::ShimenawaReminiscence => "shimenawaReminiscence",
            ArtifactSetName::HuskOfOpulentDreams => "huskOfOpulentDreams",
            ArtifactSetName::OceanHuedClam => "oceanHuedClam",
            _ => same.as_str(),
        };
        String::from(temp)
    }
}

impl ArtifactSlot {
    pub fn to_mona(&self) -> String {
        let temp = match self {
            ArtifactSlot::Flower => "flower",
            ArtifactSlot::Feather => "feather",
            ArtifactSlot::Sand => "sand",
            ArtifactSlot::Goblet => "cup",
            ArtifactSlot::Head => "head",
        };
        String::from(temp)
    }
}

impl Serialize for ArtifactStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("name", &self.name.to_mona()).unwrap();
        root.serialize_entry("value", &self.value).unwrap();
        root.end()
    }
}

impl Serialize for MonaArtifact {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(7))?;

        root.serialize_entry("setName", &self.set_name.to_mona())
            .unwrap();
        root.serialize_entry("position", &self.slot.to_mona())
            .unwrap();
        root.serialize_entry("mainTag", &self.main_stat).unwrap();

        let mut sub_stats: Vec<&ArtifactStat> = vec![];
        if let Some(ref s) = self.sub_stat_1 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_2 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_3 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_4 {
            sub_stats.push(s);
        }
        // let mut subs = serializer.serialize_seq(Some(sub_stats.len()))?;
        //
        // for i in sub_stats {
        //     subs.serialize_element(i);
        // }
        // subs.end();
        // subs.

        root.serialize_entry("normalTags", &sub_stats)?;
        root.serialize_entry("omit", &false)?;
        root.serialize_entry("level", &self.level)?;
        root.serialize_entry("star", &self.star)?;
        root.serialize_entry("equip", &self.equip)?;
        // let random_id = thread_rng().gen::<u64>();
        // root.serialize_entry("id", &random_id);

        root.end()
    }
}

pub struct MonaFormat<'a> {
    version: String,
    flower: Vec<&'a MonaArtifact>,
    feather: Vec<&'a MonaArtifact>,
    cup: Vec<&'a MonaArtifact>,
    sand: Vec<&'a MonaArtifact>,
    head: Vec<&'a MonaArtifact>,
}

impl<'a> Serialize for MonaFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(6))?;
        root.serialize_entry("version", &self.version).unwrap();
        root.serialize_entry("flower", &self.flower).unwrap();
        root.serialize_entry("feather", &self.feather).unwrap();
        root.serialize_entry("sand", &self.sand).unwrap();
        root.serialize_entry("cup", &self.cup).unwrap();
        root.serialize_entry("head", &self.head).unwrap();
        root.end()
    }
}

impl<'a> MonaFormat<'a> {
    pub fn new(results: &Vec<GenshinArtifact>) -> MonaFormat {
        let mut flower: Vec<&MonaArtifact> = Vec::new();
        let mut feather: Vec<&MonaArtifact> = Vec::new();
        let mut cup: Vec<&MonaArtifact> = Vec::new();
        let mut sand: Vec<&MonaArtifact> = Vec::new();
        let mut head: Vec<&MonaArtifact> = Vec::new();

        for art in results.iter() {
            match art.slot {
                ArtifactSlot::Flower => flower.push(art),
                ArtifactSlot::Feather => feather.push(art),
                ArtifactSlot::Sand => sand.push(art),
                ArtifactSlot::Goblet => cup.push(art),
                ArtifactSlot::Head => head.push(art),
            }
        }

        MonaFormat {
            flower,
            feather,
            cup,
            sand,
            head,

            version: String::from("1"),
        }
    }

    pub fn save(&self, path: String) {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let s = serde_json::to_string(&self).unwrap();

        match file.write_all(s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path, why),
            _ => {},
        }
    }
}
