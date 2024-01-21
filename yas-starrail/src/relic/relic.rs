use log::error;
use regex::Regex;
use std::hash::{Hash, Hasher};
use strum_macros::Display;
use crate::scanner::relic_scanner::StarRailRelicScanResult;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicStatName {
    HP,
    HPPercentage,
    ATK,
    ATKPercentage,
    DEFPercentage,
    SPD,
    CRITRate,
    CRITDMG,
    BreakEffect,
    OutgoingHealingBoost,
    EnergyRegenerationRate,
    EffectHitRate,
    PhysicalDMGBoost,
    FireDMGBoost,
    IceDMGBoost,
    LightningDMGBoost,
    WindDMGBoost,
    QuantumDMGBoost,
    ImaginaryDMGBoost,
    DEF,
    EffectRES,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicSlot {
    Head,
    Hands,
    Body,
    Feet,
    PlanarSphere,
    LinkRope,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum RelicSetName {
    PasserbyofWanderingCloud,
    MusketeerofWildWheat,
    KnightofPurityPalace,
    HunterofGlacialForest,
    ChampionofStreetwiseBoxing,
    GuardofWutheringSnow,
    FiresmithofLavaForging,
    GeniusofBrilliantStars,
    BandofSizzlingThunder,
    EagleofTwilightLine,
    ThiefofShootingMeteor,
    WastelanderofBanditryDesert,
    SpaceSealingStation,
    FleetoftheAgeless,
    PanGalacticCommercialEnterprise,
    BelobogoftheArchitects,
    CelestialDifferentiator,
    InertSalsotto,
    TaliaKingdomofBanditry,
    SprightlyVonwacq,
    RutilantArena,
    BrokenKeel,
    LongevousDisciple,
    MessengerTraversingHackerspace,
}

#[derive(Debug, Clone)]
pub struct RelicStat {
    pub name: RelicStatName,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct StarRailRelic {
    pub set_name: RelicSetName,
    pub slot: RelicSlot,
    pub star: i32,
    pub level: i32,
    pub main_stat: RelicStat,
    pub sub_stat_1: Option<RelicStat>,
    pub sub_stat_2: Option<RelicStat>,
    pub sub_stat_3: Option<RelicStat>,
    pub sub_stat_4: Option<RelicStat>,
    pub equip: Option<String>,
}

impl Hash for RelicStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for RelicStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for RelicStat {}

impl RelicStatName {
    #[rustfmt::skip]
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<RelicStatName> {
        match name {
            "生命值" => if is_percentage { Some(RelicStatName::HPPercentage) } else { Some(RelicStatName::HP) },
            "攻击力" => if is_percentage { Some(RelicStatName::ATKPercentage) } else { Some(RelicStatName::ATK) },
            "防御力" => if is_percentage { Some(RelicStatName::DEFPercentage) } else { Some(RelicStatName::DEF) },
            "速度" => Some(RelicStatName::SPD),
            "暴击率" => Some(RelicStatName::CRITRate),
            "暴击伤害" => Some(RelicStatName::CRITDMG),
            "击破特攻" => Some(RelicStatName::BreakEffect),
            "治疗量加成" => Some(RelicStatName::OutgoingHealingBoost),
            "能量恢复效率" => Some(RelicStatName::EnergyRegenerationRate),
            "效果命中" => Some(RelicStatName::EffectHitRate),
            "物理属性伤害提高" => Some(RelicStatName::PhysicalDMGBoost),
            "火属性伤害提高" => Some(RelicStatName::FireDMGBoost),
            "冰属性伤害提高" => Some(RelicStatName::IceDMGBoost),
            "雷属性伤害提高" => Some(RelicStatName::LightningDMGBoost),
            "风属性伤害提高" => Some(RelicStatName::WindDMGBoost),
            "量子属性伤害提高" => Some(RelicStatName::QuantumDMGBoost),
            "虚数属性伤害提高" => Some(RelicStatName::ImaginaryDMGBoost),
            "效果抵抗" => Some(RelicStatName::EffectRES),
            _ => None,
        }
    }
}

impl RelicStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<RelicStat> {
        let temp: Vec<&str> = s.split('+').collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains('%');
        let stat_name = match RelicStatName::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let mut value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("stat `{}` parse error", s);
                return None;
            },
        };
        if is_percentage {
            value /= 100.0;
        }

        Some(RelicStat {
            name: stat_name,
            value,
        })
    }
}

