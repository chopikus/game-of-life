# rust implementation
Compared to the previous C++ implementation, now:
* Computation, drawing, mouse handling are done on different threads;
* Frontend code readability is better;
* Game cycle is better, before it was off for small speeds;
* Parsing the pattern file is faster (was >10s for large files (16MB+))
* Fixed memory issues

## steps to run
1. `./build.sh` or `wasm-pack build --out-dir front/pkg --target web --release`
2. run a local server from `front` folder, (e.g. `cd front; basic-http-server .`)

## benchmark
Before proceding with refactoring, I measured how much time does it take to transfer data from Wasm memory on a worker to JS on a main thread.

Hardware & OS: Ryzen 7 PRO 6850U processor, Radeon 680M GPU, 32GB RAM, Fedora 40.

* Time to generate vectors (postMessage commented) &mdash; **1769.39ms**
* Time including copying vectors between threads &mdash; **2592ms**

It takes ~0.8ms to copy 1MB between wasm memory on a worker thread to the main thread.


### benchmark code
```lib.rs
#[wasm_bindgen]
struct Universe {
    output: Vec<u8>,  
}

...

#[wasm_bindgen]
pub fn read(&mut self, x: String) -> *const u8 { 
    let mut my_vec= vec![];
    
    for i in 0..1048576 {
        let val = (i % 255) as u8;
        my_vec.push(val);
    }

    self.output = my_vec;

    return self.output.as_ptr();
}
```

worker.js:
```
import wasmInit, { Universe } from './pkg/game_of_life_rust.js';
let onMessageResponse = async () => {
    console.time("cells");
    const rustWasm = await wasmInit("./pkg/game_of_life_rust_bg.wasm");
    let memory = rustWasm.memory;
    
    // sending response to main thread
    const universe = Universe.new();
    
    console.time("cells");
    for (let i = 0; i<1000; i++) {
        const cellsPtr = universe.read("hello world 123123123");
        const cells = new Uint8Array(memory.buffer, cellsPtr, 1048576);
        
        const cellsToPass = new Uint8Array(cells);
        // postMessage(cellsToPass.buffer, [cellsToPass.buffer]); /* comment or uncomment this line */
    }
    console.timeEnd("cells");
};
onmessage = (event) => {
    onMessageResponse();
};
```


index.js:
```
const myWorker = new Worker('worker.js', {type: 'module'});

let i = 0;
console.time("cells2");
myWorker.onmessage = function (event) {
  const bar = new Uint8Array(event.data);
  console.log(bar.length);
  i++;
  if (i==1000) {
    console.timeEnd("cells2");
  }
};
```

