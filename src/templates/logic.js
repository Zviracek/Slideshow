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
let currentElem = null; // currently visible element

const fadeDuration = 2000;    // CSS fade in/out duration
const displayDuration = 3000; // fully visible time

function showNext() {
    console.log("LOL");
    const activeCats = data.categories.filter(cat => selected[cat.name]);
    if (activeCats.length === 0) return;

    const cat = activeCats[currentCategoryIndex % activeCats.length];

    let nextElem;
    if (currentImageIndex === 0) {
        nextElem = document.createElement('h1');

        // create logo <img> (same for all categories)
        const logoImg = document.createElement('img');
        logoImg.src = '../logo.png';   // path to your single logo file
        logoImg.classList.add('logo'); // optional CSS class
        nextElem.appendChild(logoImg);

        // add category name text
        const nameSpan = document.createElement('span');
        nameSpan.textContent = cat.name;
        nextElem.appendChild(nameSpan);
    } else {
        console.log("showing picture");
        nextElem = document.createElement('img');
        nextElem.onerror = () => console.log("image failed to load:", nextElem.src);

        nextElem.src = '../' + cat.images[(currentImageIndex - 1) % cat.images.length];
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

        // Update indexes
        currentImageIndex++;
        if (currentImageIndex > cat.images.length) {
            currentImageIndex = 0;
            currentCategoryIndex++;
        }

        setTimeout(showNext, displayDuration);
    };

    slideshowDiv.appendChild(nextElem);

    if (nextElem.tagName === 'IMG') {
        if (nextElem.complete) {
            startFade();
        } else {
            nextElem.onload = startFade;
        }
    } else {
        startFade(); // header
    }

    //if (nextElem.tagName === 'IMG') {
        //if (nextElem.complete) {
            //startFade();
        //} else {
            //nextElem.onload = startFade;
        //}
    //} else {
        //startFade(); // header
    //}
}

// Start slideshow
showNext();