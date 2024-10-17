import { isPaused, log2speed, setIsPaused, setlog2speed } from "./gen.js";

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

function onPausePlay() {
   if (!isPaused) {
        setIsPaused(true);
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(/static/play-button.png)";
        setSpeedText();
   } else {
        setIsPaused(false);
        document.getElementById("game-pause-play-button").style.backgroundImage = "url(/static/pause-button.png)";
        setSpeedText();
   }
}

function onZoomOut(canvasWorker) {
    document.getElementById("game-zoom-out-button").removeEventListener('click', onZoomOut);

    canvasWorker.postMessage({req: "canvasZoomOut"});

    document.getElementById("game-zoom-out-button").addEventListener('click', onZoomOut);
}

function onZoomIn(canvasWorker) {
    document.getElementById("game-zoom-in-button").removeEventListener('click', onZoomIn);

    canvasWorker.postMessage({req: "canvasZoomIn"});

    document.getElementById("game-zoom-in-button").addEventListener('click', onZoomIn);
}

function onSlowDown() {
    if (log2speed > 0)
        setlog2speed(log2speed - 1);
    if (log2speed == 0)
        document.getElementById("game-slow-down-button").disabled = true;
    setSpeedText();
}

function onSpeedUp() {
    setlog2speed(log2speed + 1);
    document.getElementById("game-slow-down-button").disabled = false;
    setSpeedText();
}

function addGameMenuListeners(canvasWorker) {
    document.getElementById("game-pause-play-button").addEventListener('click', onPausePlay);
    document.getElementById("game-zoom-out-button").addEventListener('click', () => onZoomOut(canvasWorker));
    document.getElementById("game-zoom-in-button").addEventListener('click', () => onZoomIn(canvasWorker));
    document.getElementById("game-slow-down-button").addEventListener('click', onSlowDown);
    document.getElementById("game-speed-up-button").addEventListener('click', onSpeedUp);
}


function showGame() {
    document.getElementById("loading").hidden = true;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = false; 
    document.getElementById("game-menu").hidden = false;
}

function addPan(canvasWorker) { 
    let mouseStart = null;
    let canvas_div = document.getElementById("canvas-div");

    canvas_div.addEventListener("mousedown", e => {
        mouseStart = {x: e.clientX, y: e.clientY};
    });
    
    ["mouseup", "mouseleave"].forEach(name => canvas_div.addEventListener(name, _ => {
        mouseStart = null;
    }));

    canvas_div.addEventListener("mousemove", e => {
        if (!mouseStart)
            return;
        let dx = e.clientX - mouseStart.x;
        let dy = e.clientY - mouseStart.y;
        canvasWorker.postMessage({req: "canvasPan", dx, dy});
        
        mouseStart = {x: e.clientX, y: e.clientY};
    });
}

function fixCanvas(canvasWorker) {
    function resizeCanvas() {
        canvasWorker.postMessage({req: "canvasResize", width: window.innerWidth, height: window.innerHeight});
    }
    window.addEventListener('resize', resizeCanvas, false);
    resizeCanvas();
}

function gameDOMInit(canvasWorker) {
    addGameMenuListeners(canvasWorker);
    addPan(canvasWorker);
    fixCanvas(canvasWorker);
    showGame();
}

export {gameDOMInit};