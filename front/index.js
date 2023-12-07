function main() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    window.addEventListener('resize', resizeCanvas, false);

    function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        drawGrid(ctx);
        fillCells(ctx);
    }

    resizeCanvas(); 
}


const cell_size = 50;
const offset_x = 20;
const offset_y = 20;
const line_thickness = 1;

function drawGrid(ctx) {
    const w = ctx.canvas.width;
    const h = ctx.canvas.height;
    ctx.strokeStyle = "#000000";
    ctx.lineWidth = line_thickness;
    const cellX = Math.ceil(w / cell_size);
    const cellY = Math.ceil(h / cell_size);
    ctx.beginPath();
    for (let x = 0; x < cellX; x += 1) {
        ctx.moveTo(grid_start + x * cell_size, 0);
        ctx.lineTo(grid_start + x * cell_size, h);
        ctx.stroke();
    }
    for (let y = 0; y < cellY; y += 1) {
        ctx.moveTo(0, grid_start + y * cell_size);
        ctx.lineTo(w, grid_start + y * cell_size);
        ctx.stroke();
    }
    ctx.closePath();
}

function fillCells(ctx) {
    ctx.fillStyle = "yellow";
    cells_to_fill = [[0, 0], [5, 5], [10, 10]];
    for (const cell of cells_to_fill) {
        let x = cell[0];
        let y = cell[1];
        ctx.fillRect(grid_start + (x-1) * cell_size + line_thickness / 2,
                     grid_start + (y-1) * cell_size + line_thickness / 2,
                     cell_size - line_thickness, 
                     cell_size - line_thickness);
    }
}

main();
