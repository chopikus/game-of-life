#pragma once
#include "../units/units.h"
#include <cstdint>
#include <memory>

class NodeAllocator {
   public:
   NodeAllocator();

   /* Allocating all space possible for nodes in one go. */
   std::unique_ptr<Node[]> _nodes{nullptr};
   size_t _allocated_nodes{0};
   const size_t MAX_NODES = 15000000;

   inline Node* new_node(uint8_t k, 
                         NodePtr a, NodePtr b, NodePtr c, NodePtr d,
                         int64_t n,
                         uint64_t hash) {
      if (_allocated_nodes >= MAX_NODES) {
         throw std::runtime_error("All nodes' memory is used!");
      }
      Node* node = &(_nodes[_allocated_nodes]);
      _allocated_nodes += 1;
      node->k = k;
      node->a = a;
      node->b = b;
      node->c = c;
      node->d = d;
      node->n = n;
      node->hash = hash;
      return node;
   };
};