#pragma once

#include <emscripten/bind.h>
#include <map>
#include <set>
#include <string>
#include <vector>
#include "../lib/json.hpp"
#include "../units/units.h"
#include "../hashlife/hashlife.h"


class Universe {
    public:
    Universe(const std::vector<Cell>& alive_cells);

    /* scale pixels per cell */
    std::vector<Cell> get_alive_cells(double scale, double min_x, double min_y, double max_x, double max_y);

    emscripten::val get_alive_cells_val(double scale, double min_x, double min_y, double max_x, double max_y);

    void tick(int32_t speed);

    private:
    Hashlife life;
    std::vector<uint32_t> raw_bytes;
};