use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::artifact::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, GenshinArtifact,
};

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
        root.serialize_entry(
            "location",
            equip_from_zh_cn(artifact.equip.clone().as_deref()),
        )?;
        root.serialize_entry("lock", &artifact.lock)?;
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
            ArtifactSetName::SongOfDaysPast => "SongOfDaysPast",
            ArtifactSetName::NighttimeWhispersInTheEchoingWoods => "NighttimeWhispersInTheEchoingWoods",
            ArtifactSetName::FragmentOfHarmonicWhimsy => "FragmentOfHarmonicWhimsy",
            ArtifactSetName::UnfinishedReverie => "UnfinishedReverie",
        }
    }
}

fn equip_from_zh_cn(equip: Option<&str>) -> &'static str {
    match equip {
        Some("旅行者") => "Traveler",
        Some("神里绫华") => "KamisatoAyaka",
        Some("琴") => "Jean",
        Some("丽莎") => "Lisa",
        Some("芭芭拉") => "Barbara",
        Some("凯亚") => "Kaeya",
        Some("迪卢克") => "Diluc",
        Some("雷泽") => "Razor",
        Some("安柏") => "Amber",
        Some("温迪") => "Venti",
        Some("香菱") => "Xiangling",
        Some("北斗") => "Beidou",
        Some("行秋") => "Xingqiu",
        Some("魈") => "Xiao",
        Some("凝光") => "Ningguang",
        Some("可莉") => "Klee",
        Some("钟离") => "Zhongli",
        Some("菲谢尔") => "Fischl",
        Some("班尼特") => "Bennett",
        Some("达达利亚") => "Tartaglia",
        Some("诺艾尔") => "Noelle",
        Some("七七") => "Qiqi",
        Some("重云") => "Chongyun",
        Some("甘雨") => "Ganyu",
        Some("阿贝多") => "Albedo",
        Some("迪奥娜") => "Diona",
        Some("莫娜") => "Mona",
        Some("刻晴") => "Keqing",
        Some("砂糖") => "Sucrose",
        Some("辛焱") => "Xinyan",
        Some("罗莎莉亚") => "Rosaria",
        Some("胡桃") => "HuTao",
        Some("枫原万叶") => "KaedeharaKazuha",
        Some("烟绯") => "Yanfei",
        Some("宵宫") => "Yoimiya",
        Some("托马") => "Thoma",
        Some("优菈") => "Eula",
        Some("雷电将军") => "RaidenShogun",
        Some("早柚") => "Sayu",
        Some("珊瑚宫心海") => "SangonomiyaKokomi",
        Some("五郎") => "Gorou",
        Some("九条裟罗") => "KujouSara",
        Some("荒泷一斗") => "AratakiItto",
        Some("八重神子") => "YaeMiko",
        Some("鹿野院平藏") => "ShikanoinHeizou",
        Some("夜兰") => "Yelan",
        Some("绮良良") => "Kirara",
        Some("埃洛伊") => "Aloy",
        Some("申鹤") => "Shenhe",
        Some("云堇") => "YunJin",
        Some("久岐忍") => "KukiShinobu",
        Some("神里绫人") => "KamisatoAyato",
        Some("柯莱") => "Collei",
        Some("多莉") => "Dori",
        Some("提纳里") => "Tighnari",
        Some("妮露") => "Nilou",
        Some("赛诺") => "Cyno",
        Some("坎蒂丝") => "Candace",
        Some("纳西妲") => "Nahida",
        Some("莱依拉") => "Layla",
        Some("流浪者") => "Wanderer",
        Some("珐露珊") => "Faruzan",
        Some("瑶瑶") => "Yaoyao",
        Some("艾尔海森") => "Alhaitham",
        Some("迪希雅") => "Dehya",
        Some("米卡") => "Mika",
        Some("卡维") => "Kaveh",
        Some("白术") => "Baizhu",
        Some("琳妮特") => "Lynette",
        Some("林尼") => "Lyney",
        Some("菲米尼") => "Freminet",
        Some("那维莱特") => "Neuvillette",
        Some("莱欧斯利") => "Wriothesley",
        Some("夏洛蒂") => "Charlotte",
        Some("芙宁娜") => "Furina",
        Some("夏沃蕾") => "Chevreuse",
        Some("娜维娅") => "Navia",
        Some("嘉明") => "Gaming",
        Some("闲云") => "Xianyun",
        Some("千织") => "Chiori",
        Some("阿蕾奇诺") => "Arlecchino",
        Some("希格雯") => "Sigewinne",
        Some("赛索斯") => "Sethos",
        Some("克洛琳德") => "Clorinde",
        Some("艾梅莉埃") => "Emilie",
        _ => "",
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
}
