use std::collections::HashSet;
use anyhow::Result;
use crate::{game_info::game_info::Resolution, common::positioning::{Size, Scalable}};

use super::{window_info::WindowInfo, window_info_prototypes::WindowInfoPrototypes};

pub struct WindowInfoBuilder<'a> {
    pub required_key: HashSet<String>,
}

impl WindowInfoBuilder {
    pub fn add_required_key(&mut self, key: &str) -> &mut self {
        self.required_key.insert(String::from(key));
        self
    }

    pub fn build(&self, prototypes: &WindowInfoPrototypes, resolution: Size) -> Result<WindowInfo> {
        let res = Resolution::new(resolution);
        let proto = prototypes.get_window_info(res)?;
        
        let factor = resolution.height / proto.current_resolution.height;
        let result = proto.scale(factor);

        anyhow::Ok(result)
    }
}
