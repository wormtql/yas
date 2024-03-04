use crate::positioning::Size;
use crate::window_info::WindowInfoRepository;

pub trait FromWindowInfoRepository: Sized {
    fn from_window_info_repository(window_size: Size<usize>, repo: &WindowInfoRepository) -> anyhow::Result<Self>;
}
