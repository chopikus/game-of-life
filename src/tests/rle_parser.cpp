#include <fstream>
#include <gtest/gtest.h>
#include <sstream>
#include "../rle_parser.h"
#include "mock/rle.h"

TEST(RleParser, glider) {
    RleParser parser{};
    parser.add_chunk(gliderText);

    Universe glider{parser.result()};

    nlohmann::json::array_t alive_cells = glider.alive_cells();
    auto contains = [&](const nlohmann::json& key) {
        return std::find(alive_cells.begin(), alive_cells.end(), key) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 5);
    EXPECT_TRUE(contains({{"x", 1}, {"y", 0}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 0}, {"y", 2}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 2}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 2}}));
}

TEST(RleParser, twoSquares) {
    RleParser parser{};
    parser.add_chunk(twoSquaresText);

    Universe twoSquares{parser.result()};

    nlohmann::json alive_cells = twoSquares.alive_cells();
    auto contains = [&](const nlohmann::json& key) {
        return std::find(alive_cells.begin(), alive_cells.end(), key) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 8);
    EXPECT_TRUE(contains({{"x", 0}, {"y", 0}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 0}}));
    EXPECT_TRUE(contains({{"x", 0}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 4}}));
    EXPECT_TRUE(contains({{"x", 3}, {"y", 4}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 5}}));
    EXPECT_TRUE(contains({{"x", 3}, {"y", 5}}));
}