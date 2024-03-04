use crate::common::positioning::Size;
use crate::window_info::WindowInfoRepository;

pub trait FromWindowInfoRepository: Sized {
    fn from_window_info_repository(window_size: Size, repo: &WindowInfoRepository) -> anyhow::Result<Self>;
}
