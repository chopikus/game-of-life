#include <string>

std::string gliderText = R""""(
# comment1
# comment2
x = 10, y = 10
# comment3
bo$2bo$3o!

# is the same as

# bob
# bbo
# ooo
# (1,0), (2,1), (0,2), (1,2), (2,2)    
)"""";

std::string twoSquaresText = R""""(
# comment1
# comment2
x = 1024, y = 2048
# comment3
2o2b$2o2b3$2b2o$2b2o!

# is the same as

# oobb
# oobb
# bbbb
# bbbb
# bboo
# bboo
# (0,0), (1,0), (0,1), (1,1), (2,4), (3,4), (2,5), (3,5)
)"""";