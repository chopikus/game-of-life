let viewX = 0;
let viewY = 0;
let scale = 50;
let ctx;
let universe;
let lastTimeDrawn = Date.now();
let alive_cells;
let log2speed = 0;
let log2fps = 5;
let frameInterval = 1000/32;
let frame_number = 0;

function cellCoordsToScreen(x, y) {
    return {x: (Number(x) - viewX) * scale, 
            y: (Number(y) - viewY) * scale};
}

function makeTick() {
    frame_number += 1;
    frame_number %= (1 << log2fps);

    if (!isPaused)
    {
        if (log2speed > log2fps) {
            /* Both speed and log2fps are stored as logarithms.
               Suppose the speed is 64gen/s, and the fps rate is 32gen/s.
               Then the speed is stored as 6, log2fps is 5.
               Then we need to make 2 ticks, calling universe.tick(log2(2))=universe.tick(6-5).
               universe.tick also accepts log2 of tick amount*/

            /* TODO FIX */
            universe.tick(log2speed - log2fps);
        }
        else {
            /* Suppose the speed is 2gen/s, framerate is 32gen/s. */
            /* Then we need to tick 1gen forward every 16'th frame */
            if (frame_number % (1 << (log2fps-log2speed)) == 0) {
                /* TODO FIX */
                universe.tick(0);
            }
        }
    }
}

function gameCycle() {
    /* TODO: fix this game cycle implementation. */
    let interval = 1000/(1 << log2fps);
    let now = Date.now();
    let delta = now - lastTimeDrawn;
    if (delta > interval) {
        makeTick();
        
        let w = ctx.canvas.width / scale;
        let h = ctx.canvas.height / scale;

        /* TODO FIX */
        alive_cells = universe.get_alive_cells_val(scale, viewX, viewY, viewX + w, viewY + h);

        draw();
        lastTimeDrawn = Date.now() - delta;
        if (Date.now() - now > 1.2 * interval)
            log2fps = Math.max(log2fps - 1, 1);
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
    let drawScale = Math.max(2, scale);
    if (alive_cells) {
        for (var i = 0; i < alive_cells.length / 2; i++) {
            let cell_x = alive_cells[i*2];
            let cell_y = alive_cells[i*2+1];
            let {x, y} = cellCoordsToScreen(cell_x, cell_y);
            x = Math.floor(x);
            y = Math.floor(y);
            ctx.fillRect(x, y, drawScale, drawScale);
            ctx.closePath();  
        }
    }
}

function startGame(worker) {
    console.log("started game");
    /*
    addGameMenuListeners();
    fixCanvas();
    addPan();
    showGame();
    gameCycle();*/
}


export {startGame};
