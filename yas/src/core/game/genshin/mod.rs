use super::*;
mod artifact;
mod const_info;
mod scanner;
pub use artifact::*;
pub use const_info::get_window_info;

#[derive(Clone, Debug)]
pub struct GenshinScanInfo<T = ScanInfoType> {
    pub shared: SharedScanInfo<T>,

    pub sub_stat_pos: [RectBound<T>; 4],
}

pub type GenshinWindowInfo = GenshinScanInfo<WindowInfoType>;

impl ConvertToScanInfo<GenshinScanInfo> for GenshinWindowInfo {
    fn to_scan_info(&self, size: Size<f64>) -> GenshinScanInfo {
        let radio = self.shared.get_radio(size);

        GenshinScanInfo {
            shared: self.shared.to_scan_info(size),

            sub_stat_pos: [
                self.sub_stat_pos[0].scale_to_scan(radio),
                self.sub_stat_pos[1].scale_to_scan(radio),
                self.sub_stat_pos[2].scale_to_scan(radio),
                self.sub_stat_pos[3].scale_to_scan(radio),
            ],
        }
    }
}
