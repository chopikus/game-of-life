#include <fstream>
#include <gtest/gtest.h>
#include <sstream>
#include "../hashlife.h"

TEST(Hashlife, glider) {
    std::vector<Cell> alive_cells{{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}};
    Hashlife life{0, 0, alive_cells};
    std::vector<Cell> output;
    life.append_alive_cells_root(output, 0, 0, 0, 5, 5);
    sort(alive_cells.begin(), alive_cells.end());
    sort(output.begin(), output.end());
    EXPECT_EQ(output, alive_cells);
}