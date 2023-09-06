use std::sync::Arc;
use std::time::SystemTime;

use crate::core::scanner_core::*;
use crate::core::*;
use anyhow::Result;

pub struct GenshinScanner(pub ScannerCore);

impl ItemScanner for GenshinScanner {
    fn scan_item_image(
        model_inference: &ModelInferenceFunc,
        info: &Arc<ScanInfo>,
        item: ItemImage,
        cnt: usize,
    ) -> Result<ScanResult> {
        let image = &item.image;
        let _now = SystemTime::now();

        let str_title = model_inference(&info.title_pos, "title", image, cnt)?;
        let str_main_stat_name =
            model_inference(&info.main_stat_name_pos, "main_stat_name", image, cnt)?;
        let str_main_stat_value =
            model_inference(&info.main_stat_value_pos, "main_stat_value", image, cnt)?;

        let genshin_info = &info.inner_genshin();

        let str_sub_stat_1 =
            model_inference(&genshin_info.sub_stat_pos[0], "sub_stat_1", image, cnt)?;
        let str_sub_stat_2 =
            model_inference(&genshin_info.sub_stat_pos[1], "sub_stat_2", image, cnt)?;
        let str_sub_stat_3 =
            model_inference(&genshin_info.sub_stat_pos[2], "sub_stat_3", image, cnt)?;
        let str_sub_stat_4 =
            model_inference(&genshin_info.sub_stat_pos[3], "sub_stat_4", image, cnt)?;

        let str_level = model_inference(&info.level_pos, "level", image, cnt)?;
        let str_equip = model_inference(&info.item_equip_pos, "equip", image, cnt)?;

        Ok(ScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat: [
                str_sub_stat_1,
                str_sub_stat_2,
                str_sub_stat_3,
                str_sub_stat_4,
            ],
            level: parse_level(&str_level),
            equip: str_equip,
            star: item.star,
        })
    }
}
