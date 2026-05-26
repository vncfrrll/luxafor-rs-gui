use winres;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let icon_path = format!("{}/assets/icons/icon.ico", manifest_dir);

        println!("cargo:warning=build.rs running on Windows");
        println!("cargo:warning=Looking for icon at: {}", icon_path);

        if std::path::Path::new(&icon_path).exists() {
            println!("cargo:warning=Icon file found!");
        } else {
            println!("cargo:warning=Icon file NOT found!");
        }

        let mut res = winres::WindowsResource::new();
        res.set_icon(&icon_path);
        res.compile().unwrap();
    }
}