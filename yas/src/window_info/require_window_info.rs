use super::window_info_builder::WindowInfoBuilder;

pub trait RequireWindowInfo {
    fn require_window_info(window_info_builder: &mut WindowInfoBuilder);
}
