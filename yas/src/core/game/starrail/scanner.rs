use crate::core::scanner_core::*;
use crate::core::*;
use std::sync::Arc;

pub struct StarRailScanner(pub ScannerCore);

impl ItemScanner for StarRailScanner {
    fn scan_item_image(
        model_inference: &ModelInferenceFunc,
        info: &Arc<ScanInfo>,
        item: ItemImage,
        cnt: usize,
    ) -> anyhow::Result<ScanResult> {
        let image = &item.image;

        let str_title = model_inference(&info.title_pos, "title", image, cnt)?;
        let str_main_stat_name =
            model_inference(&info.main_stat_name_pos, "main_stat_name", image, cnt)?;
        let str_main_stat_value =
            model_inference(&info.main_stat_value_pos, "main_stat_value", image, cnt)?;

        let starrail_info = &info.inner_starrail();
        let sub_stat_pos = [
            &starrail_info.sub_stat_name_pos,
            &starrail_info.sub_stat_value_pos,
        ];

        let str_sub_stat_1_name =
            model_inference(&sub_stat_pos[0][0], "sub_stat_1_name", image, cnt)?;
        let str_sub_stat_1_value =
            model_inference(&sub_stat_pos[1][0], "sub_stat_1_value", image, cnt)?;
        let str_sub_stat_2_name =
            model_inference(&sub_stat_pos[0][1], "sub_stat_2_name", image, cnt)?;
        let str_sub_stat_2_value =
            model_inference(&sub_stat_pos[1][1], "sub_stat_2_value", image, cnt)?;
        let str_sub_stat_3_name =
            model_inference(&sub_stat_pos[0][2], "sub_stat_3_name", image, cnt)?;
        let str_sub_stat_3_value =
            model_inference(&sub_stat_pos[1][2], "sub_stat_3_value", image, cnt)?;
        let str_sub_stat_4_name =
            model_inference(&sub_stat_pos[0][3], "sub_stat_4_name", image, cnt)?;
        let str_sub_stat_4_value =
            model_inference(&sub_stat_pos[1][3], "sub_stat_4_value", image, cnt)?;

        let str_level = model_inference(&info.level_pos, "level", image, cnt)?;
        let str_equip = model_inference(&info.item_equip_pos, "equip", image, cnt)?;

        Ok(ScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat: [
                str_sub_stat_1_name + "+" + &str_sub_stat_1_value,
                str_sub_stat_2_name + "+" + &str_sub_stat_2_value,
                str_sub_stat_3_name + "+" + &str_sub_stat_3_value,
                str_sub_stat_4_name + "+" + &str_sub_stat_4_value,
            ],
            level: parse_level(&str_level),
            equip: str_equip,
            star: item.star,
        })
    }
}
