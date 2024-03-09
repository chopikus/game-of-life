#pragma once
#include "../units/units.h"
#include <cstdint>
#include <memory>

class NodeAllocator {
   public:
   NodeAllocator();

   inline NodePtr new_node(uint8_t k, 
                           NodePtr a, NodePtr b, NodePtr c, NodePtr d,
                           bool n,
                           uint64_t hash) {
      auto node = std::make_shared_for_overwrite<Node>();
      node->kn = k | (n << 7);
      node->a = a;
      node->b = b;
      node->c = c;
      node->d = d;
      node->hash = hash;
      return node;
   };
};