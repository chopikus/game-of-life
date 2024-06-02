#include <emscripten/bind.h>
#include "rle_parser.h"
#include <regex>
#include <string>

void RleParser::read(const std::string& chunk) {
    /* Following symbols can appear in file: #,b,o,$,!,[0-9]*/
    size_t cur_number{0};
    bool is_comment_line{false};
    bool had_exclamation{false};
    bool is_line_start{true};
    int64_t x{0}, y{0};

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
        .function("read", &RleParser::read)
        .function("result", &RleParser::result);
}
