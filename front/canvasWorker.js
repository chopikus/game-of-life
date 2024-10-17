import {log2fps} from "./gen.js";

let viewX = 0;
let viewY = 0;
let scale = 50;
let ctx;
let aliveCells;

let lastTimeDrawn = Date.now();
let updated = false;

function cellCoordsToScreen(x, y) {
    return {x: (Number(x) - viewX) * scale, 
            y: (Number(y) - viewY) * scale};
}


function drawCycle() {
    let interval = 1000 / (1 << log2fps);
    let now = Date.now();
    let delta = now - lastTimeDrawn;
    if (delta > interval) {        
        draw();
        lastTimeDrawn = Date.now() - delta;
    }
    requestAnimationFrame(drawCycle);
}

function draw() {
    if (!updated) {
        return;
    }
    updated = false;

    console.timeLog("draw");
    
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
    if (aliveCells) {
        for (var i = 0; i < aliveCells.length / 2; i++) {
            let cell_x = aliveCells[i*2];
            let cell_y = aliveCells[i*2+1];
            let {x, y} = cellCoordsToScreen(cell_x, cell_y);
            x = Math.floor(x);
            y = Math.floor(y);
            ctx.fillRect(x, y, drawScale, drawScale);
            ctx.closePath();  
        }
    }
}

onmessage = (event) => {
    const data = event.data;
    switch (data.req) {
        case "canvasStart": {
            let canvas = data.canvas;
            ctx = canvas.getContext("2d");

            requestAnimationFrame(drawCycle);
            break;
        }

        case "canvasUpdate": {
            viewX = data.viewX;
            viewY = data.viewY;
            scale = data.scale;
            break;
        }

        case "canvasUpdateCells": {
            aliveCells = new BigInt64Array(data.buf);
            break;
        }

        case "canvasResize": {
            ctx.canvas.width = data.width;
            ctx.canvas.height = data.height;
            break;
        }

        case "canvasPan": {
            let dx = data.dx;
            let dy = data.dy;
            viewX -= dx / scale; 
            viewY -= dy / scale;
            break;
        }

        case "canvasZoomIn": {
            if (scale <= 10)
                scale *= 1.5;
            else
                scale += 10;
            break;
        }

        case "canvasZoomOut": {
            if (scale <= 10)
                scale /= 1.5;
            else
                scale -= 10;
            break;
        }

        default: {
            break;
        }
    }
    updated = true;
};


console.log("draw");