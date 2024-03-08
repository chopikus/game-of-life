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
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 5, 5), 
                {{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}});

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 1, 1), 
                {{1, 0}});
            
    EXPECT_V_EQ(life.get_alive_cells(0, 1, 1, 2, 2),
                {{2, 1}, {1, 2}, {2, 2}});

    EXPECT_V_EQ(life.get_alive_cells(0, 1000, 1000, 2000, 2000),
                {});
}

TEST(Hashlife, zoom_out) {
    std::vector<Cell> input;
    for (int x=0; x<=3; x++)
        for (int y=0; y<=3; y++)
            input.push_back({x, y});
    for (int x=6; x<=7; x++)
        for (int y=2; y<=3; y++)
            input.push_back({x, y});
    
    Hashlife life{input};
    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 7, 7),
                input);
    
    EXPECT_V_EQ(life.get_alive_cells(1, 0, 0, 7, 7),
                {{0, 0}, {0, 2}, {2, 0}, {2, 2}, {6, 2}});
    
    EXPECT_V_EQ(life.get_alive_cells(2, 0, 0, 7, 7),
                {{0, 0}, {4, 0}});
}

TEST(Hashlife, glider_1_step) {
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.successor(0);

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 100, 100),
                {{0, 1}, {2, 1}, {1, 2}, {2, 2}, {1, 3}});
}

TEST(Hashlife, glider_2_steps) {
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.successor(1);

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 100, 100),
                {{2, 1}, {0, 2}, {2, 2}, {1, 3}, {2, 3}});
}

TEST(Hashlife, glider_3_steps) {
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.successor(1);
    life.successor(0);

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 100, 100),
                {{1, 1}, {2, 2}, {3, 2}, {1, 3}, {2, 3}});
}

TEST(Hashlife, glider_4_steps) {
    std::vector<Cell> output;
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.successor(2);

    EXPECT_V_EQ(life.get_alive_cells(0, 0, 0, 100, 100), 
                {{2, 1}, {3, 2}, {1, 3}, {2, 3}, {3, 3}});
}

TEST(Hashlife, glider_1000_steps_time) {
    std::vector<Cell> output;
    Hashlife life{{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}}};
    life.successor(12);
    std::cout << "successor cache size " << life._successor_cache.size() << std::endl;
    std::cout << "join cache size " << life._join_cache.size() << std::endl;
    //assert(false);
}