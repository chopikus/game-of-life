import init, { greet } from "../pkg/hello_wasm.js"

await init();
greet();
