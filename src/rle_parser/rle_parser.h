#pragma once
#include <string>
#include <vector>
#include "../units/units.h"
#include "../universe/universe.h"

class RleParser {
    public:
    RleParser() {};

    void add_chunk(const std::string& s);
    Universe result();

    private:
    void parse(const std::string& chunk);
    
    /* Following symbols can appear in file: #,b,o,$,!,[0-9]*/
    /* Since chunk can end in the middle of a line, we need to all the state.*/
    size_t cur_number{0};
    bool is_comment_line{false};
    bool had_exclamation{false};
    bool is_line_start{true};
    int64_t x{0}, y{0};

    std::vector<Cell> alive_cells;
};