impl TryFrom<&StarRailRelicScanResult> for StarRailRelic {
    type Error = ();

    fn try_from(value: &StarRailRelicScanResult) -> Result<Self, Self::Error> {
        let set_name = RelicSetName::from_zh_cn(&value.name).ok_or(())?;
        let slot = RelicSlot::from_zh_cn(&value.name).ok_or(())?;
        let star = value.star;

        let main_stat = RelicStat::from_zh_cn_raw(
            (value.main_stat_name.clone() + "+" + value.main_stat_value.as_str()).as_str(),
        )
        .ok_or(())?;
        let sub1 = RelicStat::from_zh_cn_raw(&(value.sub_stat_name[0].clone() + "+" + value.sub_stat_value[0].as_str()));
        let sub2 = RelicStat::from_zh_cn_raw(&(value.sub_stat_name[1].clone() + "+" + value.sub_stat_value[1].as_str()));
        let sub3 = RelicStat::from_zh_cn_raw(&(value.sub_stat_name[2].clone() + "+" + value.sub_stat_value[2].as_str()));
        let sub4 = RelicStat::from_zh_cn_raw(&(value.sub_stat_name[3].clone() + "+" + value.sub_stat_value[3].as_str()));

        let equip = None;

        Ok(StarRailRelic {
            set_name,
            slot,
            star,
            level: value.level,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
            equip,
        })
    }
}

impl RelicSetName {
    #[rustfmt::skip]
    pub fn from_zh_cn(s: &str) -> Option<RelicSetName> {
        match s {
            "过客的逢春木簪" | "过客的游龙臂鞲" | "过客的残绣风衣" | "过客的冥途游履" => Some(RelicSetName::PasserbyofWanderingCloud),
            "快枪手的野穗毡帽" | "快枪手的粗革手套" | "快枪手的猎风披肩" | "快枪手的铆钉马靴" => Some(RelicSetName::MusketeerofWildWheat),
            "圣骑的宽恕盔面" | "圣骑的沉默誓环" | "圣骑的肃穆胸甲" | "圣骑的秩序铁靴" => Some(RelicSetName::KnightofPurityPalace),
            "雪猎的荒神兜帽" | "雪猎的巨蜥手套" | "雪猎的冰龙披风" | "雪猎的鹿皮软靴" => Some(RelicSetName::HunterofGlacialForest),
            "拳王的冠军护头" | "拳王的重炮拳套" | "拳王的贴身护胸" | "拳王的弧步战靴" => Some(RelicSetName::ChampionofStreetwiseBoxing),
            "铁卫的铸铁面盔" | "铁卫的银鳞手甲" | "铁卫的旧制军服" | "铁卫的白银护胫" => Some(RelicSetName::GuardofWutheringSnow),
            "火匠的黑耀目镜" | "火匠的御火戒指" | "火匠的阻燃围裙" | "火匠的合金义肢" => Some(RelicSetName::FiresmithofLavaForging),
            "天才的超距遥感" | "天才的频变捕手" | "天才的元域深潜" | "天才的引力漫步" => Some(RelicSetName::GeniusofBrilliantStars),
            "乐队的偏光墨镜" | "乐队的巡演手绳" | "乐队的钉刺皮衣" | "乐队的铆钉短靴" => Some(RelicSetName::BandofSizzlingThunder),
            "翔鹰的长喙头盔" | "翔鹰的鹰击指环" | "翔鹰的翼装束带" | "翔鹰的绒羽绑带" => Some(RelicSetName::EagleofTwilightLine),
            "怪盗的千人假面" | "怪盗的绘纹手套" | "怪盗的纤钢爪钩" | "怪盗的流星快靴" => Some(RelicSetName::ThiefofShootingMeteor),
            "废土客的呼吸面罩" | "废土客的荒漠终端" | "废土客的修士长袍" | "废土客的动力腿甲" => Some(RelicSetName::WastelanderofBanditryDesert),
            "「黑塔」的空间站点" | "「黑塔」的漫历轨迹" => Some(RelicSetName::SpaceSealingStation),
            "罗浮仙舟的天外楼船" | "罗浮仙舟的建木枝蔓" => Some(RelicSetName::FleetoftheAgeless),
            "公司的巨构总部" | "公司的贸易航道" => Some(RelicSetName::PanGalacticCommercialEnterprise),
            "贝洛伯格的存护堡垒" | "贝洛伯格的铁卫防线" => Some(RelicSetName::BelobogoftheArchitects),
            "螺丝星的机械烈阳" | "螺丝星的环星孔带" => Some(RelicSetName::CelestialDifferentiator),
            "萨尔索图的移动城市" | "萨尔索图的晨昏界线" => Some(RelicSetName::InertSalsotto),
            "塔利亚的钉壳小镇" | "塔利亚的裸皮电线" => Some(RelicSetName::TaliaKingdomofBanditry),
            "翁瓦克的诞生之岛" | "翁瓦克的环岛海岸" => Some(RelicSetName::FleetoftheAgeless),
            "泰科铵的镭射球场" | "泰科铵的弧光赛道" => Some(RelicSetName::RutilantArena),
            "伊须磨洲的残船鲸落" | "伊须磨洲的坼裂缆索" => Some(RelicSetName::BrokenKeel),
            "莳者的复明义眼" | "莳者的机巧木手" | "莳者的承露羽衣" | "莳者的天人丝履" => Some(RelicSetName::LongevousDisciple),
            "信使的全息目镜" | "信使的百变义手" | "信使的密信挎包" | "信使的酷跑板鞋" => Some(RelicSetName::MessengerTraversingHackerspace),
            _ => None,
        }
    }
}

