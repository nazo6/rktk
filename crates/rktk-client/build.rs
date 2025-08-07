fn main() {
    if !std::path::Path::new("assets/tailwind.css").exists() {
        std::fs::write("assets/tailwind.css", "").unwrap();
    }
}
