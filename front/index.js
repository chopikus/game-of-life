import { Universe } from "game-of-life";

let viewX = 0;
let viewY = 0;
let scale = 10;
let ctx;
const universe = Universe.example();

function main() {
    const canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');

    window.addEventListener('resize', resizeCanvas, false);

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        draw();
    }

    resizeCanvas();
    addListeners(ctx);
    requestAnimationFrame(cycle);
}


let mouseStart = null;

function addListeners(ctx) { 
    ctx.canvas.addEventListener("mousedown", e => {
        mouseStart = {x: e.clientX, y: e.clientY};
    });

    ["mouseup", "mouseleave"].forEach(name => ctx.canvas.addEventListener(name, _ => {
        mouseStart = null;
    }));

    ctx.canvas.addEventListener("wheel", e => {
        let old_w = ctx.canvas.width / scale;
        let old_h = ctx.canvas.height / scale;
        scale *= Math.pow(2, e.deltaY / 3000);
        let new_w = ctx.canvas.width / scale;
        let new_h = ctx.canvas.height / scale;
        viewX += (old_w - new_w) / 2;
        viewY += (old_h - new_h) / 2;
        draw();
    });

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
    return {x: (x - viewX) * scale, 
            y: (y - viewY) * scale};
}

let then = Date.now();
let fps = 5;
let alive_cells;

function cycle() {
    let interval = 1000/fps;
    let now = Date.now();
    let delta = now - then;
    if (delta > 1000 / fps) {
        universe.tick();
        alive_cells = universe.alive_cells_str();
        draw();
        then = now - (delta % interval);
    }
    requestAnimationFrame(cycle);
}

function draw() {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    drawGrid(); 
    drawCells();
}

function drawGrid() {
    let w = ctx.canvas.width / scale;
    let h = ctx.canvas.height / scale;
    ctx.strokeStyle = "#000000";
    let drawing = true;
    if (scale >= 40)
        ctx.lineWidth = Math.ceil(scale / 50);
    else
        drawing = false;
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
    if (alive_cells) {
        let parsed_response = JSON.parse(alive_cells);
        parsed_response.forEach((cell) => {
            let {x, y} = cellCoordsToScreen(cell[0], cell[1]);
            ctx.fillRect(x, y, scale, scale);
            ctx.closePath();    
        });
    }
}

main();
