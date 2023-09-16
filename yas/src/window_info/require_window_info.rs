use super::window_info_builder::WindowInfoBuilder;

pub trait RequireWindowInfo {
    pub fn require_window_info(window_info_builder: &mut WindowInfoBuilder);
}
