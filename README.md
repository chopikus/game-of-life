## benchmark results
generating 1000 vectors, each has 1048576 values

time to generate vectors -- 1769.39ms
time with copying vectors between threads -- 2592ms

It takes ~0.8ms to copy 1MB between wasm memory on a worker thread to the main thread.
