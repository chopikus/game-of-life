#include <fstream>
#include <gtest/gtest.h>
#include <sstream>
#include "../rle_parser/rle_parser.h"
#include "mock/rle.h"

TEST(RleParser, glider) {
    RleParser parser{};
    parser.read(gliderText);

    Universe glider{parser.result()};

    std::vector<Cell> alive_cells = glider.get_alive_cells(0, 0, 0, 1000, 1000);
    auto contains = [&](int64_t x, int64_t y) -> bool {
        return std::find(alive_cells.begin(), alive_cells.end(), Cell{x, y}) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 5);
    EXPECT_TRUE(contains(1, 0));
    EXPECT_TRUE(contains(2, 1));
    EXPECT_TRUE(contains(0, 2));
    EXPECT_TRUE(contains(1, 2));
    EXPECT_TRUE(contains(2, 2));
}

TEST(RleParser, twoSquares) {
    RleParser parser{};
    parser.read(twoSquaresText);

    Universe twoSquares{parser.result()};

    std::vector<Cell> alive_cells = twoSquares.get_alive_cells(0, 0, 0, 1000, 1000);
    auto contains = [&](int64_t x, int64_t y) -> bool {
        return std::find(alive_cells.begin(), alive_cells.end(), Cell{x, y}) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 8);
    EXPECT_TRUE(contains(0, 0));
    EXPECT_TRUE(contains(1, 0));
    EXPECT_TRUE(contains(0, 1));
    EXPECT_TRUE(contains(1, 1));
    EXPECT_TRUE(contains(2, 4));
    EXPECT_TRUE(contains(3, 4));
    EXPECT_TRUE(contains(2, 5));
    EXPECT_TRUE(contains(3, 5));
}