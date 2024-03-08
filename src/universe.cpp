#include <algorithm>
//#include <emscripten/bind.h>
#include "lib/json.hpp"
#include "universe.h"
#include <utility>

#include <iostream>

Universe::Universe(const std::vector<Cell>& cells) {
    this->alive_cells_row = {};
    for (const auto& [x, y] : cells) {
        auto& set = this->alive_cells_row[y];
        set.insert(x);
    }
}

Universe Universe::example() {
    return Universe({{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}});
}

bool Universe::is_cell_alive(int64_t x, int64_t y) {
    auto it = alive_cells_row.find(y);
    if (it == alive_cells_row.end()) {
        return false;
    }
    return it->second.contains(x);
}

bool Universe::tick_one_cell(bool is_alive, int alive_neighbours) {
    return ((!is_alive && alive_neighbours == 3)
            ||
            (is_alive && alive_neighbours >= 2 && alive_neighbours <= 3));
}

void Universe::tick() {
    std::set<int64_t> rows_to_compute;
    int64_t min_column = 0;
    int64_t max_column = 0;
    for (const auto& [row, columns] : alive_cells_row) {
        rows_to_compute.insert(row - 1);
        rows_to_compute.insert(row);
        rows_to_compute.insert(row + 1);
        if (columns.empty())
            continue;
        min_column = std::min(min_column, *(columns.begin()));
        max_column = std::max(max_column, *(columns.rbegin()));
    }

    std::vector<std::pair<Cell, bool> > changes;
    for (int64_t y : rows_to_compute) {
        for (int64_t x = min_column - 1; x <= max_column + 1; x++) {
            bool was_alive = is_cell_alive(x, y);
            int alive_neighbours = 0;
            for (int dx = -1; dx <= 1; dx++) {
                for (int dy = -1; dy <= 1; dy++) {
                    if (dx == 0 && dy == 0) {
                        continue;
                    }
                    alive_neighbours += is_cell_alive(x+dx, y+dy);
                }
            }
            bool is_alive = tick_one_cell(was_alive, alive_neighbours);
            if (!was_alive && is_alive) {
                changes.push_back({{x, y}, true});
            } else if (was_alive && !is_alive) {
                changes.push_back({{x, y}, false});
            }
        }
    }
    for (auto [cell, alive] : changes) {
        if (alive)
            alive_cells_row[cell.y].insert(cell.x);
        else
            alive_cells_row[cell.y].erase(cell.x);
    }
}

nlohmann::json Universe::alive_cells() {
    auto result = nlohmann::json::array();
    for (const auto& [y, columns]: alive_cells_row) {
        for (const auto& x : columns) {
            result.push_back({{"x", x}, {"y", y}});
        }
    }
    return result;
}

std::string Universe::alive_cells_str() {
    return alive_cells().dump();
}

/*EMSCRIPTEN_BINDINGS(universe) {
    emscripten::class_<Universe>("Universe")
        .constructor<const std::vector<Cell>&>()
        .class_function("example", &Universe::example)
        .function("tick", &Universe::tick)
        .function("alive_cells_str", &Universe::alive_cells_str);
}*/