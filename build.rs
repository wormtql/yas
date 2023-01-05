#[cfg(windows)]
use winres::WindowsResource;

#[cfg(windows)]
fn main() {
    let mut res = WindowsResource::new();
    res.set_manifest_file("manifest.xml");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
