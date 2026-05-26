use winres;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon(&format!("{}/assets/icons/icon.ico", env!("CARGO_MANIFEST_DIR")));
        res.compile().unwrap();
    }
}