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

emscripten::val Universe::get_alive_cells_val(double scale, double min_x, double min_y, double max_x, double max_y) {
    std::vector<Cell> res{std::move(get_alive_cells(scale, min_x, min_y, max_x, max_y))};
    raw_bytes.clear();
    raw_bytes.reserve(res.size() * 2);
    for (size_t i=0; i<res.size(); i++) {
        raw_bytes.push_back(res[i].x);
        raw_bytes.push_back(res[i].y);
    }
    return emscripten::val(emscripten::typed_memory_view(raw_bytes.size(), raw_bytes.data()));
}

void Universe::tick(int speed) {
    life.successor(speed);
}

EMSCRIPTEN_BINDINGS(universe) {
    emscripten::class_<Universe>("Universe")
        .constructor<const std::vector<Cell>&>()
        .function("get_alive_cells_val", &Universe::get_alive_cells_val)
        .function("tick", &Universe::tick);

    emscripten::value_object<Cell>("Cell")
        .field("x", &Cell::x)
        .field("y", &Cell::y);

    emscripten::register_vector<Cell>("VectorCell");
}