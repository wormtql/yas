use crate::artifact::ArtifactSlot;

impl ArtifactSlot {
    pub fn to_zh_cn(&self) -> &'static str {
        match *self {
            ArtifactSlot::Flower => "生之花",
            ArtifactSlot::Feather => "死之羽",
            ArtifactSlot::Sand => "时之沙",
            ArtifactSlot::Goblet => "空之杯",
            ArtifactSlot::Head => "理之冠",
        }
    }
}