impl RelicSlot {
    #[rustfmt::skip]
    pub fn from_zh_cn(s: &str) -> Option<RelicSlot> {
        match s {
            "过客的逢春木簪" | "快枪手的野穗毡帽" | "圣骑的宽恕盔面" | "雪猎的荒神兜帽" | "拳王的冠军护头" | "铁卫的铸铁面盔" | "火匠的黑耀目镜" | "天才的超距遥感" | "乐队的偏光墨镜" | "翔鹰的长喙头盔" | "怪盗的千人假面" | "废土客的呼吸面罩" | "莳者的复明义眼" | "信使的全息目镜" => Some(RelicSlot::Head),
            "过客的游龙臂鞲" | "快枪手的粗革手套" | "圣骑的沉默誓环" | "雪猎的巨蜥手套" | "拳王的重炮拳套" | "铁卫的银鳞手甲" | "火匠的御火戒指" | "天才的频变捕手" | "乐队的巡演手绳" | "翔鹰的鹰击指环" | "怪盗的绘纹手套" | "废土客的荒漠终端" | "莳者的机巧木手" | "信使的百变义手" => Some(RelicSlot::Hands),
            "过客的残绣风衣" | "快枪手的猎风披肩" | "圣骑的肃穆胸甲" | "雪猎的冰龙披风" | "拳王的贴身护胸" | "铁卫的旧制军服" | "火匠的阻燃围裙" | "天才的元域深潜" | "乐队的钉刺皮衣" | "翔鹰的翼装束带" | "怪盗的纤钢爪钩" | "废土客的修士长袍" | "莳者的承露羽衣" | "信使的密信挎包" => Some(RelicSlot::Body),
            "过客的冥途游履" | "快枪手的铆钉马靴" | "圣骑的秩序铁靴" | "雪猎的鹿皮软靴" | "拳王的弧步战靴" | "铁卫的白银护胫" | "火匠的合金义肢" | "天才的引力漫步" | "乐队的铆钉短靴" | "翔鹰的绒羽绑带" | "怪盗的流星快靴" | "废土客的动力腿甲" | "莳者的天人丝履" | "信使的酷跑板鞋" => Some(RelicSlot::Feet),
            "「黑塔」的空间站点" | "罗浮仙舟的天外楼船" | "公司的巨构总部" | "贝洛伯格的存护堡垒" | "螺丝星的机械烈阳" | "萨尔索图的移动城市" | "塔利亚的钉壳小镇" | "翁瓦克的诞生之岛" | "泰科铵的镭射球场" | "伊须磨洲的残船鲸落" => Some(RelicSlot::PlanarSphere),
            "「黑塔」的漫历轨迹" | "罗浮仙舟的建木枝蔓" | "公司的贸易航道" | "贝洛伯格的铁卫防线" | "螺丝星的环星孔带" | "萨尔索图的晨昏界线" | "塔利亚的裸皮电线" | "翁瓦克的环岛海岸" | "泰科铵的弧光赛道" | "伊须磨洲的坼裂缆索" => Some(RelicSlot::LinkRope),
            _ => None,
        }
    }
}