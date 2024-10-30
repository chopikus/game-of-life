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
            universe.tick(data.speed);
            let bounds = data.bounds;
            universe.req_output(bounds.scale, 
                                BigInt(bounds.min_x), 
                                BigInt(bounds.min_y),
                                BigInt(bounds.max_x),
                                BigInt(bounds.max_y));
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