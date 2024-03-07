#include <fstream>
#include <gtest/gtest.h>
#include <sstream>
#include "../hashlife.h"

void PrintTo(const Cell& cell, std::ostream* os) {
    *os << "(x=" << cell.x << ",y=" << cell.y << ")";
}

void EXPECT_V_EQ(std::vector<Cell> v1, std::vector<Cell> v2) {
    sort(v1.begin(), v1.end());
    sort(v2.begin(), v2.end());
    EXPECT_EQ(v1, v2);
}

TEST(Hashlife, glider) {
    Hashlife life{0, 0, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};

    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 5, 5);
    EXPECT_V_EQ(output, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}});

    output.clear();
    life.append_alive_cells_root(output, 0, 0, 0, 1, 1);
    EXPECT_V_EQ(output, {{1, 0}});

    output.clear();
    life.append_alive_cells_root(output, 0, 1, 1, 2, 2);
    EXPECT_V_EQ(output, {{2, 1}, {1, 2}, {2, 2}});

    output.clear();
    life.append_alive_cells_root(output, 0, 1000, 1000, 2000, 2000);
    EXPECT_V_EQ(output, {});
}

TEST(Hashlife, zoom_out) {
    std::vector<Cell> input;
    for (int x=0; x<=3; x++)
        for (int y=0; y<=3; y++)
            input.push_back({x, y});
    for (int x=6; x<=7; x++)
        for (int y=2; y<=3; y++)
            input.push_back({x, y});
    
    Hashlife life{0, 0, input};
    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 7, 7);
    EXPECT_V_EQ(output, input);
    
    output.clear();
    life.append_alive_cells_root(output, 1, 0, 0, 7, 7);
    EXPECT_V_EQ(output, {{0, 0}, {0, 2}, {2, 0}, {2, 2}, {6, 2}});

    output.clear();
    life.append_alive_cells_root(output, 2, 0, 0, 7, 7);
    EXPECT_V_EQ(output, {{0, 0}, {4, 0}});
}

TEST(Hashlife, glider_1_step) {
    Hashlife life{0, 0, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.rootSuccessor(0);

    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 100, 100);
    EXPECT_V_EQ(output, {{0, 1}, {2, 1}, {1, 2}, {2, 2}, {1, 3}});
}

TEST(Hashlife, glider_2_steps) {
    Hashlife life{0, 0, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.rootSuccessor(1);

    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 100, 100);
    EXPECT_V_EQ(output, {{2, 1}, {0, 2}, {2, 2}, {1, 3}, {2, 3}});
}

TEST(Hashlife, glider_3_steps) {
    Hashlife life{0, 0, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.rootSuccessor(1);
    life.rootSuccessor(0);

    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 100, 100);
    EXPECT_V_EQ(output, {{1, 1}, {2, 2}, {3, 2}, {1, 3}, {2, 3}});
}

TEST(Hashlife, glider_4_steps) {
    std::vector<Cell> output;
    Hashlife life{0, 0, {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.rootSuccessor(2);

    life.append_alive_cells_root(output, 0, 0, 0, 100, 100);
    EXPECT_V_EQ(output, {{2, 1}, {3, 2}, {1, 3}, {2, 3}, {3, 3}});
}
