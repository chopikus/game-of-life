const log2fps = 5;

let log2speed = 0;
let frame_number = 0;
let lastTimeGenerated = Date.now();
let isPaused = false;

function genCycle(wasmWorker) {    
    let interval = 1000 / (1 << log2fps);
    let now = Date.now();
    let delta = now - lastTimeGenerated;
    if (delta > interval) {        
        gen(wasmWorker);
        lastTimeGenerated = Date.now();
    }
    requestAnimationFrame(() => genCycle(wasmWorker));
}

function gen(wasmWorker) {
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
            wasmWorker.postMessage({req: "tick", speed: log2speed - log2fps});
        }
        else {
            /* Suppose the speed is 2gen/s, framerate is 32gen/s. */
            /* Then we need to tick 1gen forward every 16'th frame */
            if (frame_number % (1 << (log2fps-log2speed)) == 0) {
                console.log("tick");
                wasmWorker.postMessage({req: "tick", speed: 0});
            }
        }
    }
}

export {genCycle, gen, log2fps};