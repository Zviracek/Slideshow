use std::fs;
use std::io::Write;
use std::path::Path;

use rand::seq::SliceRandom;
use rand::rng;

// Category structure
struct Category {
    name: String,
    images: Vec<String>,
    videos: Vec<String>,
    sponsors: Vec<String>,
}

fn main() {
    // Path to photos & sponsors
    let photos_dir = "photos";
    let sponsors_dir = format!("{}/sponsors", photos_dir);

    // Collect sponsor logos first
    let mut sponsors_pool = vec![];
    if let Ok(entries) = fs::read_dir(&sponsors_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext.to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" | "gif" => {
                        sponsors_pool.push(format!(
                            "../{}/{}",
                            sponsors_dir,
                            entry.file_name().to_string_lossy()
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    let mut categories: Vec<Category> = vec![];

    // Scan photos_dir for category folders (skip sponsors folder itself)
    if let Ok(entries) = fs::read_dir(photos_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let category_name = entry.file_name().to_string_lossy().to_string();
                if category_name == "sponsors" {
                    continue; // skip sponsor folder
                }

                let mut images = vec![];
                let mut videos = vec![];

                // read files inside each category folder
                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        let path = file.path();
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let fname = file.file_name().to_string_lossy().to_string();
                            let rel_path = format!("{}/{}", category_name, fname);

                            match ext.to_lowercase().as_str() {
                                "mp4" => videos.push(format!("../{}/{}", photos_dir, rel_path)),
                                "jpg" | "jpeg" | "png" | "gif" => images.push(format!("../{}/{}", photos_dir, rel_path)),
                                _ => {}
                            }
                        }
                    }
                }

                // pick two random sponsors (if available)
                let mut rng = rng();
                let mut picked_sponsors = sponsors_pool.clone();
                picked_sponsors.shuffle(&mut rng);
                let sponsors = picked_sponsors.into_iter().take(2).collect::<Vec<_>>();

                categories.push(Category {
                    name: category_name,
                    images,
                    videos,
                    sponsors,
                });
            }
        }
    }

    // JSON-like data
    let mut data = String::from("const data = { categories: [\n");
    for cat in &categories {
        data.push_str(&format!(" {{ name: \"{}\", ", cat.name));

        // videos array
        data.push_str("videos: [\n");
        for vid in &cat.videos {
            data.push_str(&format!(" \"{}\",\n", vid));
        }
        data.push_str("], ");

        // images array
        data.push_str("images: [\n");
        for img in &cat.images {
            data.push_str(&format!(" \"{}\",\n", img));
        }
        data.push_str("], ");

        // sponsors array
        data.push_str("sponsors: [\n");
        for s in &cat.sponsors {
            data.push_str(&format!(" \"{}\",\n", s));
        }
        data.push_str("]},\n");
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
