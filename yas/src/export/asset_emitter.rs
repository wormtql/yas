use crate::export::ExportAssets;

pub trait AssetEmitter {
    fn emit(&self, asset_bundle: &mut ExportAssets);
}