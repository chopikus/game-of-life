#pragma once
#include <string>
#include <vector>
#include "../units/units.h"
#include "../universe/universe.h"

class RleParser {
    public:
    RleParser() {};

    /* This function is meant to be called once. */
    void read(const std::string& chunk);
    Universe result();

    private:
    std::vector<Cell> alive_cells;
};