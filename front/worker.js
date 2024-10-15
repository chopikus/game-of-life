import wasmInit, { Universe } from './pkg/game_of_life_rust.js';

let onMessageResponse = async () => {
    const rustWasm = await wasmInit("./pkg/game_of_life_rust_bg.wasm");
    let memory = rustWasm.memory;
    
    // sending response to main thread
    //const u = new Uint8Array(1024 * 1024 * 8);//.map((v, i) => 0);
    //console.log("created u");
    const universe = Universe.new();
    
    console.time("cells");
    for (let i = 0; i<1000; i++) {
        const cellsPtr = universe.read("hello world 123123123");
        const cells = new Uint8Array(memory.buffer, cellsPtr, 1048576);
        
        const cellsToPass = new Uint8Array(cells);
        postMessage(cellsToPass.buffer, [cellsToPass.buffer]);
    }
    console.timeEnd("cells");
};

onmessage = (event) => {
    onMessageResponse();
};
