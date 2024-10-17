import wasmInit, { Universe } from './pkg/game_of_life_rust.js';

let rustWasm, universe;

let onMessageResponse = async (data) => {
    switch (data.req) {
        case "initWasm": {
            rustWasm = await wasmInit("./pkg/game_of_life_rust_bg.wasm");
            postMessage({res: "initWasmOk"});
            break;
        }
        
        case "parseFile": {
            let s = data.s;
            if (universe)
                universe.free();
            universe = Universe.new(s);
            postMessage({res: "parseFileOk"});
            break;
        }

        case "tick": {
            universe.tick();
            postMessage({res: "tickOk"});
            break;
        }

        case "aliveCells": {
            universe.req_output();
            const outputPtr = universe.output();
            const outputLen = universe.output_len();
            const cells0 = new BigInt64Array(rustWasm.memory.buffer, outputPtr, outputLen);
            const cells = new BigInt64Array(cells0);

            postMessage({res: "aliveCellsOk", buf: cells.buffer}, [cells.buffer]);
            break;
        }

        default: {
            break;
        }
    }
};

onmessage = (event) => {
    onMessageResponse(event.data);
};