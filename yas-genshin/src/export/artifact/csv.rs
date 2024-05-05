use serde::{Serialize, Serializer};
use crate::artifact::GenshinArtifact;

pub struct GenshinArtifactCSVFormat<'a> {
    artifacts: &'a [GenshinArtifact],
}

/// CSV format:
/// set name, slot, star, level, main stat name, main stat value, [sub state name, sub state value]*4, equip
fn single_artifact_to_string(artifact: &GenshinArtifact) -> String {
    let mut s = String::new();
    s = s + &artifact.set_name.to_string();
    s = s + "," + &artifact.slot.to_string();
    s = s + "," + &format!("{}", artifact.star);
    s = s + "," + &format!("{}", artifact.level);
    s = s + "," + &artifact.main_stat.name.to_string();
    s = s + "," + &format!("{}", artifact.main_stat.value);
    if let Some(sub) = &artifact.sub_stat_1 {
        s = s + "," + &sub.name.to_string();
        s = s + "," + &format!("{}", sub.value);
    } else {
        s = s + ",,";
    }
    if let Some(sub) = &artifact.sub_stat_2 {
        s = s + "," + &sub.name.to_string();
        s = s + "," + &format!("{}", sub.value);
    } else {
        s = s + ",,";
    }
    if let Some(sub) = &artifact.sub_stat_3 {
        s = s + "," + &sub.name.to_string();
        s = s + "," + &format!("{}", sub.value);
    } else {
        s = s + ",,";
    }
    if let Some(sub) = &artifact.sub_stat_4 {
        s = s + "," + &sub.name.to_string();
        s = s + "," + &format!("{}", sub.value);
    } else {
        s = s + ",,";
    }
    if let Some(e) = &artifact.equip {
        s = s + "," + e;
    } else {
        s = s + ","
    }

    s
}

impl<'a> GenshinArtifactCSVFormat<'a> {
    pub fn new(artifacts: &'a [GenshinArtifact]) -> Self {
        Self {
            artifacts
        }
    }

    pub fn to_csv_string(&self) -> String {
        let header = "套装,部位,星级,等级,主词条名,主词条值,副词条名1,副词条值1,副词条名2,副词条值2,副词条名3,副词条值3,副词条名4,副词条值4,装备";
        let mut result = String::from(header) + "\n";

        for artifact in self.artifacts.iter() {
            let line = single_artifact_to_string(artifact);
            result = result + &line + "\n";
        }

        result
    }
}

impl<'a> Serialize for GenshinArtifactCSVFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let s = self.to_csv_string();
        serializer.serialize_str(&s)
    }
}
