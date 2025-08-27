const controlsDiv = document.getElementById('controls');
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


