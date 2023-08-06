fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("manifest.xml");
        res.compile().unwrap();
    }
}
