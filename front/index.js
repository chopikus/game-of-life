import {showMenu} from "./menu.js";
const wasmWorker = new Worker('./wasmWorker.js', {type: 'module'});

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
            wasmWorker.postMessage({req: "aliveCells"});
            break;
        }
        
        case "aliveCellsOk": {
            const bar = new BigInt64Array(data.buf);
            console.log(bar);
            console.timeEnd("start");
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