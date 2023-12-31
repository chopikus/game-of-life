#pragma once

#include <map>
#include <set>
#include <string>
#include <vector>

struct Cell {
    int64_t x;
    int64_t y;
};

class Universe {
    public:
    Universe(const std::vector<Cell>& alive_cells);
    static Universe example();
    void tick();
    std::string alive_cells_str();

    inline bool is_cell_alive(int64_t x, int64_t y);
    inline bool tick_one_cell(bool is_alive, int alive_neighbours);
    std::map<int64_t, std::set<int64_t>> alive_cells_row;
};