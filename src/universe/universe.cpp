#include <algorithm>
#include <cmath>
#include <emscripten/bind.h>
#include "../lib/json.hpp"
#include "universe.h"
#include <utility>

Universe::Universe(const std::vector<Cell>& alive_cells) : life(alive_cells) {};

std::vector<Cell> Universe::get_alive_cells(double scale, double min_x, double min_y, double max_x, double max_y) {
    int64_t i_min_x = floor(min_x);
    int64_t i_min_y = floor(min_y);
    int64_t i_max_x = floor(max_x);
    int64_t i_max_y = floor(max_y);
    uint8_t i_scale = 0;
    if (scale < 2)
    {
        i_scale = ceil(log2(2 / scale));
    }
    return life.get_alive_cells(i_scale, i_min_x, i_min_y, i_max_x, i_max_y);
}

void Universe::tick(int speed) {
    int j = 0;
    while (speed > 0) {
        if (speed % 2 == 1)
            life.successor(j);
        speed /= 2;
        j += 1;
    }
}

EMSCRIPTEN_BINDINGS(universe) {
    emscripten::class_<Universe>("Universe")
        .constructor<const std::vector<Cell>&>()
        .function("get_alive_cells", &Universe::get_alive_cells)
        .function("tick", &Universe::tick);

    emscripten::value_object<Cell>("Cell")
        .field("x", &Cell::x)
        .field("y", &Cell::y);

    emscripten::register_vector<Cell>("VectorCell");
}