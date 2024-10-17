function addPan() { 
    let mouseStart = null;
    ctx.canvas.addEventListener("mousedown", e => {
        mouseStart = {x: e.clientX, y: e.clientY};
    });

    ["mouseup", "mouseleave"].forEach(name => ctx.canvas.addEventListener(name, _ => {
        mouseStart = null;
    }));

    ctx.canvas.addEventListener("mousemove", e => {
        if (!mouseStart)
            return;
        let dx = e.clientX - mouseStart.x;
        let dy = e.clientY - mouseStart.y;
        viewX -= dx / scale;
        viewY -= dy / scale;
        mouseStart = {x: e.clientX, y: e.clientY};
        draw();
    });
}

function setSpeedText() {
    let s = "paused";
    if (!isPaused) {
        if (log2speed < 5)
            s = String(1 << log2speed) + " gen/s";
        else
            s = "(2 ^ " + String(log2speed) + ") gen/s";
    }
    document.getElementById("speed-text").innerText = s;
}

let isPaused = true;
function onPausePlay() {
   if (!isPaused) {
        isPaused = true;
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(img/play-button.png)";
        setSpeedText();
   } else {
        isPaused = false;
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(img/pause-button.png)";
        setSpeedText();
   }
}

function onZoomOut() {
    document.getElementById("game-zoom-out-button").removeEventListener('click', onZoomOut);
    if (scale <= 10)
        scale /= 1.5;
    else
        scale -= 10;
    draw();
    lastTimeDrawn = Date.now();
    document.getElementById("game-zoom-out-button").addEventListener('click', onZoomOut);
}

function onZoomIn() {
    document.getElementById("game-zoom-in-button").removeEventListener('click', onZoomIn);
    if (scale <= 10)
        scale *= 1.5;
    else
        scale += 10;
    draw();
    lastTimeDrawn = Date.now();
    document.getElementById("game-zoom-in-button").addEventListener('click', onZoomIn);
}

function onSlowDown() {
    if (log2speed > 0)
        log2speed -= 1;
    if (log2speed == 0)
        document.getElementById("game-slow-down-button").disabled = true;
    setSpeedText();
}

function onSpeedUp() {
    log2speed += 1;
    document.getElementById("game-slow-down-button").disabled = false;
    setSpeedText();
}

function addGameMenuListeners() {
    document.getElementById("game-pause-play-button").addEventListener('click', onPausePlay);
    document.getElementById("game-zoom-out-button").addEventListener('click', onZoomOut);
    document.getElementById("game-zoom-in-button").addEventListener('click', onZoomIn);
    document.getElementById("game-slow-down-button").addEventListener('click', onSlowDown);
    document.getElementById("game-speed-up-button").addEventListener('click', onSpeedUp);
}

function fixCanvas() {
    const canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        draw();
    }
    window.addEventListener('resize', resizeCanvas, false);
    resizeCanvas();
}

function showGame() {
    document.getElementById("loading").hidden = true;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = false; 
    document.getElementById("game-menu").hidden = false;
}