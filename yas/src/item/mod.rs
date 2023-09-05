pub mod genshin_artifact;
pub mod starrail_relic;

pub enum Item {
    GenshinArtifact(genshin_artifact::GenshinArtifact),
    StarrailRelic(starrail_relic::StarrailRelic),
}
