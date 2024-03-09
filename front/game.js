let backend = globalThis.backend;

let viewX = 0;
let viewY = 0;
let scale = 50;
let ctx;
let universe;

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

function cellCoordsToScreen(x, y) {
    return {x: (Number(x) - viewX) * scale, 
            y: (Number(y) - viewY) * scale};
}

let lastTimeDrawn = Date.now();
let fps = 30;
let alive_cells;

function gameCycle() {
    let interval = 1000/fps;
    let now = Date.now();
    let delta = now - lastTimeDrawn;
    if (delta > 1000 / fps) {
        if (!isPaused)
            universe.tick(200);
        
        let w = ctx.canvas.width / scale;
        let h = ctx.canvas.height / scale;
        if (alive_cells)
            alive_cells.delete();
        alive_cells = universe.get_alive_cells(scale, viewX, viewY, viewX + w, viewY + h);
        draw();
        lastTimeDrawn = now - (delta % interval);
    }
    requestAnimationFrame(gameCycle);
}

function draw() {
    ctx.fillStyle = "#000000";
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    drawGrid(); 
    drawCells();
}

function drawGrid() {
    let w = ctx.canvas.width / scale;
    let h = ctx.canvas.height / scale;
    ctx.strokeStyle = "#ffffff";

    let drawing = false;
    if (scale >= 40)
    {
        ctx.lineWidth = Math.ceil(scale / 50);
        drawing = true;
    }
    if (drawing) {
        ctx.beginPath();
        for (let x = Math.floor(viewX); x <= Math.ceil(viewX + w); x += 1) {
            let start = cellCoordsToScreen(x, viewY);
            let end = cellCoordsToScreen(x, viewY + h);
            ctx.moveTo(start.x + 0.5, start.y + 0.5);
            ctx.lineTo(end.x + 0.5, end.y + 0.5);
            ctx.stroke();
        }
        for (let y = Math.floor(viewY); y <= Math.ceil(viewY + h); y += 1) {
            let start = cellCoordsToScreen(viewX, y);
            let end = cellCoordsToScreen(viewX + w, y);
            ctx.moveTo(start.x + 0.5, start.y + 0.5);
            ctx.lineTo(end.x + 0.5, end.y + 0.5);
            ctx.stroke();
        }
    }
}

function drawCells() {
    ctx.fillStyle = "#ffffff";
    let drawScale = Math.max(1, scale);
    if (alive_cells) {
        for (var i = 0; i < alive_cells.size(); i++) {
            let cell = alive_cells.get(i);
            let {x, y} = cellCoordsToScreen(cell.x, cell.y);
            x = Math.floor(x);
            y = Math.floor(y);
            ctx.fillRect(x, y, drawScale, drawScale);
            ctx.closePath();  
        }
    }
}

let isPaused = false;
function onPausePlay() {
   if (!isPaused) {
        isPaused = true;
        document.getElementById("game-pause-play-button").innerText = "▶";
   } else {
        isPaused = false;
        document.getElementById("game-pause-play-button").innerText = "||";
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

function addGameMenuListeners() {
    document.getElementById("game-pause-play-button").addEventListener('click', onPausePlay);
    document.getElementById("game-zoom-out-button").addEventListener('click', onZoomOut);
    document.getElementById("game-zoom-in-button").addEventListener('click', onZoomIn);
}

function fixCanvasResizing() {
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

function startGame(parsedUniverse) {
    universe = parsedUniverse;
    document.getElementById("loading").hidden = true;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = false; 
    document.getElementById("game-menu").hidden = false;
    addGameMenuListeners();
    fixCanvasResizing();
    addPan();
    requestAnimationFrame(gameCycle);
}

export {startGame};
