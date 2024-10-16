import wasmInit, { Universe } from './pkg/game_of_life_rust.js';

let rustWasm, universe;

let onMessageResponse = async (data) => {
    switch (data.req) {
        case "init": {
            rustWasm = await wasmInit("./pkg/game_of_life_rust_bg.wasm");
            postMessage({res: "initOk"});
            break;
        }
        
        case "file": {
            let s = data.s;
            if (universe)
                universe.free();
            universe = Universe.new(s);
            postMessage({res: "fileOk"});
            break;
        }

        case "tick": {
            universe.tick();
            postMessage({res: "tickOk"});
            break;
        }

        case "cell": {
            universe.req_output();
            const outputPtr = universe.output();
            const outputLen = universe.output_len();
            const cells0 = new BigInt64Array(rustWasm.memory.buffer, outputPtr, outputLen);
            const cells = new BigInt64Array(cells0);

            postMessage({res: "cellOk", buf: cells.buffer}, [cells.buffer]);
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
