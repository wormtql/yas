use std::ops::Deref;
use nanoid::nanoid;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::relic::{RelicSetName, RelicSlot, RelicStat, RelicStatName, StarRailRelic};

struct HSRRelic<'a>(&'a StarRailRelic);

impl<'a> Deref for HSRRelic<'a> {
    type Target = StarRailRelic;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// https://github.com/kel-z/HSR-Scanner
pub struct StarRailHSRFormat<'a> {
    results: Vec<HSRRelic<'a>>,
    version: usize,
}

impl<'a> StarRailHSRFormat<'a> {
    pub fn new_version3(results: &'a [StarRailRelic]) -> Self {
        let mut r = Vec::new();
        for item in results.iter() {
            r.push(HSRRelic(item));
        }
        Self {
            results: r,
            version: 3
        }
    }
}

impl RelicStatName {
    pub fn to_hsr_stat_name(&self) -> &'static str {
        match *self {
            RelicStatName::SPD => "SPD",
            RelicStatName::HP => "HP",
            RelicStatName::ATK => "ATK",
            RelicStatName::DEF => "DEF",
            RelicStatName::BreakEffect => "Break Effect",
            RelicStatName::EffectHitRate => "Effect Hit Rate",
            RelicStatName::EnergyRegenerationRate => "Energy Regeneration Rate",
            RelicStatName::OutgoingHealingBoost => "Outgoing Healing Boost",
            RelicStatName::PhysicalDMGBoost => "Physical DMG Boost",
            RelicStatName::FireDMGBoost => "Fire DMG Boost",
            RelicStatName::IceDMGBoost => "Ice DMG Boost",
            RelicStatName::WindDMGBoost => "Wind DMG Boost",
            RelicStatName::LightningDMGBoost => "Lightning DMG Boost",
            RelicStatName::QuantumDMGBoost => "Quantum DMG Boost",
            RelicStatName::ImaginaryDMGBoost => "Imaginary DMG Boost",
            RelicStatName::CRITRate => "CRIT Rate",
            RelicStatName::CRITDMG => "CRIT DMG",
            RelicStatName::EffectRES => "Effect RES",
            RelicStatName::HPPercentage => "HP",
            RelicStatName::ATKPercentage => "ATK",
            RelicStatName::DEFPercentage => "DEF",
        }
    }
}

impl RelicSetName {
    pub fn to_hsr_set_name(&self) -> &'static str {
        match *self {
            RelicSetName::PasserbyofWanderingCloud => "Passerby of Wandering Cloud",
            RelicSetName::MusketeerofWildWheat => "Musketeer of Wild Wheat",
            RelicSetName::KnightofPurityPalace => "Knight of Purity Palace",
            RelicSetName::HunterofGlacialForest => "Hunter of Glacial Forest",
            RelicSetName::ChampionofStreetwiseBoxing => "Champion of Streetwise Boxing",
            RelicSetName::GuardofWutheringSnow => "Guard of Wuthering Snow",
            RelicSetName::FiresmithofLavaForging => "Firesmith of Lava-Forging",
            RelicSetName::GeniusofBrilliantStars => "Genius of Brilliant Stars",
            RelicSetName::BandofSizzlingThunder => "Band of Sizzling Thunder",
            RelicSetName::EagleofTwilightLine => "Eagle of Twilight Line",
            RelicSetName::ThiefofShootingMeteor => "Thief of Shooting Meteor",
            RelicSetName::WastelanderofBanditryDesert => "Wastelander of Banditry Desert",
            RelicSetName::LongevousDisciple => "Longevous Disciple",
            RelicSetName::MessengerTraversingHackerspace => "Messenger Traversing Hackerspace",
            RelicSetName::TheAshblazingGrandDuke => "The Ashblazing Grand Duke",
            RelicSetName::PrisonerinDeepConfinement => "Prisoner in Deep Confinement",
            RelicSetName::PioneerDiverofDeadWaters => "Pioneer Diver of Dead Waters",
            RelicSetName::WatchmakerMasterofDreamMachinations => "Watchmaker, Master of Dream Machinations",
            RelicSetName::SpaceSealingStation => "Space Sealing Station",
            RelicSetName::FleetoftheAgeless => "Fleet of the Ageless",
            RelicSetName::PanCosmicCommercialEnterprise => "Pan-Cosmic Commercial Enterprise",
            RelicSetName::BelobogoftheArchitects => "Belobog's Fortress of Preservation",
            RelicSetName::CelestialDifferentiator => "Celestial Differentiator",
            RelicSetName::InertSalsotto => "Inert Salsotto",
            RelicSetName::TaliaKingdomofBanditry => "Talia: Kingdom of Banditry",
            RelicSetName::SprightlyVonwacq => "Sprightly Vonwacq",
            RelicSetName::RutilantArena => "Rutilant Arena",
            RelicSetName::BrokenKeel => "Broken Keel",
            RelicSetName::FirmamentFrontlineGlamoth => "Firmament Frontline: Glamoth",
            RelicSetName::PenaconyLandoftheDreams => "Penacony, Land of the Dreams",
        }
    }
}

