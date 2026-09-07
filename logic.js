const controlsDiv = document.getElementById('controls');
const slideshowDiv = document.getElementById('slideshow');

// URL of your deployed Worker's /list endpoint. Change this after `wrangler deploy`.
const WORKER_URL = 'https://slideshow-media-list.YOUR-SUBDOMAIN.workers.dev/list';

// Path to your logo. Simplest option: commit logo.png next to index.html in this repo.
const LOGO_PATH = 'logo.png';

// Fullscreen button logic
const fullscreenBtn = document.getElementById('fullscreen-btn');
fullscreenBtn.onclick = () => {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
  }
};

document.addEventListener('keydown', (e) => {
  if (e.key === 'f') { // press "f" to fullscreen
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }
});

let data = null;
const selected = {};

let currentCategoryIndex = 0;
let currentPhaseIndex = 0;
let currentElem = null;

const fadeDuration = 2000;
const displayDuration = 3000;

async function init() {
  try {
    const res = await fetch(WORKER_URL);
    if (!res.ok) throw new Error(`Worker returned ${res.status}`);
    data = await res.json();
  } catch (err) {
    slideshowDiv.textContent = 'Could not load media list.';
    console.error('Failed to load slideshow data:', err);
    return;
  }

  if (!data.categories || data.categories.length === 0) {
    slideshowDiv.textContent = 'No categories found in bucket.';
    return;
  }

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

  showNext();
}

function showNext() {
  const activeCats = data.categories.filter(cat => selected[cat.name]);
  if (activeCats.length === 0) return;

  const cat = activeCats[currentCategoryIndex % activeCats.length];

  // Build phases dynamically
  const phases = [];
  phases.push({ type: 'header' }); // header first
  cat.videos.forEach(v => phases.push({ type: 'video', src: v }));
  cat.images.forEach(img => phases.push({ type: 'image', src: img }));

  // Append sponsor logos at the end as individual image slides
  if (data.sponsors && data.sponsors.length > 0) {
    const shuffled = [...data.sponsors].sort(() => 0.5 - Math.random());
    const chosen = shuffled.slice(0, 2);
    chosen.forEach(s => phases.push({ type: 'image', src: s }));
  }

  const phase = phases[currentPhaseIndex % phases.length];
  currentPhaseIndex++;

  let nextElem;

  if (phase.type === 'header') {
    nextElem = document.createElement('h1');
    const logoImg = document.createElement('img');
    logoImg.src = LOGO_PATH;
    logoImg.classList.add('logo');
    nextElem.appendChild(logoImg);

    const nameSpan = document.createElement('span');
    nameSpan.textContent = cat.name;
    nextElem.appendChild(nameSpan);
  } else if (phase.type === 'video') {
    nextElem = document.createElement('video');
    nextElem.src = phase.src;
    nextElem.autoplay = true;
    nextElem.controls = false;
    nextElem.loop = false;
    nextElem.muted = true;
    nextElem.playsInline = true;
    nextElem.style.height = '100%'; // fit by height
    nextElem.style.width = 'auto';
    nextElem.style.maxWidth = 'none';
    nextElem.style.maxHeight = 'none';
  } else if (phase.type === 'image') {
    nextElem = document.createElement('img');
    nextElem.src = phase.src;
    nextElem.classList.add('slide'); // optional for CSS targeting
  }

  const startFade = () => {
    const oldElem = currentElem;
    slideshowDiv.appendChild(nextElem);

    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        nextElem.classList.add('active');
      });
    });

    if (oldElem) {
      oldElem.classList.remove('active');
      setTimeout(() => {
        if (oldElem.parentNode) slideshowDiv.removeChild(oldElem);
      }, fadeDuration);
    }

    currentElem = nextElem;

    if (phase.type === 'video') {
      nextElem.onended = () => {
        setTimeout(showNext, 500);
      };
    } else if (phase.type === 'image') {
      if (nextElem.complete) {
        setTimeout(showNext, displayDuration);
      } else {
        nextElem.onload = () => setTimeout(showNext, displayDuration);
      }
    } else {
      setTimeout(showNext, displayDuration);
    }
  };

  if (phase.type === 'image' && !nextElem.complete) {
    nextElem.onload = startFade;
  } else if (phase.type === 'video') {
    nextElem.onloadeddata = () => {
      startFade();
      nextElem.play();
    };
  } else {
    startFade();
  }

  // If we reached end of phases, go to next category
  if (currentPhaseIndex >= phases.length) {
    currentPhaseIndex = 0;
    currentCategoryIndex++;
  }
}

// Start slideshow once data is loaded
init();
