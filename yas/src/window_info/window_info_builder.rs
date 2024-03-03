use std::collections::HashSet;
use anyhow::{Result, anyhow};
use crate::{game_info::Resolution, common::positioning::{Size, Scalable}};

use super::{window_info_repository::WindowInfoRepository, window_info_prototypes::WindowInfoPrototypes};

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

    pub fn build_window_info(&self, prototypes: &WindowInfoPrototypes, resolution: Size) -> Result<WindowInfoRepository> {
        let resolution_family = Resolution::new(resolution);
        let proto = match prototypes.get_window_info(resolution_family) {
            Some(v) => v,
            None => {
                return Err(anyhow!("window info not found"));
            }
        };
        
        let factor = resolution.height / proto.current_resolution.height;
        // let result = proto.scale(factor);

        let mut result = WindowInfoRepository::new(resolution, resolution_family);
        for key in self.required_key.iter() {
            if !proto.data.contains_key(key) {
                return Err(anyhow!("window info {} is not present", key));
            }

            let value = proto.data.get(key).unwrap();
            result.data.insert(key.clone(), value.scale(factor));
        }

        Ok(result)
    }
}
