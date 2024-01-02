# game-of-life
My implementation of [Conway's Game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

This project relies on [WebAssembly](https://webassembly.org/) and [Emscripten](https://emscripten.org/).

Computation of a grid is done in C++; visualization is in written in JavaScript (`front` folder).

## Structure
* `src` -- C++ backend;
* `src/tests` -- C++ backend tests;
* `src/lib` -- libraries needed for C++ backend;
* `front/` -- JS frontend;
* `front/pkg` -- Emscripten output of C++ backend.

Frontend scripts are executed in the following order:
1. `front/wasm_loader.js` -- loads emscripten output on page loading;
2. `front/pkg/backend.js`;
3. `front/index.js`; (dynamically added from `front/wasm_loader.js`)

## Steps to build and run locally
0. Install [emsdk](https://github.com/emscripten-core/emsdk).
1. Clone a repository.
2. `mkdir build; cd build` -- create `build` folder and go to it.
3. `emcmake cmake ..`;
4. `make`;
5. `make test` -- tests C++ backend;
6. `cd ../front`.
7. Run local server from the current folder. Example: `basic-http-server .`

## Test
To test the backend, run `cargo test` from the project directory.

## Debug build demo
![demo](https://github.com/chopikus/game-of-life/assets/67230858/8dec5b9a-3b7a-43d0-a12c-5dfd28f4a0bd)

