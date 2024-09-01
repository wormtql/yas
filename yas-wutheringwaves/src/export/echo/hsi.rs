use std::ops::Deref;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::echo::{WWEcho, WWStat};

struct HsiStat<'a>(&'a WWStat);

impl<'a> Deref for HsiStat<'a> {
    type Target = WWStat;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Serialize for HsiStat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;

        root.serialize_entry("name", &self.name.to_string())?;
        root.serialize_entry("value", &self.value)?;

        root.end()
    }
}

struct HsiEcho<'a>(&'a WWEcho);

impl<'a> Deref for HsiEcho<'a> {
    type Target = WWEcho;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Serialize for HsiEcho<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(None)?;

        let mut hsi_sub_stats = Vec::new();
        for item in self.sub_stats.iter() {
            hsi_sub_stats.push(HsiStat(item));
        }

        root.serialize_entry("name", &self.name.to_string())?;
        root.serialize_entry("main_stat1", &HsiStat(&self.main_stat1))?;
        root.serialize_entry("main_stat2", &HsiStat(&self.main_stat2))?;
        root.serialize_entry("sub_stats", &hsi_sub_stats)?;
        root.serialize_entry("star", &self.star)?;
        root.serialize_entry("level", &self.level)?;
        // root.serialize_entry("lock", &self.lock)?;

        root.end()
    }
}

pub struct WWHsiFormat<'a> {
    pub echoes: &'a [HsiEcho<'a>],
    pub version: usize,
}

impl<'a> Serialize for WWHsiFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("echoes", self.echoes)?;
        map.serialize_entry("version", &self.version)?;
        map.end()
    }
}
