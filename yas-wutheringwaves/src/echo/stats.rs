pub enum WWStatName {
    CriticalDamage,
    CriticalRate,
    GlacioBonus,
    AeroBonus,
    FusionBonus,
    ElectroBonus,
    HavocBonus,
    SpectroBonus,
    EnergyRegeneration,
    ATK,
    ATKPercentage,
    HP,
    HPPercentage,
    DEF,
    DEFPercentage,
    HealingBonus,
    BasicAttackBonus,
    HeavyAttackBonus,
    ResonanceSkillBonus,
    ResonanceLiberationBonus,
}

impl WWStatName {
    pub fn from_chs(chs: &str, is_percentage: bool) -> Option<Self> {
        let ret = match chs {
            "暴击伤害" => Self::CriticalDamage,
            "暴击率" => Self::CriticalRate,
            "冷凝伤害加成" => Self::GlacioBonus,
            "气动伤害加成" => Self::AeroBonus,
            "热熔伤害加成" => Self::FusionBonus,
            "导电伤害加成" => Self::ElectroBonus,
            "湮灭伤害加成" => Self::HavocBonus,
            "衍射伤害加成" => Self::SpectroBonus,
            "共鸣效率" => Self::EnergyRegeneration,
            "攻击" => if is_percentage { Self::ATKPercentage } else { Self::ATK },
            "防御" => if is_percentage { Self::DEFPercentage } else { Self::DEF },
            "生命" => if is_percentage { Self::HPPercentage } else { Self::HP },
            "治疗效果加成" => Self::HealingBonus,
            "普攻伤害加成" => Self::BasicAttackBonus,
            "重击伤害加成" => Self::HeavyAttackBonus,
            "共鸣技能伤害加成" => Self::ResonanceSkillBonus,
            "共鸣解放伤害加成" => Self::ResonanceLiberationBonus,
            _ => return None,
        };

        Some(ret)
    }
}

pub struct WWStat {
    pub name: WWStatName,
    pub value: f64,
}
