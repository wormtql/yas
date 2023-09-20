use super::game_info::GameInfo;
use anyhow::Result;

pub struct GameInfoBuilder {
    pub local_window_names: Vec<String>,
    pub cloud_window_names: Vec<String>,
}

impl GameInfoBuilder {
    pub fn new() -> Self {
        GameInfoBuilder {
            local_window_names: Vec::new(),
            cloud_window_names: Vec::new(),
        }
    }

    pub fn add_local_window_name(&mut self, name: &str) -> &mut Self {
        self.local_window_names.push(String::from(name));
        self
    }

    pub fn add_cloud_window_name(&mut self, name: &str) -> &mut Self {
        self.cloud_window_names.push(String::from(name));
        self
    }

    pub fn build(&self) -> Result<GameInfo> {
        #[cfg(windows)]
        {
            let mut window_names = Vec::new();
            for name in self.local_window_names.iter() {
                window_names.push(name.as_str());
            }
            for name in self.cloud_window_names.iter() {
                window_names.push(name.as_str());
            }
            crate::game_info::os::get_game_info(&window_names)
            // crate::game_info::os::get_game_info(&["原神", "Genshin Impact", "云·原神"])
        }
        
        // todo other platforms
    }
}
