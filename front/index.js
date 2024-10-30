import {showMenu} from "./menu.js";
import {gameDOMInit} from "./game-dom.js";
import {genCycle, isPaused, setBounds} from "./gen.js";

const wasmWorker = new Worker('./wasmWorker.js', {type: 'module'});
const canvasWorker = new Worker('./canvasWorker.js', {type: 'module'});

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
            // 1. Pass canvas to canvas worker
            const offscreen = document.getElementById("canvas").transferControlToOffscreen();
            canvasWorker.postMessage({req: "canvasStart", canvas: offscreen}, [offscreen]);

            // 2. Initialize game controls
            gameDOMInit(canvasWorker);

            requestAnimationFrame((timeStamp) => genCycle(timeStamp, wasmWorker));
            break;
        }
        
        case "aliveCellsOk": {
            if (!isPaused) {
                canvasWorker.postMessage({req: "canvasUpdateCells", buf: data.buf}, [data.buf]);
            }
            break;
        }

        case "bounds": {
            setBounds(data.bounds);
            break;
        }

        default: {
            break;
        }
    }
}

canvasWorker.addEventListener("message", (event) => {response(event.data)});

wasmWorker.addEventListener("message", (event) => {response(event.data)});
wasmWorker.postMessage({req: "initWasm"});

export {response}