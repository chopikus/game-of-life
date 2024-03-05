#include <emscripten/bind.h>
#include "rle_parser.h"
#include <regex>
#include <string>

void RleParser::add_chunk(const std::string& chunk) {
    parse(chunk);
}

void RleParser::parse(const std::string& chunk) {
    for (size_t i = 0; i < chunk.size(); i++) {
        char c = chunk[i];
        if (c == '!')
            had_exclamation = true;
        if (had_exclamation)
            break;
        
        if (c == '\n') {
            is_line_start = true;
            continue;
        }

        if (is_line_start) { 
            is_line_start = false;
            is_comment_line = (c == '#' || c == 'x');
        }
        if (is_comment_line)
            continue;
        
        if (c == '$') {
            size_t repeat = (cur_number == 0) ? 1 : cur_number;
            y += repeat;
            x = 0;
            cur_number = 0;
            continue;
        }
        if (c == 'b') {
            size_t repeat = (cur_number == 0) ? 1 : cur_number;
            x += repeat;
            cur_number = 0;
            continue;
        }
        if (c == 'o') {
            size_t repeat = (cur_number == 0) ? 1 : cur_number;
            for (size_t i = 0; i < repeat; i++) {
                alive_cells.push_back({.x = x, .y = y});
                x += 1;
            }
            cur_number = 0;
            continue;
        }
        if (c >= '0' && c <= '9') {
            cur_number *= 10;
            cur_number += c - '0';
            continue;
        }
    }
}

Universe RleParser::result() {
    return Universe{alive_cells};
}

EMSCRIPTEN_BINDINGS(RleParser) {
    emscripten::class_<RleParser>("RleParser")
        .constructor<>()
        .function("add_chunk", &RleParser::add_chunk)
        .function("result", &RleParser::result);
}