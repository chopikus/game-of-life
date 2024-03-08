#include <gtest/gtest.h>
#include "../universe/universe.h"
#include "../lib/json.hpp"

class Glider : public ::testing::Test {
    protected:
        Universe glider{Universe::example()};
};

TEST_F(Glider, start_cells) {
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

TEST_F(Glider, tick_cells) {
    glider.tick();
    nlohmann::json::array_t alive_cells = glider.alive_cells();
    auto contains = [&](const nlohmann::json& key) {
        return std::find(alive_cells.begin(), alive_cells.end(), key) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 5);
    EXPECT_TRUE(contains({{"x", 0}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 2}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 2}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 3}}));
}

TEST_F(Glider, two_ticks_cells) {
    glider.tick();
    glider.tick();
    nlohmann::json::array_t alive_cells = glider.alive_cells();
    auto contains = [&](const nlohmann::json& key) {
        return std::find(alive_cells.begin(), alive_cells.end(), key) != alive_cells.end();
    };
    EXPECT_EQ(alive_cells.size(), 5);
    EXPECT_TRUE(contains({{"x", 2}, {"y", 1}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 2}}));
    EXPECT_TRUE(contains({{"x", 2}, {"y", 3}}));
    EXPECT_TRUE(contains({{"x", 1}, {"y", 3}}));
    EXPECT_TRUE(contains({{"x", 0}, {"y", 2}}));
}