impl RelicSlot {
    pub fn to_hsr_slot_name(&self) -> &'static str {
        match *self {
            RelicSlot::Head => "Head",
            RelicSlot::Hands => "Hands",
            RelicSlot::Body => "Body",
            RelicSlot::Feet => "Feet",
            RelicSlot::PlanarSphere => "Planar Sphere",
            RelicSlot::LinkRope => "Link Rope",
        }
    }
}

struct HSRStat(RelicStat);

impl Serialize for HSRStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut root = serializer.serialize_map(Some(2))?;

        let mut name = String::from(self.0.name.to_hsr_stat_name());
        let is_percentage = self.0.name.is_percentage();
        if is_percentage {
            name += "_";
        }
        root.serialize_entry("key", &name)?;
        let value = if is_percentage {
            self.0.value * 100.0
        } else {
            self.0.value
        };
        root.serialize_entry("value", &value)?;

        root.end()
    }
}

impl<'a> Serialize for HSRRelic<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut root = serializer.serialize_map(Some(10))?;

        root.serialize_entry("set", self.set_name.to_hsr_set_name())?;
        root.serialize_entry("slot", self.slot.to_hsr_slot_name())?;
        root.serialize_entry("rarity", &self.star)?;
        root.serialize_entry("level", &self.level)?;
        root.serialize_entry("mainstat", self.main_stat.name.to_hsr_stat_name())?;

        let mut sub_stats: Vec<HSRStat> = Vec::new();
        if let Some(s) = &self.sub_stat_1 {
            sub_stats.push(HSRStat(s.clone()));
        }
        if let Some(s) = &self.sub_stat_2 {
            sub_stats.push(HSRStat(s.clone()));
        }
        if let Some(s) = &self.sub_stat_3 {
            sub_stats.push(HSRStat(s.clone()));
        }
        if let Some(s) = &self.sub_stat_4 {
            sub_stats.push(HSRStat(s.clone()));
        }
        root.serialize_entry("substats", &sub_stats)?;
        root.serialize_entry("location", "")?;
        root.serialize_entry("lock", &self.lock)?;
        root.serialize_entry("discard", &self.discard)?;
        root.serialize_entry("_id", &nanoid!())?;

        root.end()
    }
}

impl<'a> Serialize for StarRailHSRFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut root = serializer.serialize_map(None)?;

        root.serialize_entry("source", "yas-scanner")?;
        root.serialize_entry("build", &(String::from("v") + env!("CARGO_PKG_VERSION")))?;
        root.serialize_entry("version", &self.version)?;
        root.serialize_entry::<str, [usize; 0]>("light_cones", &[])?;
        root.serialize_entry("relics", &self.results)?;
        root.serialize_entry::<str, [usize; 0]>("characters", &[])?;

        root.end()
    }
}
