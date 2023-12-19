let viewX = 0;
let viewY = 0;
let scale = 50;

function main() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    window.addEventListener('resize', resizeCanvas, false);

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        addListeners(ctx);
        draw(ctx);
    }

    resizeCanvas();
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
        scale *= Math.pow(2, e.deltaY / 1000);
        let new_w = ctx.canvas.width / scale;
        let new_h = ctx.canvas.height / scale;
        viewX += (old_w - new_w) / 2;
        viewY += (old_h - new_h) / 2;
        draw(ctx);
    });

    ctx.canvas.addEventListener("mousemove", e => {
        if (!mouseStart)
            return;
        dx = e.clientX - mouseStart.x;
        dy = e.clientY - mouseStart.y;
        viewX -= dx / scale;
        viewY -= dy / scale;
        draw(ctx);
        mouseStart = {x: e.clientX, y: e.clientY};
    });
}

function cellCoordsToScreen(x, y) {
    return {x: (x - viewX) * scale, 
            y: (y - viewY) * scale};
}

function draw(ctx) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    drawGrid(ctx); 
    drawCells(ctx);
}

function drawGrid(ctx) {
    let w = ctx.canvas.width / scale;
    let h = ctx.canvas.height / scale;
    ctx.strokeStyle = "#000000";
    drawing = true;
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

function drawCells(ctx) {
    let {x, y} = cellCoordsToScreen(0, 0);
    ctx.fillRect(x, y, scale, scale);
    ctx.closePath();
}

main();
