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
                let mut video: Option<String> = None;

                // read files inside each category folder
                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        let path = file.path();
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let fname = file.file_name().to_string_lossy().to_string();
                            let rel_path = format!("{}/{}", category_name, fname);

                            // classify file type
                            match ext.to_lowercase().as_str() {
                                "mp4" => {
                                    // take first video found as the category video
                                    if video.is_none() {
                                        video = Some(rel_path);
                                    }
                                }
                                "jpg" | "jpeg" | "png" | "gif" => {
                                    images.push(rel_path);
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // store (category, images, optional video)
                // we can store as a struct or triple — here use triple
                categories.push((category_name, images, video));
            }
        }
    }

    // JSON-like data
    let mut data = String::from("const data = { categories: [\n");
    for (cat, imgs, video) in &categories {
        // opening brace
        data.push_str(&format!(" {{ name: \"{}\", ", cat));

        // optional video
        if let Some(video_path) = video {
            let full_video_path = format!("{}/{}", photos_dir, video_path);
            data.push_str(&format!("video: \"{}\", ", full_video_path));
        }

        // images array
        data.push_str("images: [\n");
        for img in imgs {
            let full_img_path = format!("{}/{}", photos_dir, img);
            data.push_str(&format!(" \"{}\",\n", full_img_path));
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
