import {showMenu} from "./menu.js";
import {gameInit} from "./game-dom.js";

const wasmWorker = new Worker('./wasmWorker.js', {type: 'module'});
const gameWorker = new Worker('./game.js', {type: 'module'});

const log2fps = 5;
let log2speed = 0;

let viewX = 0;
let viewY = 0;
let scale = 50;

function response(data) {
    switch (data.res) {
        case "initWasmOk": {
            showMenu();   
            break; 
        }

        case "readFileOk": {
            wasmWorker.postMessage({req: "parseFile", s: data.s});
            break;
        }

        case "parseFileOk": {
            gameInit();
            break;
        }
        
        case "aliveCellsOk": {
            const bar = new BigInt64Array(data.buf);
            console.log(bar);

            //gameWorker.postMessage({req: "update", aliveCells: ...});
            break;
        }

        default: {
            break;
        }
    }
}

wasmWorker.addEventListener("message", (event) => {response(event.data)});
wasmWorker.postMessage({req: "initWasm"});

export {response};

function genCycle() {
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
            wasmWorker.postMessage({req: "aliveCells", speed: log2speed - log2fps});
        }
        else {
            /* Suppose the speed is 2gen/s, framerate is 32gen/s. */
            /* Then we need to tick 1gen forward every 16'th frame */
            if (frame_number % (1 << (log2fps-log2speed)) == 0) {
                wasmWorker.postMessage({req: "aliveCells", speed: 0});
            }
        }
    }
}