# rust-impl branch
The previous C++ implementation had several issues:
* Computation, drawing, mouse handling were done on a single thread
* Frontend code readability could be better
* Game cycle is off for small speeds
* Parsing the pattern file is too slow (>10s) for large files (16MB+)
* Memory issues on some Emscripten versions

In this branch I try to fix those problems.

Done so far:
* Refactored frontend to different files
* Fixed the game cycle
* File reading seems to be faster

In progress:
* moving calculations to Rust (basic algorithm is there but not Hashlife)

## benchmark results

generating 1000 vectors, each has 1048576 values

time to generate vectors -- 1769.39ms
time with copying vectors between threads -- 2592ms

It takes ~0.8ms to copy 1MB between wasm memory on a worker thread to the main thread.
