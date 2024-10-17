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
        // TODO
        isPaused = true;
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(img/play-button.png)";
        setSpeedText();
   } else {
        // TODO
        isPaused = false;
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(img/pause-button.png)";
        setSpeedText();
   }
}

function onZoomOut() {
    document.getElementById("game-zoom-out-button").removeEventListener('click', onZoomOut);
    // TODO
    if (scale <= 10)
        scale /= 1.5;
    else
        scale -= 10;
    //draw();
    lastTimeDrawn = Date.now();
    document.getElementById("game-zoom-out-button").addEventListener('click', onZoomOut);
}

function onZoomIn() {
    document.getElementById("game-zoom-in-button").removeEventListener('click', onZoomIn);
    // TODO
    if (scale <= 10)
        scale *= 1.5;
    else
        scale += 10;
    //draw();
    lastTimeDrawn = Date.now();
    document.getElementById("game-zoom-in-button").addEventListener('click', onZoomIn);
}

function onSlowDown() {
    // TODO
    if (log2speed > 0)
        log2speed -= 1;
    if (log2speed == 0)
        document.getElementById("game-slow-down-button").disabled = true;
    setSpeedText();
}

function onSpeedUp() {
    // TODO
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


function showGame() {
    document.getElementById("loading").hidden = true;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = false; 
    document.getElementById("game-menu").hidden = false;
}

function fixCanvas(canvasWorker) {
    function resizeCanvas() {
        canvasWorker.postMessage({req: "canvasResize", width: window.innerWidth, height: window.innerHeight});
    }
    window.addEventListener('resize', resizeCanvas, false);
    resizeCanvas();
}

function gameDOMInit(canvasWorker) {
    addGameMenuListeners();
    fixCanvas(canvasWorker);
    showGame();
}

export {gameDOMInit};