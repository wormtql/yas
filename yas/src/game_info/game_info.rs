use crate::game_info::{ResolutionFamily, UI};
use crate::positioning::Rect;

#[derive(Clone, Debug)]
pub struct GameInfo {
    pub window: Rect<i32>,
    pub resolution_family: ResolutionFamily,
    pub is_cloud: bool,
    pub ui: UI,
}
