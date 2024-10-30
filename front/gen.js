const log2fps = 5;
const timeStep = 1000.0 / (1 << log2fps);

let lastTimeGenerated = 0.0;
let delta = 0.0;
let frame_number = 0;

let log2speed = 0;
function setlog2speed(x) {
    log2speed = x;
}

let isPaused = true;
function setIsPaused(x) {
    isPaused = x;
}

let bounds;
function setBounds(b) {
    bounds = b;
}

function genCycle(time, wasmWorker) {    
    const dt = time - lastTimeGenerated;
    delta += dt;
    lastTimeGenerated = time;

    while (delta > timeStep) {
        gen(wasmWorker);
        delta -= timeStep;
    }

    requestAnimationFrame((timeStamp) => genCycle(timeStamp, wasmWorker));
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
            wasmWorker.postMessage({req: "tick", speed: log2speed - log2fps, bounds: bounds});
        }
        else {
            /* Suppose the speed is 2gen/s, framerate is 32gen/s. */
            /* Then we need to tick 1gen forward every 16'th frame */
            if (frame_number % (1 << (log2fps-log2speed)) == 0) {
                wasmWorker.postMessage({req: "tick", speed: 0, bounds: bounds});
            }
        }
    }
}

export {genCycle, gen, log2fps, isPaused, setIsPaused, log2speed, setlog2speed, setBounds};