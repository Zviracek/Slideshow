const controlsDiv = document.getElementById('controls');
const slideshowDiv = document.getElementById('slideshow');

const LOGO_PATH = '../logo.png';

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

class category {
    constructor(cat_name, pictures_strs) {
        self.name = cat_name;
        self.pictures = pictures_strs;
    }

    run_category() {

    }

    show_header() {
        header = document.createElement('h1');
        const logoImg = document.createElement('img');
        logoImg.src = LOGO_PATH;
        logoImg.classList.add('logo');
        header.appendChild(logoImg);
    }
}