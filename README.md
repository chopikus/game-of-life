# game-of-life

https://github.com/chopikus/game-of-life/assets/67230858/af7350b7-7e39-4b81-b7f8-af6c471d8eeb

## Usage
0. Download any pattern in .rle format. A few examples are [here](https://github.com/chopikus/game-of-life/tree/main/rle_examples).
1. Open [https://chopikus.github.io/game-of-life/](https://chopikus.github.io/game-of-life/), choose any .rle pattern.
2. Enjoy your pattern simulation!

## Steps to build and run locally
0. Install [emsdk](https://github.com/emscripten-core/emsdk).
1. Clone a repository.
2. `mkdir build; cd build` -- create `build` folder and go to it.
3. `emcmake cmake ..`;
4. `make` -- builds C++ backend;
5. `make test` -- tests C++ backend;
6. `cd ../front`.
7. Run local server from the current folder. Example: `basic-http-server .`

## Helpful projects
This implementation is largely inspired from [this](https://johnhw.github.io/hashlife/index.md.html) amazing explanation made by [johnhw](https://github.com/johnhw)!

[cpp-lru-cache](https://github.com/lamerman/cpp-lru-cache) is used in this implementation (Copyright (c) 2014, lamerman)
