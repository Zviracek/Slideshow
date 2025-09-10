use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let photos_dir = "photos";
    let mut categories = vec![];

    // Projde složky v adresáři `photos`
    if let Ok(entries) = fs::read_dir(photos_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let category_name = entry.file_name().to_string_lossy().to_string();
                let mut images = vec![];

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        let path = file.path();
                        if let Some(ext) = path.extension() {
                            if ["jpg", "jpeg", "png", "gif"].contains(&ext.to_str().unwrap_or("")) {
                                images.push(format!(
                                    "{}/{}",
                                    category_name,
                                    file.file_name().to_string_lossy()
                                ));
                            }
                        }
                    }
                }

                categories.push((category_name, images));
            }
        }
    }

    // JSON-like data
    let mut data = String::from("const data = { categories: [\n");
    for (cat, imgs) in &categories {
        data.push_str(&format!(" {{ name: \"{}\", images: [\n", cat));
        for img in imgs {
            // directly use original path
            let path = format!("{}/{}", photos_dir, img);
            data.push_str(&format!(" \"{}\",\n", path));
        }
        data.push_str(" ]},\n");
    }
    data.push_str("]};");

    fs::remove_dir_all("dist").unwrap();
    fs::create_dir("dist").unwrap();

    // uložení JS dat
    fs::write("dist/slideshow.js", data).unwrap();

    // základní HTML
    const MAIN_HTML: &str = include_str!("templates/main.html");
    fs::write("dist/index.html", MAIN_HTML).unwrap();

    // základní CSS
    const MAIN_CSS: &str = include_str!("templates/main.css");
    fs::write("dist/style.css", MAIN_CSS).unwrap();

    // jednoduchá logika v JS
    const LOGIC_JS: &str = include_str!("templates/logic.js");
    fs::write("dist/logic.js", LOGIC_JS).unwrap();

    println!("Slideshow vygenerována ve složce dist/");
}
