fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/bits_and_bytes.ico");
        res.compile().unwrap();
    }
}