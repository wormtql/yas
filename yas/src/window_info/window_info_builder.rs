use std::collections::HashSet;
use anyhow::{Result, anyhow};
use crate::{game_info::Resolution, common::positioning::{Size, Scalable}};

use super::{window_info::WindowInfo, window_info_prototypes::WindowInfoPrototypes};

pub struct WindowInfoBuilder {
    pub required_key: HashSet<String>,
}

// constructors
impl WindowInfoBuilder {
    pub fn new() -> Self {
        WindowInfoBuilder {
            required_key: HashSet::new(),
        }
    }
}

impl WindowInfoBuilder {
    pub fn add_required_key(&mut self, key: &str) -> &mut Self {
        self.required_key.insert(String::from(key));
        self
    }

    pub fn build(&self, prototypes: &WindowInfoPrototypes, resolution: Size) -> Result<WindowInfo> {
        let res = Resolution::new(resolution);
        let proto = match prototypes.get_window_info(res) {
            Some(v) => v,
            None => {
                return Err(anyhow!("window info not found"));
            }
        };
        
        let factor = resolution.height / proto.current_resolution.height;
        let result = proto.scale(factor);

        anyhow::Ok(result)
    }
}
