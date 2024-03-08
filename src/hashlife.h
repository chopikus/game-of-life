#pragma once
#include "lib/lrucache.hpp"
#include "units.h"

#include <iostream>
#include <cstdint>
#include <map>
#include <memory>
#include <set>
#include <typeindex>
#include <vector>
#include <utility>

class Hashlife {
   /* For a block of size 2^n, 
      computes result for block of 2^(n-1) in the center, 
      2^(n-2) steps forward. */
   public:
   const static NodePtr on;
   const static NodePtr off;

   Hashlife(const std::vector<Cell>& active_cells);
   void _construct_root(int64_t min_x, int64_t min_y, const std::vector<Cell>& active_cells);

   /* returns a node from nodes a,b,c,d (cached)*/
   NodePtr _join(const NodePtr& a, const NodePtr& b, const NodePtr& c, const NodePtr& d);

   /* returns a zero node of size k (cached)*/
   NodePtr _get_zero(uint8_t k);

   /* 
      from |A B|
            |C D| of size 2^k
      
      returns  |0 0 0 0|
               |0 A B 0|
               |0 C D 0|
               |0 0 0 0| of size 2^(k+1)
   */
   NodePtr _centre(const NodePtr& m);

   /* Computes next step for node E*/
   /* | a b c |
      | d E f |
      | g h i | */
   NodePtr _life_3x3(const NodePtr& a, const NodePtr& b,const NodePtr& c,
                  const NodePtr& d, const NodePtr& E, const NodePtr& f,
                  const NodePtr& g, const NodePtr& h, const NodePtr& i);

   /* Computes next step for 2x2 node in the center for m of size 4x4*/
   NodePtr _life_4x4(const NodePtr& m);

   /* Computes min(2^j, 2^m.k) steps forward for a 2^(m.n-1)*2^(m.n-1) block in the center of m*/    
   void successor(uint8_t j = 0);
   NodePtr _successor(const NodePtr& m, uint8_t j = 0);

   void _append_alive_cells(const NodePtr& node, std::vector<Cell>& output,
                           uint8_t level,
                           int64_t x, int64_t y,
                           int64_t min_x, int64_t min_y,
                           int64_t max_x, int64_t max_y);

   std::vector<Cell> get_alive_cells(uint8_t level,
                                    int64_t min_x, int64_t min_y,
                                    int64_t max_x, int64_t max_y);

   NodePtr _root{nullptr};

   /* 
   Fixing user coordinates
   1. Add -min_x to fix_x, -min_y to fix_y, 
      so that all cells have coordinates >= (0, 0)
   2. Upon construction of a map, we centre() it until it gets big enough.
      However, (0,0) points after centering is at a different point,
      so all coordinates that user provides need to be fixed. 
   3. When calling get_alive_cells, 
      we add fix_x to min_x, max_x, 
      and add fix_y to min_y, max_y.

      Then when returning result,
      we subtract (fix_x,fix_y) from every cell we return.
   */
   int64_t _fix_x{0};
   int64_t _fix_y{0};

   cache::lru_cache<uint8_t, NodePtr> _zero_cache{100};
   cache::lru_cache<FourNodePtr, NodePtr, FourNodePtrHash> _join_cache{33554432}; // 2^25

   cache::lru_cache<NodePtrAndStep, NodePtr, NodePtrAndStepHash> _successor_cache{33554432}; // 2^25

   cache::lru_cache<NodePtr, NodePtr, NodePtrHash> _life_4x4_cache{100};
};