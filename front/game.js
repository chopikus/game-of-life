const log2fps = 5;

let viewX = 0;
let viewY = 0;
let scale = 50;
let ctx;
let alive_cells;

let log2speed = 0;
let frame_number = 0;
let lastTimeDrawn = Date.now();

function cellCoordsToScreen(x, y, {viewX, viewY, scale}) {
    return {x: (Number(x) - viewX) * scale, 
            y: (Number(y) - viewY) * scale};
}

/* params object should have 3 elements: viewX, viewY, scale */
function draw(params) {
    ctx.fillStyle = "#000000";
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    drawGrid(params); 
    drawCells(params);
}

function drawGrid(params) {
    let {viewX, viewY, scale} = params;
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
            let start = cellCoordsToScreen(x, viewY, params);
            let end = cellCoordsToScreen(x, viewY + h, params);
            ctx.moveTo(start.x + 0.5, start.y + 0.5);
            ctx.lineTo(end.x + 0.5, end.y + 0.5);
            ctx.stroke();
        }
        for (let y = Math.floor(viewY); y <= Math.ceil(viewY + h); y += 1) {
            let start = cellCoordsToScreen(viewX, y, params);
            let end = cellCoordsToScreen(viewX + w, y, params);
            ctx.moveTo(start.x + 0.5, start.y + 0.5);
            ctx.lineTo(end.x + 0.5, end.y + 0.5);
            ctx.stroke();
        }
    }
}

function drawCells(params) {
    ctx.fillStyle = "#ffffff";
    let drawScale = Math.max(2, scale);
    if (alive_cells) {
        for (var i = 0; i < alive_cells.length / 2; i++) {
            let cell_x = alive_cells[i*2];
            let cell_y = alive_cells[i*2+1];
            let {x, y} = cellCoordsToScreen(cell_x, cell_y, params);
            x = Math.floor(x);
            y = Math.floor(y);
            ctx.fillRect(x, y, drawScale, drawScale);
            ctx.closePath();  
        }
    }
}

function drawCycle() {
    let interval = 1000 / (1 << log2fps);
    let now = Date.now();
    let delta = now - lastTimeDrawn;
    if (delta > interval) {        
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

window.onmessage = (event) => {
    const data = event.data;
    switch (data.req) {
        case "speed": {
            break;
        }

        case "aliveCells": {
            break;
        }

        case "scale": {
            break;
        }
    }
};

export {startGame};
