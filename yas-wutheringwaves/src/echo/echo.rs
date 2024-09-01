use crate::echo::{WWEchoName, WWStat};

pub struct WWEcho {
    pub name: WWEchoName,
    pub main_stat1: WWStat,
    pub main_stat2: WWStat,
    pub sub_stats: Vec<WWStat>,
    pub level: usize,
    pub star: usize,
    pub lock: bool,
}
