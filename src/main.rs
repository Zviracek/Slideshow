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
            data.push_str(&format!(" \"{}\",\n", img));
        }
        data.push_str(" ]},\n");
    }
    data.push_str("]};");

    // vytvoření výstupního adresáře
    let _ = fs::create_dir_all("dist/photos");

    // zkopírování fotek
    for (cat, imgs) in &categories {
        let _ = fs::create_dir_all(format!("dist/photos/{}", cat));
        for img in imgs {
            let src = format!("{}/{}", photos_dir, img);
            let dst = format!("dist/photos/{}", img);
            let _ = fs::copy(&src, &dst);
        }
    }

    // uložení JS dat
    fs::write("dist/slideshow.js", data).unwrap();

    // základní HTML
    let html = r#"<!DOCTYPE html>
<html lang="cs">
<head>
<meta charset="UTF-8">
<title>Slideshow</title>
<link rel="stylesheet" href="style.css">
</head>
<body>
<div id="controls"></div>
<div id="slideshow"></div>
<script src="slideshow.js"></script>
<script src="logic.js"></script>
</body>
</html>"#;

    fs::write("dist/index.html", html).unwrap();

    // základní CSS
    let css = r#"
        body { font-family: sans-serif; background: black; color: white; text-align: center; margin: 0; overflow: hidden; }
#slideshow {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

#slideshow img, #slideshow h1 {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  max-width: 100%;
  max-height: 100%;
  opacity: 0;
  transition: opacity 2s ease-in-out;
  object-fit: contain; /* ensures images keep aspect ratio */
}
#slideshow img.active, #slideshow h1.active {
  opacity: 1;
}


"#;

    fs::write("dist/style.css", css).unwrap();

    // jednoduchá logika v JS
    let js_logic = r#"const controlsDiv = document.getElementById('controls');
const slideshowDiv = document.getElementById('slideshow');


// Vytvoření checkboxů
const selected = {};
data.categories.forEach(cat => {
const label = document.createElement('label');
const cb = document.createElement('input');
cb.type = 'checkbox';
cb.checked = true;
selected[cat.name] = true;
cb.onchange = () => { selected[cat.name] = cb.checked; };
label.appendChild(cb);
label.appendChild(document.createTextNode(cat.name));
controlsDiv.appendChild(label);
});


let currentCategoryIndex = 0;
let currentImageIndex = 0;
let currentElem = null; // currently visible element

const fadeDuration = 2000;    // CSS fade in/out duration
const displayDuration = 3000; // fully visible time

function showNext() {
    const activeCats = data.categories.filter(cat => selected[cat.name]);
    if (activeCats.length === 0) return;

    const cat = activeCats[currentCategoryIndex % activeCats.length];

    let nextElem;
    if (currentImageIndex === 0) {
        nextElem = document.createElement('h1');
        nextElem.textContent = cat.name;
    } else {
        nextElem = document.createElement('img');
        nextElem.src = 'photos/' + cat.images[(currentImageIndex - 1) % cat.images.length];
    }

    const startFade = () => {
        const oldElem = currentElem; // save reference to old element
        slideshowDiv.appendChild(nextElem);

        // Trigger fade-in
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                nextElem.classList.add('active');
            });
        });

        // Fade out old element
        if (oldElem) {
            oldElem.classList.remove('active');
            setTimeout(() => {
                if (oldElem.parentNode) slideshowDiv.removeChild(oldElem);
            }, fadeDuration);
        }

        // Only now update currentElem
        currentElem = nextElem;

        // Update indexes
        currentImageIndex++;
        if (currentImageIndex > cat.images.length) {
            currentImageIndex = 0;
            currentCategoryIndex++;
        }

        // Schedule next slide
        setTimeout(showNext, displayDuration);
    };

    if (nextElem.tagName === 'IMG') {
        if (nextElem.complete) {
            startFade();
        } else {
            nextElem.onload = startFade;
        }
    } else {
        startFade(); // header
    }
}

// Start slideshow
showNext();


"#;

    fs::write("dist/logic.js", js_logic).unwrap();

    println!("Slideshow vygenerována ve složce dist/");
}
