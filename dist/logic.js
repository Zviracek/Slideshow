const controlsDiv = document.getElementById('controls');
const slideshowDiv = document.getElementById('slideshow');

// Fullscreen button logic
const fullscreenBtn = document.getElementById('fullscreen-btn');
fullscreenBtn.onclick = () => {
    if (document.fullscreenElement) {
        document.exitFullscreen();
    } else {
        document.documentElement.requestFullscreen();
    }
};


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
let currentSlideIndex = 0;
let currentElem = null; // currently visible element

const fadeDuration = 2000;    // CSS fade in/out duration
const displayDuration = 3000; // fully visible time

// Build slide sequence per category
function buildCategorySequence(cat) {
    const slides = [];

    // Header
    slides.push({ type: 'header', name: cat.name, logo: '../logo.png' });

    // Optional video
    if (cat.video) {
        slides.push({ type: 'video', src: '../' + cat.video });
    }

    // Images
    cat.images.forEach(img => slides.push({ type: 'image', src: '../' + img }));

    return slides;
}

function showNext() {
    console.log("show next");
    const activeCats = data.categories.filter(cat => selected[cat.name]);
    if (activeCats.length === 0) return;

    const cat = activeCats[currentCategoryIndex % activeCats.length];
    const slides = buildCategorySequence(cat);
    const slide = slides[currentSlideIndex % slides.length];

    console.log("Current slide:", slide);
    let nextElem;
    // Build element based on slide type
    if (slide.type === 'header') {
        console.log("show header");
        nextElem = document.createElement('h1');
        const logoImg = document.createElement('img');
        logoImg.src = slide.logo;
        logoImg.classList.add('logo');
        nextElem.appendChild(logoImg);

        const nameSpan = document.createElement('span');
        nameSpan.textContent = slide.name;
        nextElem.appendChild(nameSpan);
    } else if (slide.type === 'video') {
        console.log("show video");
        nextElem = document.createElement('video');
        nextElem.src = slide.src;
        nextElem.autoplay = true;
        nextElem.muted = true;
        nextElem.playsInline = true;
        nextElem.controls = false;
    } else if (slide.type === 'image') {
        console.log("show image");
        nextElem = document.createElement('img');
        nextElem.src = slide.src;
    }

    const startFade = () => {
        console.log("starting fade");
        const oldElem = currentElem;
        
        // Append element first
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

        currentElem = nextElem;

        // Advance indexes
        currentSlideIndex++;
        if (currentSlideIndex >= slides.length) {
            currentSlideIndex = 0;
            currentCategoryIndex++;
        }

        // Determine delay
        if (slide.type === 'video') {
            // Wait until video ends
            nextElem.onended = () => showNext();
            nextElem.play();
        } else {
            setTimeout(showNext, displayDuration);
        }
    };

    slideshowDiv.appendChild(nextElem);

    if (slide.type === 'image') {
        // Only start fade after image loaded
        if (nextElem.complete) {
            startFade();
        } else {
            nextElem.onload = startFade;
        }
    } else {
        // header or video start immediately
        startFade();
    }
}

// Start slideshow
showNext();