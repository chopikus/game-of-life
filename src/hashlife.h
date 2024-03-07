#pragma once
#include "lib/lrucache.hpp"
#include "universe.h"

#include <iostream>
#include <cstdint>
#include <memory>
#include <set>

struct Node {
    /* ____
      /    \
      |A  B|
      |C  D|
      \___/
    */
    using NodePtr = std::shared_ptr<Node>;
    uint8_t k; // level
    std::shared_ptr<Node> a,b,c,d;
    int64_t n; /* number of on nodes */
    uint64_t hash; // precomputed

    Node(uint8_t k, 
         NodePtr a, NodePtr b, NodePtr c, NodePtr d,
         int64_t n,
         uint64_t hash) : k(k), a(a), b(b), c(c), d(d), n(n), hash(hash) {};
};

class Hashlife {
    /* For a block of size 2^n, 
       computes result for block of 2^(n-1) in the center, 
       2^(n-2) steps forward. */
    public:
    using NodePtr = std::shared_ptr<Node>;

    const static NodePtr on;
    const static NodePtr off;

    Hashlife(int64_t min_x, int64_t min_y, const std::vector<Cell>& active_cells);

    /* returns a node from nodes a,b,c,d (cached)*/
    NodePtr join(const NodePtr& a, const NodePtr& b, const NodePtr& c, const NodePtr& d);

    /* returns a zero node of size k (cached)*/
    NodePtr get_zero(uint8_t k);

    /* 
        from |A B|
             |C D| of size 2^k
        
        returns |0 0 0 0|
                |0 A B 0|
                |0 C D 0|
                |0 0 0 0| of size 2^(k+1)
    */
    NodePtr centre(const NodePtr& m);

    /* Computes next step for node E*/
    /* | a b c |
       | d E f |
       | g h i | */
    NodePtr life_3x3(const NodePtr& a, const NodePtr& b,const NodePtr& c,
                    const NodePtr& d, const NodePtr& E, const NodePtr& f,
                    const NodePtr& g, const NodePtr& h, const NodePtr& i);

    /* Computes next step for 2x2 node in the center for m of size 4x4*/
    NodePtr life_4x4(const NodePtr& m);

    /* Computes min(2^j, 2^m.k) steps forward for a 2^(m.n-1)*2^(m.n-1) block in the center of m*/
    NodePtr successor(const NodePtr& m, uint8_t j = 0);
    
    void rootSuccessor(uint8_t j = 0);

    void append_alive_cells(const NodePtr& node, std::vector<Cell>& output,
                            uint8_t level,
                            int64_t x, int64_t y,
                            int64_t min_x, int64_t min_y,
                            int64_t max_x, int64_t max_y);

    void append_alive_cells_root(std::vector<Cell>& output,
                                  uint8_t level,
                                  int64_t min_x, int64_t min_y,
                                  int64_t max_x, int64_t max_y);

    NodePtr root{nullptr};

    /* 
      Fixing user coordinates
      1. Add -min_x to fix_x, -min_y to fix_y, 
         so that all cells have coordinates >= (0, 0)
      2. Upon construction of a map, we centre() it until it gets big enough.
       However, (0,0) points after centering is at a different point,
       so all coordinates that user provides need to be fixed. 
      3. When calling append_alive_cells, 
       we add fix_x to min_x, max_x, 
       and add fix_y to min_y, max_y.

       Then when returning result,
       we subtract (fix_x,fix_y) from every cell we return.
    */
    int64_t fix_x{0};
    int64_t fix_y{0};

    cache::lru_cache<uint8_t, NodePtr> zero_cache{300};
    cache::lru_cache<uint64_t, NodePtr> join_cache{1000000};
};